pub trait Canvas {
  fn get_2d_context(&self) -> web_sys::CanvasRenderingContext2d;
  fn get_webgl_context(&self) -> web_sys::WebGlRenderingContext;
}
impl Canvas for web_sys::HtmlCanvasElement {
  fn get_2d_context(&self) -> web_sys::CanvasRenderingContext2d {
    let context = self
      .get_context("2d")
      .expect("failed to get context 2d")
      .expect("failed to get context 2d");
    wasm_bindgen::JsCast::dyn_into::<web_sys::CanvasRenderingContext2d>(context)
      .expect("failed to cast to CanvasRenderingContext2d")
  }
  fn get_webgl_context(&self) -> web_sys::WebGlRenderingContext {
    let context = self
      .get_context("webgl")
      .expect("failed to get webgl context")
      .expect("failed to get webgl context");
    wasm_bindgen::JsCast::dyn_into::<web_sys::WebGlRenderingContext>(context)
      .expect("failed to cast to WebGlRenderingContext")
  }
}
