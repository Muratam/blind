use super::*;
fn setup_global_style(parent: &web_sys::HtmlElement) {
  let css = r###" * {
    padding: 0px;
    border: 0px;
    margin: 0px;
    box-sizing: border-box;
    touch-action: none;
    overscroll-behavior-x: none;
    overscroll-behavior-y: none;
    overflow: hidden;
    word-break: break-all;
    font-family: BlinkMacSystemFont, -apple-system, "Segoe UI", "Roboto", "Oxygen", "Ubuntu", "Cantarell", "Fira Sans", "Droid Sans", "Helvetica Neue", "Helvetica", "Arial", sans-serif;
  }"###;
  let _ = js::html::append_css(parent, css);
}

fn setup_layer(elem: &web_sys::HtmlElement, z_index: i64) {
  let style = elem.style();
  style.set_property("position", "absolute").ok();
  style.set_property("width", "100%").ok();
  style.set_property("height", "100%").ok();
  style.set_property("z-index", &z_index.to_string()).ok();
}

pub struct Layers {
  // (下) 3D -> HTML-> 3D-overlay  (上)
  main_3d_layer: web_sys::HtmlCanvasElement,
  overlay_3d_layer: web_sys::HtmlCanvasElement,
  html_layer: Arc<web_sys::HtmlDivElement>,
  width: i32,
  height: i32,
}

impl Layers {
  pub fn new() -> Self {
    let root_element = js::html::append_div(&js::html::body());
    setup_global_style(&root_element);
    let main_3d_layer = js::html::append_canvas(&root_element);
    setup_layer(&main_3d_layer, 0);
    let html_layer = js::html::append_div(&root_element);
    setup_layer(&html_layer, 1);
    let overlay_3d_layer = js::html::append_canvas(&root_element);
    overlay_3d_layer // TODO: prglに逃がす？(画像を一枚貼り付けるだけなのでcanvasは不要かも？)
      .style()
      .set_property("pointer-events", "none")
      .ok();
    setup_layer(&overlay_3d_layer, i32::MAX as i64);
    let html_layer = Arc::new(html_layer);
    let mut result = Self {
      main_3d_layer,
      html_layer,
      overlay_3d_layer,
      width: 0,
      height: 0,
    };
    result.adjust_screen_size();
    result
  }

  pub fn main_3d_context(&self) -> web_sys::WebGl2RenderingContext {
    crate::js::html::canvas::get_webgl2_context(&self.main_3d_layer)
  }
  pub fn html_layer(&self) -> &Arc<web_sys::HtmlDivElement> {
    &self.html_layer
  }
  pub fn adjust_screen_size(&mut self) {
    let mut updated = false;
    if let Some(width) = js::html::window().inner_width().unwrap().as_f64() {
      let width = width as i32;
      if self.width != width {
        self.width = width;
        updated = true;
      }
    }
    if let Some(height) = js::html::window().inner_height().unwrap().as_f64() {
      let height = height as i32;
      if self.height != height {
        self.height = height;
        updated = true;
      }
    }
    if !updated {
      return;
    }
    self.main_3d_layer.set_width(self.width as u32);
    self.main_3d_layer.set_height(self.height as u32);
    self.overlay_3d_layer.set_width(self.width as u32);
    self.overlay_3d_layer.set_height(self.height as u32);
  }
  pub fn width(&self) -> i32 {
    self.width
  }
  pub fn height(&self) -> i32 {
    self.height
  }
}
