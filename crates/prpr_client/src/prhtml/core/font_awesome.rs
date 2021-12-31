use super::*;

pub struct Fa {
  holder: HtmlElementHolder,
}
impl Fa {
  pub fn new(parent: &dyn ElementHolderContainerTrait, classes: &str) -> Self {
    let holder = HtmlElementHolder::new(parent.holder_container(), "i");
    let mut result = Self { holder };
    result.set_icon(classes);
    result
  }
  pub fn owned(parent: &mut dyn ElementHolderContainerTrait, classes: &str) {
    parent.own(Box::new(Self::new(parent, classes)));
  }
  pub fn set_icon(&mut self, classes: &str) {
    for class in classes.split(" ") {
      self
        .holder()
        .raw_element()
        .class_list()
        .add_1(class)
        .expect("failed to set class");
    }
  }
}
impl HtmlElementHolderTrait for Fa {
  fn holder(&self) -> &HtmlElementHolder {
    &self.holder
  }
}
impl HtmlBackgroundTrait for Fa {}
impl HtmlBoxTrait for Fa {}
impl HtmlTextConfigurableTrait for Fa {}
