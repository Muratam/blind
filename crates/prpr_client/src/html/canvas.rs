pub trait Canvas {
  fn get_2d_context(&self) -> web_sys::CanvasRenderingContext2d;
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
}
