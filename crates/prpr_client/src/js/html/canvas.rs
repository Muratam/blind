pub fn get_canvas_2d_context(
  canvas: &web_sys::HtmlCanvasElement,
) -> web_sys::CanvasRenderingContext2d {
  let context = canvas
    .get_context("2d")
    .expect("failed to get context 2d")
    .expect("failed to get context 2d");
  wasm_bindgen::JsCast::dyn_into::<web_sys::CanvasRenderingContext2d>(context)
    .expect("failed to cast to CanvasRenderingContext2d")
}
pub fn get_webgl2_context(canvas: &web_sys::HtmlCanvasElement) -> web_sys::WebGl2RenderingContext {
  let context = canvas
    .get_context("webgl2")
    .expect("failed to get webgl2 context")
    .expect("failed to get webgl2 context");
  wasm_bindgen::JsCast::dyn_into::<web_sys::WebGl2RenderingContext>(context)
    .expect("failed to cast to WebGl2RenderingContext")
}
