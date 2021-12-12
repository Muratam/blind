use crate::js;
#[allow(dead_code)]
pub fn error<T: Into<wasm_bindgen::JsValue>>(value: T) {
  js::console::error(value);
}
#[allow(dead_code)]
pub fn warning<T: Into<wasm_bindgen::JsValue>>(value: T) {
  js::console::warning(value);
}
#[allow(dead_code)]
pub fn info<T: Into<wasm_bindgen::JsValue>>(value: T) {
  js::console::info(value);
}

#[allow(dead_code)]
pub fn debug<T: Into<wasm_bindgen::JsValue>>(value: T) {
  js::console::debug(value);
}
