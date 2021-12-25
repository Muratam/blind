use crate::html;
use crate::html::Canvas;

fn setup_global_style(parent: &web_sys::HtmlElement) {
  let css = r###" * {
    padding: 0px;
    border: 0px;
    margin: 0px;
    overscroll-behavior-y: none;
  }"###;
  // overscroll-behavior-x: none;
  // touch-action: none;
  let _ = html::append_css(parent, css);
}

fn setup_layer(elem: &web_sys::HtmlElement, z_index: i64) {
  let style = elem.style();
  style.set_property("position", "absolute").ok();
  style.set_property("width", "100%").ok();
  style.set_property("height", "100%").ok();
  style.set_property("z-index", &z_index.to_string()).ok();
}

pub struct Layers {
  // (下) 3D -> 2D -> HTML (上)
  main_3d_layer: web_sys::HtmlCanvasElement,
  main_2d_layer: web_sys::HtmlCanvasElement,
  // overlay3d_layer: web_sys::HtmlCanvasElement,
  // overlay2d_layer: web_sys::HtmlCanvasElement,
  html_layer: web_sys::HtmlDivElement,
  width: i32,
  height: i32,
}

impl Layers {
  pub fn new() -> Self {
    let root_element = html::append_div(&html::body());
    setup_global_style(&root_element);
    let main_3d_layer = html::append_canvas(&root_element);
    setup_layer(&main_3d_layer, 0);
    let main_2d_layer = html::append_canvas(&root_element);
    setup_layer(&main_2d_layer, 1);
    let html_layer = html::append_div(&root_element);
    setup_layer(&html_layer, 2);
    html_layer.style().set_property("overflow", "scroll").ok();
    let mut result = Self {
      main_2d_layer,
      main_3d_layer,
      html_layer,
      width: 0,
      height: 0,
    };
    result.adjust_screen_size();
    result
  }

  pub fn main_2d_context(&self) -> web_sys::CanvasRenderingContext2d {
    self.main_2d_layer.canvas_2d_context()
  }
  pub fn main_3d_context(&self) -> web_sys::WebGl2RenderingContext {
    self.main_3d_layer.webgl2_context()
  }
  pub fn html_layer(&self) -> &web_sys::HtmlDivElement {
    &self.html_layer
  }
  pub fn adjust_screen_size(&mut self) {
    let mut updated = false;
    if let Some(width) = html::window().inner_width().unwrap().as_f64() {
      let width = width as i32;
      if self.width != width {
        self.width = width;
        updated = true;
      }
    }
    if let Some(height) = html::window().inner_height().unwrap().as_f64() {
      let height = height as i32;
      if self.height != height {
        self.height = height;
        updated = true;
      }
    }
    if !updated {
      return;
    }
    for c in vec![&self.main_2d_layer, &self.main_3d_layer] {
      c.set_attribute("width", &self.width.to_string()).ok();
      c.set_attribute("height", &self.height.to_string()).ok();
    }
  }
  pub fn width(&self) -> i32 {
    self.width
  }
  pub fn height(&self) -> i32 {
    self.height
  }
}
