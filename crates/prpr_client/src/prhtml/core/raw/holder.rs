use super::*;

struct HtmlElementHolderInternal {
  display: Option<String>, // DisplayNone対策
  box_shadow: Option<String>,
  box_inset_shadow: Option<String>,
  filter_dropshadow: Whys<FilterDropShadow>,
  filter_blur: Whys<FilterBlur>,
  filter_huerotate: Whys<FilterHueRotate>,
  filter_brightness: Whys<FilterBrightness>,
  filter_contrast: Whys<FilterContrast>,
  filter_grayscale: Whys<FilterGrayscale>,
  filter_opacity: Whys<FilterOpacity>,
  transform: Whys<HtmlTransform>,
  // skew(3次元に移す場合)
}

pub struct HtmlElementHolder {
  raw_element: web_sys::HtmlElement,
  internal: RwLock<HtmlElementHolderInternal>,
}
impl Drop for HtmlElementHolder {
  fn drop(&mut self) {
    self.raw_element.remove();
  }
}

impl HtmlElementHolder {
  pub fn new(parent: &web_sys::HtmlElement, tag: &str) -> Self {
    Self {
      raw_element: js::html::append_tag(parent, tag),
      internal: RwLock::new(HtmlElementHolderInternal {
        display: None,
        box_shadow: None,
        box_inset_shadow: None,
        filter_dropshadow: Whys::new(),
        filter_blur: Whys::new(),
        filter_huerotate: Whys::new(),
        filter_brightness: Whys::new(),
        filter_contrast: Whys::new(),
        filter_grayscale: Whys::new(),
        filter_opacity: Whys::new(),
        transform: Whys::new(),
      }),
    }
  }
  pub fn raw_element(&self) -> &web_sys::HtmlElement {
    &self.raw_element
  }

  // Raw Style
  pub fn set_by_name_impl(&self, key: &str, value: &str) {
    let style = self.raw_element.style();
    if style.set_property(key, value).is_err() {
      log::error(format!("failted to set_property: {} -> {}", key, value));
    }
  }
  pub fn set_float_percentage_parameter_impl(&self, key: &str, value: f32) {
    self.set_by_name_impl(key, &convert_percent_str(value));
  }
  pub fn set_color_impl(&self, key: &str, rgba: Vec4) {
    self.set_by_name_impl(key, &rgba_to_css_value(rgba));
  }
  fn parse_shadow(&self, shadow: FilterDropShadow, inset: bool) -> String {
    format!(
      "{} {} {} {} {}",
      convert_percent_str(shadow.x),
      convert_percent_str(shadow.y),
      convert_percent_str(shadow.r),
      &rgba_to_css_value(shadow.rgba),
      if inset { "inset" } else { "" }
    )
  }
  fn update_filter_impl(&self) {
    let mut text = String::from("");
    let internal = self.internal.read().unwrap();
    if let Some(calc) = internal.filter_dropshadow.calc() {
      text += &calc.to_css_value();
    }
    if let Some(calc) = internal.filter_blur.calc() {
      text += &calc.to_css_value();
    }
    if let Some(calc) = internal.filter_huerotate.calc() {
      text += &calc.to_css_value();
    }
    if let Some(calc) = internal.filter_brightness.calc() {
      text += &calc.to_css_value();
    }
    if let Some(calc) = internal.filter_contrast.calc() {
      text += &calc.to_css_value();
    }
    if let Some(calc) = internal.filter_grayscale.calc() {
      text += &calc.to_css_value();
    }
    if let Some(calc) = internal.filter_opacity.calc() {
      text += &calc.to_css_value();
    }
    if text == "" {
      self.set_by_name_impl("filter", "none");
    } else {
      self.set_by_name_impl("filter", &text);
    }
  }

  fn set_text_decoration_impl(&self, line: &str, style: TextDecorationStyle, rgba: Vec4) {
    if style == TextDecorationStyle::None {
      self.set_by_name_impl("text-decoration-line", "none");
    } else {
      self.set_by_name_impl("text-decoration-line", line);
      self.set_by_name_impl("text-decoration-style", style.to_css_value());
    }
    self.set_color_impl("text-decoration-color", rgba);
  }
  fn set_background_textclip_impl(&self) {
    // TODO: text clip
    // to clip to gradation
    // self.set_by_name_impl("background-clip", "text");
    // self.set_by_name_impl("-webkit-background-clip", "text");
    // self.set_by_name_impl("color", "transparent");
    // self.set_by_name_impl("text-shadow", "none");
    system::log::error("background textclop is not implemented");
  }
}

// API
impl HtmlElementHolder {
  // OVERALL
  pub fn set_cursor(&self, cursor: Cursor) {
    self.set_by_name_impl("cursor", cursor.to_css_value());
  }
  pub fn set_filter_blur(&self, filter: Option<f32>, why: Why) {
    self
      .internal
      .write()
      .unwrap()
      .filter_blur
      .set(filter.map(|x| FilterBlur(x)), why);
    self.update_filter_impl();
  }
  pub fn set_filter_huerotate(&self, filter: Option<f32>, why: Why) {
    self
      .internal
      .write()
      .unwrap()
      .filter_huerotate
      .set(filter.map(|x| FilterHueRotate(x)), why);
    self.update_filter_impl();
  }
  pub fn set_filter_brightness(&self, filter: Option<f32>, why: Why) {
    self
      .internal
      .write()
      .unwrap()
      .filter_brightness
      .set(filter.map(|x| FilterBrightness(x)), why);
    self.update_filter_impl();
  }
  pub fn set_filter_contrast(&self, filter: Option<f32>, why: Why) {
    self
      .internal
      .write()
      .unwrap()
      .filter_contrast
      .set(filter.map(|x| FilterContrast(x)), why);
    self.update_filter_impl();
  }
  pub fn set_filter_grayscale(&self, filter: Option<f32>, why: Why) {
    self
      .internal
      .write()
      .unwrap()
      .filter_grayscale
      .set(filter.map(|x| FilterGrayscale(x)), why);
  }
  pub fn set_filter_opacity(&self, filter: Option<f32>, why: Why) {
    self
      .internal
      .write()
      .unwrap()
      .filter_opacity
      .set(filter.map(|x| FilterOpacity(x)), why);
    self.update_filter_impl();
  }
  pub fn set_filter_dropshadow(&self, filter: Option<FilterDropShadow>, why: Why) {
    self
      .internal
      .write()
      .unwrap()
      .filter_dropshadow
      .set(filter, why);
    self.update_filter_impl();
  }
  pub fn set_visibility(&self, visibility: bool) {
    self.set_by_name_impl("visibility", if visibility { "visible" } else { "hidden" });
  }
  pub fn set_display_none(&self, is_none: bool) {
    if is_none {
      if let Some(pre) = self.raw_element.style().get_property_value("display").ok() {
        if pre != "none" {
          self.internal.write().unwrap().display = Some(pre);
        }
      }
      self.set_by_name_impl("display", "none");
    } else {
      if let Some(pre) = &self.internal.read().unwrap().display {
        self.set_by_name_impl("display", &pre);
      } else {
        self.raw_element.style().remove_property("display");
      }
    }
  }

  // BACKGROUND
  pub fn set_background_color(&self, rgba: Vec4) {
    self.set_color_impl("background-color", rgba);
  }
  pub fn set_background_gradation(&self, gradation: &Gradation) {
    self.set_by_name_impl("background", &gradation.to_css_value());
  }

  // BOX + BORDER
  pub fn set_box_shadow(&self, x: f32, y: f32, r: f32, rgba: Vec4) {
    let shadow = self.parse_shadow(FilterDropShadow { x, y, r, rgba }, false);
    let mut text = shadow.clone();
    if let Some(another) = &self.internal.read().unwrap().box_inset_shadow {
      text += &format!(", {}", another);
    }
    self.set_by_name_impl("box-shadow", &text);
    self.internal.write().unwrap().box_shadow = Some(shadow);
  }
  pub fn set_box_shadow_inset(&self, x: f32, y: f32, r: f32, rgba: Vec4) {
    let shadow = self.parse_shadow(FilterDropShadow { x, y, r, rgba }, false);
    let mut text = shadow.clone();
    if let Some(another) = &self.internal.read().unwrap().box_shadow {
      text += &format!(", {}", another);
    }
    self.set_by_name_impl("box-shadow", &text);
    self.internal.write().unwrap().box_inset_shadow = Some(shadow);
  }
  pub fn set_border_radius(&self, percent: f32) {
    self.set_float_percentage_parameter_impl("border-radius", percent);
  }
  pub fn set_border_color(&self, rgba: Vec4) {
    self.set_border_x_color(rgba);
    self.set_border_y_color(rgba);
  }
  pub fn set_border_width(&self, percent: f32) {
    self.set_border_x_width(percent);
    self.set_border_y_width(percent);
  }
  pub fn set_border_style(&self, border_style: BorderStyle) {
    self.set_border_x_style(border_style);
    self.set_border_y_style(border_style);
  }

  // TEXT
  pub fn set_text_color(&self, rgba: Vec4) {
    self.set_color_impl("color", rgba);
  }
  pub fn set_text_shadow(&self, x: f32, y: f32, r: f32, rgba: Vec4) {
    let text = self.parse_shadow(FilterDropShadow { x, y, r, rgba }, false);
    self.set_by_name_impl("text-shadow", &text);
  }
  pub fn set_text_size(&self, percent: f32) {
    self.set_float_percentage_parameter_impl("font-size", percent);
  }
  pub fn set_text_line_height(&self, percent: f32) {
    self.set_float_percentage_parameter_impl("line-height", percent);
  }
  pub fn set_text_letter_spacing(&self, percent: f32) {
    self.set_float_percentage_parameter_impl("letter-spacing", percent);
  }
  pub fn set_text_bold(&self, is_bold: bool) {
    self.set_by_name_impl("font-weight", if is_bold { "bold" } else { "normal" });
  }
  pub fn set_text_italic(&self, is_italic: bool) {
    self.set_by_name_impl("font-style", if is_italic { "italic" } else { "normal" });
  }
  pub fn set_text_underline(&self, style: TextDecorationStyle, rgba: Vec4) {
    self.set_text_decoration_impl("underline", style, rgba);
  }
  pub fn set_text_linethrough(&self, style: TextDecorationStyle, rgba: Vec4) {
    self.set_text_decoration_impl("line-through", style, rgba);
  }

  // CONTAINER
  pub fn set_align(&self, align: Align) {
    self.set_by_name_impl("text-align", align.to_css_value());
  }
  pub fn set_padding(&self, percent: f32) {
    self.set_padding_x(percent);
    self.set_padding_y(percent);
  }
}

// aliases
impl HtmlElementHolder {
  // XY
  pub fn set_padding_x(&self, percent: f32) {
    self.set_padding_left(percent);
    self.set_padding_right(percent);
  }
  pub fn set_padding_y(&self, percent: f32) {
    self.set_padding_top(percent);
    self.set_padding_bottom(percent);
  }
  pub fn set_border_x_color(&self, rgba: Vec4) {
    self.set_border_left_color(rgba);
    self.set_border_right_color(rgba);
  }
  pub fn set_border_y_color(&self, rgba: Vec4) {
    self.set_border_bottom_color(rgba);
    self.set_border_top_color(rgba);
  }
  pub fn set_border_x_width(&self, percent: f32) {
    self.set_border_left_width(percent);
    self.set_border_right_width(percent);
  }
  pub fn set_border_y_width(&self, percent: f32) {
    self.set_border_bottom_width(percent);
    self.set_border_top_width(percent);
  }
  pub fn set_border_x_style(&self, border_style: BorderStyle) {
    self.set_border_left_style(border_style);
    self.set_border_right_style(border_style);
  }
  pub fn set_border_y_style(&self, border_style: BorderStyle) {
    self.set_border_bottom_style(border_style);
    self.set_border_top_style(border_style);
  }

  // LRTB
  pub fn set_padding_left(&self, percent: f32) {
    self.set_float_percentage_parameter_impl("padding-left", percent);
  }
  pub fn set_padding_right(&self, percent: f32) {
    self.set_float_percentage_parameter_impl("padding-right", percent);
  }
  pub fn set_padding_top(&self, percent: f32) {
    self.set_float_percentage_parameter_impl("padding-top", percent);
  }
  pub fn set_padding_bottom(&self, percent: f32) {
    self.set_float_percentage_parameter_impl("padding-bottom", percent);
  }
  pub fn set_border_left_color(&self, rgba: Vec4) {
    self.set_color_impl("border-left-color", rgba);
  }
  pub fn set_border_left_width(&self, percent: f32) {
    self.set_float_percentage_parameter_impl("border-left-width", percent);
  }
  pub fn set_border_left_style(&self, border_style: BorderStyle) {
    self.set_by_name_impl("border-left-style", border_style.to_css_value());
  }
  pub fn set_border_right_color(&self, rgba: Vec4) {
    self.set_color_impl("border-right-color", rgba);
  }
  pub fn set_border_right_width(&self, percent: f32) {
    self.set_float_percentage_parameter_impl("border-right-width", percent);
  }
  pub fn set_border_right_style(&self, border_style: BorderStyle) {
    self.set_by_name_impl("border-right-style", border_style.to_css_value());
  }
  pub fn set_border_top_color(&self, rgba: Vec4) {
    self.set_color_impl("border-top-color", rgba);
  }
  pub fn set_border_top_width(&self, percent: f32) {
    self.set_float_percentage_parameter_impl("border-top-width", percent);
  }
  pub fn set_border_top_style(&self, border_style: BorderStyle) {
    self.set_by_name_impl("border-top-style", border_style.to_css_value());
  }
  pub fn set_border_bottom_color(&self, rgba: Vec4) {
    self.set_color_impl("border-bottom-color", rgba);
  }
  pub fn set_border_bottom_width(&self, percent: f32) {
    self.set_float_percentage_parameter_impl("border-bottom-width", percent);
  }
  pub fn set_border_bottom_style(&self, border_style: BorderStyle) {
    self.set_by_name_impl("border-bottom-style", border_style.to_css_value());
  }
}
