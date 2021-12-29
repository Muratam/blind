use super::*;
use collections::BitSet64;
use std::sync::mpsc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
// TODO: タッチ入力
// NOTE: Keyboard, GamePad are not implemented
//     -> Dropping Support for Android, ios

pub enum MouseState {
  IsDown,
  IsLeftClicked,
  IsRightClicked,
  IsDoubleClicked,
}
#[derive(Clone, Copy)]
enum MouseEvent {
  Move,
  Down,
  Up,
  Click,
  DoubleClick,
  ContextMenu,
}
struct MouseEventInfo {
  x: i32,
  y: i32,
  event: MouseEvent,
}
struct WheelEventInfo {
  delta_x: i32,
  delta_y: i32,
}
use once_cell::sync::OnceCell;
static INSTANCE: OnceCell<RwLock<EventHolderImpl>> = OnceCell::new();
unsafe impl Send for EventHolderImpl {}
unsafe impl Sync for EventHolderImpl {}
pub struct EventHolderImpl {
  mouse_x: Option<i32>,
  mouse_y: Option<i32>,
  mouse_pre_x: Option<i32>,
  mouse_pre_y: Option<i32>,
  mouse_state: BitSet64,
  mouse_rx: mpsc::Receiver<MouseEventInfo>,
  wheel_delta_x: i32,
  wheel_delta_y: i32,
  wheel_rx: mpsc::Receiver<WheelEventInfo>,
}
impl EventHolderImpl {
  pub fn read_global() -> RwLockReadGuard<'static, Self> {
    INSTANCE
      .get()
      .expect("event holder is not initialized")
      .read()
      .unwrap()
  }
  pub fn write_global() -> RwLockWriteGuard<'static, Self> {
    INSTANCE
      .get()
      .expect("event holder is not initialized")
      .write()
      .unwrap()
  }
  pub fn initialize_global(elem: &web_sys::HtmlElement) {
    INSTANCE.set(RwLock::new(Self::new(elem))).ok();
  }
  pub fn new(elem: &web_sys::HtmlElement) -> Self {
    let (mouse_tx, mouse_rx) = mpsc::channel::<MouseEventInfo>();
    let (wheel_tx, wheel_rx) = mpsc::channel::<WheelEventInfo>();
    let mut result = Self {
      mouse_x: None,
      mouse_y: None,
      mouse_pre_x: None,
      mouse_pre_y: None,
      mouse_state: BitSet64::new(),
      mouse_rx,
      wheel_delta_x: 0,
      wheel_delta_y: 0,
      wheel_rx,
    };
    result.setup_mouse_events(elem, mouse_tx);
    result.setup_wheel_events(elem, wheel_tx);
    result.setup_prevent_defaults(elem);
    result
  }
  fn setup_mouse_events(&mut self, elem: &web_sys::HtmlElement, tx: mpsc::Sender<MouseEventInfo>) {
    let tx = Arc::new(tx);
    let setup_callback = |event_type: MouseEvent, event_name: &str, prevent_default: bool| {
      let tx = tx.clone();
      let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        tx.send(MouseEventInfo {
          x: event.offset_x(),
          y: event.offset_y(),
          event: event_type,
        })
        .ok();
        if prevent_default {
          event.prevent_default();
        }
      }) as Box<dyn FnMut(_)>);
      elem
        .add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())
        .ok();
      closure.forget();
    };
    setup_callback(MouseEvent::Move, "mousemove", false);
    setup_callback(MouseEvent::Up, "mouseup", false);
    setup_callback(MouseEvent::Down, "mousedown", false);
    setup_callback(MouseEvent::Click, "click", false);
    setup_callback(MouseEvent::DoubleClick, "dblclick", false);
    setup_callback(MouseEvent::ContextMenu, "contextmenu", true);
  }
  fn setup_wheel_events(&mut self, elem: &web_sys::HtmlElement, tx: mpsc::Sender<WheelEventInfo>) {
    let tx = Arc::new(tx);
    let setup_callback = |event_name: &str, prevent_default: bool| {
      let tx = tx.clone();
      let closure = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
        tx.send(WheelEventInfo {
          delta_x: event.delta_x() as i32,
          delta_y: event.delta_y() as i32,
        })
        .ok();
        if prevent_default {
          event.prevent_default();
        }
      }) as Box<dyn FnMut(_)>);
      elem
        .add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())
        .ok();
      closure.forget();
    };
    setup_callback("wheel", true);
  }
  fn setup_prevent_defaults(&mut self, elem: &web_sys::HtmlElement) {
    let setup_callback = |event_name: &str| {
      let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
        event.prevent_default();
      }) as Box<dyn FnMut(_)>);
      elem
        .add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())
        .ok();
      closure.forget();
    };
    let setup_callback_keyboard = |event_name: &str| {
      let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        // allow reload
        if event.key() == "r" || event.key() == "F12" {
          return;
        }
        event.prevent_default();
      }) as Box<dyn FnMut(_)>);
      js::html::body()
        .add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())
        .ok();
      closure.forget();
    };
    // setup_callback("touchstart");
    // setup_callback("touchcancel");
    // setup_callback("touchforcechange");
    // setup_callback("touchmove");
    // setup_callback("touchend");
    // setup_callback("focus");
    // setup_callback("blur");

    // disable pinch-in zoom
    setup_callback("gesturestart");
    setup_callback("gesturechage");
    setup_callback("gestureend");
    // disable zoom
    setup_callback_keyboard("keydown");
    setup_callback_keyboard("keypress");
    setup_callback_keyboard("keydown");
  }
  pub fn mouse_x(&self) -> i32 {
    self.mouse_x.unwrap_or(0)
  }
  pub fn mouse_y(&self) -> i32 {
    self.mouse_y.unwrap_or(0)
  }
  pub fn mouse_dx(&self) -> i32 {
    self.mouse_x() - self.mouse_pre_x.unwrap_or(self.mouse_x())
  }
  pub fn mouse_dy(&self) -> i32 {
    self.mouse_y() - self.mouse_pre_y.unwrap_or(self.mouse_y())
  }
  pub fn mouse_state(&self, state: MouseState) -> bool {
    self.mouse_state.get(state as usize)
  }
  fn set_mouse_state(&mut self, state: MouseState, value: bool) {
    self.mouse_state.set(state as usize, value);
  }
  pub fn wheel_dx(&self) -> i32 {
    self.wheel_delta_x
  }
  pub fn wheel_dy(&self) -> i32 {
    self.wheel_delta_y
  }
}
impl Updatable for EventHolderImpl {
  fn update(&mut self) {
    // mouse state
    let is_mouse_down = self.mouse_state(MouseState::IsDown);
    self.mouse_state.set_all_false();
    self.set_mouse_state(MouseState::IsDown, is_mouse_down);
    self.mouse_pre_x = self.mouse_x;
    self.mouse_pre_y = self.mouse_y;
    // NOTE: めっちゃはやいとDownがだめかも？反応しないことが多ければwhile を if に
    while let Ok(info) = self.mouse_rx.try_recv() {
      self.mouse_x = Some(info.x);
      self.mouse_y = Some(info.y);
      match info.event {
        MouseEvent::Move => {}
        MouseEvent::Down => self.set_mouse_state(MouseState::IsDown, true),
        MouseEvent::Up => self.set_mouse_state(MouseState::IsDown, false),
        MouseEvent::Click => self.set_mouse_state(MouseState::IsLeftClicked, true),
        MouseEvent::ContextMenu => self.set_mouse_state(MouseState::IsRightClicked, true),
        MouseEvent::DoubleClick => self.set_mouse_state(MouseState::IsDoubleClicked, true),
      }
    }
    // wheel state
    self.wheel_delta_x = 0;
    self.wheel_delta_y = 0;
    while let Ok(info) = self.wheel_rx.try_recv() {
      self.wheel_delta_x += info.delta_x;
      self.wheel_delta_y += info.delta_y;
    }
  }
}
