use super::*;

pub struct Text {
  raw_element: web_sys::HtmlElement,
}
impl Text {
  pub fn new(parent: &dyn HtmlContainerTrait) -> Self {
    Self {
      raw_element: js::html::append_span(parent.get_raw_element()),
    }
  }
}
impl HtmlTextConfigurableTrait for Text {}
impl HtmlTextHolderTrait for Text {}
impl HtmlElementHolderTrait for Text {
  fn get_raw_element(&self) -> &web_sys::HtmlElement {
    &self.raw_element
  }
}
impl Drop for Text {
  fn drop(&mut self) {
    self.raw_element.remove();
  }
}
