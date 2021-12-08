pub fn log<T: Into<wasm_bindgen::JsValue>>(value: T) {
  web_sys::console::log_1(&value.into());
}
pub fn error<T: Into<wasm_bindgen::JsValue>>(value: T) {
  web_sys::console::error_1(&value.into());
}
