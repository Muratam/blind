use super::*;

pub struct IFrame {
  holder: HtmlElementHolder,
}
impl IFrame {
  pub fn new(
    parent: &dyn ElementHolderContainerTrait,
    width: Option<f32>,
    height: Option<f32>,
    src: &str,
  ) -> Self {
    let holder = HtmlElementHolder::new(parent.holder_container(), "iframe");
    if let Some(width) = width {
      holder.set_float_percentage_attribute_impl("width", width);
    } else {
      holder.set_attribute_impl("width", Some("100%"));
    }
    if let Some(height) = height {
      holder.set_float_percentage_attribute_impl("height", height);
    } else {
      holder.set_attribute_impl("height", Some("100%"));
    }
    let result = Self { holder };
    result.set_src(src);
    result
  }
  pub fn owned(
    parent: &mut dyn ElementHolderContainerTrait,
    width: Option<f32>,
    height: Option<f32>,
    src: &str,
  ) {
    parent.own(Box::new(Self::new(parent, width, height, src)));
  }
  pub fn set_src(&self, src: &str) {
    self.holder.set_attribute_impl("src", Some(src));
  }
}
impl HtmlElementHolderTrait for IFrame {
  fn holder(&self) -> &HtmlElementHolder {
    &self.holder
  }
}
impl HtmlBoxTrait for IFrame {}
