// use super::*;
#[allow(dead_code)]
pub fn log<T: Into<wasm_bindgen::JsValue>>(value: T) {
  web_sys::console::log_1(&value.into());
}
#[allow(dead_code)]
pub fn error<T: Into<wasm_bindgen::JsValue>>(value: T) {
  web_sys::console::error_1(&value.into());
}
#[allow(dead_code)]
pub fn warning<T: Into<wasm_bindgen::JsValue>>(value: T) {
  web_sys::console::warn_1(&value.into());
}
#[allow(dead_code)]
pub fn info<T: Into<wasm_bindgen::JsValue>>(value: T) {
  web_sys::console::info_1(&value.into());
}
#[allow(dead_code)]
pub fn debug<T: Into<wasm_bindgen::JsValue>>(value: T) {
  web_sys::console::debug_1(&value.into());
}
