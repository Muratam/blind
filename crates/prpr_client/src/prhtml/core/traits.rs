use super::*;

// 最低限仮定できるもの
pub trait HtmlElementHolderTrait {
  fn holder(&self) -> &HtmlElementHolder;
  fn set_cursor(&self, cursor: Cursor) {
    self.holder().set_cursor(cursor);
  }
  fn set_translate(&self, translate: Vec2, why: Why) {
    self.holder().set_translate(translate, why);
  }
  fn set_rotate_degree(&self, rotate_deg: f32, why: Why) {
    self.holder().set_rotate_degree(rotate_deg, why);
  }
  fn set_scale(&self, scale: f32, why: Why) {
    self.holder().set_scale(scale, why);
  }
  fn set_transform(&self, translate: Vec2, rotate_deg: f32, scale: f32, why: Why) {
    self
      .holder()
      .set_transform(translate, rotate_deg, scale, why);
  }

  fn set_filter_blur(&self, filter: Option<f32>, why: Why) {
    self.holder().set_filter_blur(filter, why);
  }
  fn set_filter_huerotate(&self, filter: Option<f32>, why: Why) {
    self.holder().set_filter_huerotate(filter, why);
  }
  fn set_filter_brightness(&self, filter: Option<f32>, why: Why) {
    self.holder().set_filter_brightness(filter, why);
  }
  fn set_filter_contrast(&self, filter: Option<f32>, why: Why) {
    self.holder().set_filter_contrast(filter, why);
  }
  fn set_filter_grayscale(&self, filter: Option<f32>, why: Why) {
    self.holder().set_filter_grayscale(filter, why);
  }
  fn set_filter_opacity(&self, filter: Option<f32>, why: Why) {
    self.holder().set_filter_opacity(filter, why);
  }
  fn set_filter_dropshadow(&self, filter: Option<FilterDropShadow>, why: Why) {
    self.holder().set_filter_dropshadow(filter, why);
  }
  fn set_visibility(&self, visibility: bool) {
    self.holder().set_visibility(visibility);
  }
}

// Background
pub trait HtmlBackgroundTrait
where
  Self: HtmlElementHolderTrait,
{
  fn set_background_color(&self, rgba: Vec4) {
    self.holder().set_background_color(rgba);
  }
  fn set_background_gradation(&self, gradation: &Gradation) {
    self.holder().set_background_gradation(gradation);
  }
}

// Box := BoxShadow + Border
pub trait HtmlBoxTrait
where
  Self: HtmlElementHolderTrait,
{
  fn set_box_shadow(&self, dx: f32, dy: f32, blur_radius: f32, rgba: Vec4) {
    self.holder().set_box_shadow(dx, dy, blur_radius, rgba)
  }
  fn set_box_shadow_inset(&self, dx: f32, dy: f32, blur_radius: f32, rgba: Vec4) {
    self
      .holder()
      .set_box_shadow_inset(dx, dy, blur_radius, rgba)
  }

  fn set_border_radius(&self, percent: f32) {
    self.holder().set_border_radius(percent);
  }
  fn set_border_color(&self, rgba: Vec4) {
    self.holder().set_border_color(rgba);
  }
  fn set_border_width(&self, percent: f32) {
    self.holder().set_border_width(percent);
  }
  fn set_border_style(&self, border_style: BorderStyle) {
    self.holder().set_border_style(border_style);
  }
  // XY
  fn set_border_x_color(&self, rgba: Vec4) {
    self.holder().set_border_x_color(rgba);
  }
  fn set_border_y_color(&self, rgba: Vec4) {
    self.holder().set_border_y_color(rgba);
  }
  fn set_border_x_width(&self, percent: f32) {
    self.holder().set_border_x_width(percent);
  }
  fn set_border_y_width(&self, percent: f32) {
    self.holder().set_border_y_width(percent);
  }
  fn set_border_x_style(&self, border_style: BorderStyle) {
    self.holder().set_border_x_style(border_style);
  }
  fn set_border_y_style(&self, border_style: BorderStyle) {
    self.holder().set_border_y_style(border_style);
  }

  // LRTB
  fn set_border_left_color(&self, rgba: Vec4) {
    self.holder().set_border_left_color(rgba);
  }
  fn set_border_left_width(&self, percent: f32) {
    self.holder().set_border_left_width(percent);
  }
  fn set_border_left_style(&self, border_style: BorderStyle) {
    self.holder().set_border_left_style(border_style);
  }
  fn set_border_right_color(&self, rgba: Vec4) {
    self.holder().set_border_right_color(rgba);
  }
  fn set_border_right_width(&self, percent: f32) {
    self.holder().set_border_right_width(percent);
  }
  fn set_border_right_style(&self, border_style: BorderStyle) {
    self.holder().set_border_right_style(border_style);
  }
  fn set_border_top_color(&self, rgba: Vec4) {
    self.holder().set_border_top_color(rgba);
  }
  fn set_border_top_width(&self, percent: f32) {
    self.holder().set_border_top_width(percent);
  }
  fn set_border_top_style(&self, border_style: BorderStyle) {
    self.holder().set_border_top_style(border_style);
  }
  fn set_border_bottom_color(&self, rgba: Vec4) {
    self.holder().set_border_bottom_color(rgba);
  }
  fn set_border_bottom_width(&self, percent: f32) {
    self.holder().set_border_bottom_width(percent);
  }
  fn set_border_bottom_style(&self, border_style: BorderStyle) {
    self.holder().set_border_bottom_style(border_style);
  }
}

// 中のテキストの内容をいじれるもの
pub trait HtmlTextConfigurableTrait
where
  Self: HtmlElementHolderTrait,
{
  fn set_text_color(&self, rgba: Vec4) {
    self.holder().set_text_color(rgba);
  }
  fn set_text_shadow(&self, dx: f32, dy: f32, blur_radius: f32, rgba: Vec4) {
    self.holder().set_text_shadow(dx, dy, blur_radius, rgba)
  }
  fn set_text_size(&self, percent: f32) {
    self.holder().set_text_size(percent);
  }
  fn set_text_line_height(&self, percent: f32) {
    self.holder().set_text_line_height(percent);
  }
  fn set_text_letter_spacing(&self, percent: f32) {
    self.holder().set_text_letter_spacing(percent);
  }
  fn set_text_bold(&self, is_bold: bool) {
    self.holder().set_text_bold(is_bold);
  }
  fn set_text_italic(&self, is_italic: bool) {
    self.holder().set_text_italic(is_italic);
  }
  fn set_text_underline(&self, style: TextDecorationStyle, rgba: Vec4) {
    self.holder().set_text_underline(style, rgba);
  }
  fn set_text_linethrough(&self, style: TextDecorationStyle, rgba: Vec4) {
    self.holder().set_text_linethrough(style, rgba);
  }
}

// 中に別の要素を入れられるもの
pub trait HtmlContainerTrait
where
  Self: HtmlElementHolderTrait,
{
  fn set_align(&self, align: Align) {
    self.holder().set_align(align);
  }
  fn set_padding(&self, percent: f32) {
    self.holder().set_padding(percent);
  }
  fn set_padding_x(&self, percent: f32) {
    self.holder().set_padding_x(percent);
  }
  fn set_padding_y(&self, percent: f32) {
    self.holder().set_padding_y(percent);
  }
  fn set_padding_left(&self, percent: f32) {
    self.holder().set_padding_left(percent);
  }
  fn set_padding_right(&self, percent: f32) {
    self.holder().set_padding_right(percent);
  }
  fn set_padding_top(&self, percent: f32) {
    self.holder().set_padding_top(percent);
  }
  fn set_padding_bottom(&self, percent: f32) {
    self.holder().set_padding_bottom(percent);
  }
}

// href できるもの(Aタグがいりそう)
pub trait HrefTrait
where
  Self: HtmlElementHolderTrait,
{
  fn set_href(&mut self, href: Option<&str>) {
    self.holder().set_attribute_impl("href", href);
  }
}

// HTMLHolderの親になれるもの
pub trait ElementHolderContainerTrait {
  fn holder_container(&self) -> &web_sys::HtmlElement;
  fn own(&mut self, elem: Box<dyn HtmlElementHolderTrait>);
}
