pub trait Canvas {
  fn get_2d_context(&self) -> web_sys::CanvasRenderingContext2d;
  fn get_webgl2_context(&self) -> web_sys::WebGl2RenderingContext;
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
  fn get_webgl2_context(&self) -> web_sys::WebGl2RenderingContext {
    let context = self
      .get_context("webgl2")
      .expect("failed to get webgl2 context")
      .expect("failed to get webgl2 context");
    wasm_bindgen::JsCast::dyn_into::<web_sys::WebGl2RenderingContext>(context)
      .expect("failed to cast to WebGl2RenderingContext")
  }
}
