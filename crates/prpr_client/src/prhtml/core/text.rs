use super::*;

pub struct Text {
  holder: HtmlElementHolder,
  text: String,
}
impl Text {
  pub fn new(parent: &dyn ElementHolderContainerTrait, text: &str) -> Self {
    let holder = HtmlElementHolder::new(parent.holder_container(), "span");
    let mut result = Self {
      holder,
      text: String::from(""),
    };
    result.set_text(text);
    result
  }
  pub fn text(&self) -> &str {
    &self.text
  }
  pub fn set_text(&mut self, s: &str) {
    self.text = String::from(s);
    // text_content だと改行が効かない
    self.holder().raw_element().set_inner_text(s);
  }
}
impl HtmlElementHolderTrait for Text {
  fn holder(&self) -> &HtmlElementHolder {
    &self.holder
  }
}
impl HtmlBackgroundTrait for Text {}
impl HtmlBoxTrait for Text {}
impl HtmlTextConfigurableTrait for Text {}

// - H1~H6, BlockQuote, Hr, Li, Ol, Pre, Ul, A, Code,
// - Table / FlexBox(Area)
// - img, video, audio, iframe,
// - button, input, textarea
//   meter(slider, progress),
//   select datalist fieldset optgroup+option
// - other methods...
// SVG(いろいろなコンテンツ用)
// font-awesome / chart-js

// pub struct Hr {
//   raw_element: web_sys::HtmlElement,
// }
// impl Hr {
//   pub fn new(parent: &dyn HtmlContainerTrait, height: f32) -> Self {
//     let result = Self {
//       raw_element: js::html::append_tag(parent.get_raw_element(), "hr"),
//     };
//     result.set_float_percentage_parameter_impl("margin-top", height * 0.5);
//     result.set_float_percentage_parameter_impl("margin-bottom", height * 0.5);
//     result.set_by_name_impl("border-top", "inherit");
//     result
//   }
// }
// impl HtmlElementHolderTrait for Hr {
//   fn get_raw_element(&self) -> &web_sys::HtmlElement {
//     &self.raw_element
//   }
// }
// impl Drop for Hr {
//   fn drop(&mut self) {
//     self.raw_element.remove();
//   }
// }
