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

  pub fn owned(parent: &mut dyn ElementHolderContainerTrait, text: &str) {
    parent.own(Box::new(Self::new(parent, text)));
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

// pub struct Hr {
//   holder: HtmlElementHolder,
// }
// impl Hr {
//   pub fn new(parent: &dyn ElementHolderContainerTrait) -> Self {
//     let holder = HtmlElementHolder::new(parent.holder_container(), "hr");
//     let mut result = Self { holder };
//     // result.set_float_percentage_parameter_impl("margin-top", height * 0.5);
//     // result.set_float_percentage_parameter_impl("margin-bottom", height * 0.5);
//     // result.set_by_name_impl("border-top", "inherit");
//     result
//   }
// }
// impl HtmlElementHolderTrait for Hr {
//   fn holder(&self) -> &HtmlElementHolder {
//     &self.holder
//   }
// }
