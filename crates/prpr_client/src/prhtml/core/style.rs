use super::*;

// 小さすぎると崩れるので、ある程度の大きさのheightを仮定している
const EXPECTED_BROWSER_HEIGHT: f32 = 1000.0;
fn percent(x: f32) -> f32 {
  x * EXPECTED_BROWSER_HEIGHT * 0.01
}
pub enum Gradation {
  Linear(f32, Vec<Color>),            // degree, colors
  Radial(bool, f32, f32, Vec<Color>), // is_circle, x, y, colors
}
impl Gradation {
  fn to_css(&self) -> String {
    match self {
      Self::Linear(degree, colors) => {
        let mut result = format!("linear-gradient({}deg ", *degree as i32);
        for color in colors {
          result += &format!(", {}", color.to_css());
        }
        result += ")";
        result
      }
      Self::Radial(is_circle, x, y, colors) => {
        let mut result = format!(
          "radial-gradient({} at {:.2}% {:.2}%, ",
          if *is_circle { "circle" } else { "ellipse" },
          *x * 100.0,
          *y * 100.0,
        );
        for color in colors {
          result += &format!(", {}", color.to_css());
        }
        result += ")";
        result
      }
    }
  }
}

pub enum Filter {
  Blur(f32),                        // px per
  Brightness(f32),                  // 1.0: Identity
  Contrast(f32),                    // 1.0: Identity
  DropShadow(f32, f32, f32, Color), // x, y, r, color
  GrayScale(f32),                   // 0.0: Identity
  HueRotate(f32),                   // Degree
  Invert(f32),                      // 0.0: Identity
  Opacity(f32),                     // 1.0: Identity
  Saturate(f32),                    // 1.0: Identity
  Sepia(f32),                       // 1.0: Identity
}
impl Filter {
  fn value(&self) -> String {
    match self {
      Self::Blur(per) => format!("blur({}px)", percent(*per)),
      Self::Brightness(x) => format!("brightness({:.4})", x),
      Self::Contrast(x) => format!("contrast({:.4})", x),
      Self::GrayScale(x) => format!("grayscale({:.4})", x),
      Self::HueRotate(degree) => format!("hue-rotate({:.4}deg)", degree),
      Self::Invert(x) => format!("invert({:.4})", x),
      Self::Opacity(x) => format!("opacity({:.4})", x),
      Self::Saturate(x) => format!("saturate({:.4})", x),
      Self::Sepia(x) => format!("sepia({:.4})", x),
      Self::DropShadow(x, y, r, color) => format!(
        "drop-shadow({}px {}px {}px {} ",
        percent(*x),
        percent(*y),
        percent(*r),
        color.to_css()
      ),
    }
  }
}

#[derive(Clone, Copy)]
pub enum BorderStyle {
  Solid,
  Double,
  Hidden,
  Dashed,
}
impl BorderStyle {
  fn value(&self) -> &'static str {
    match self {
      Self::Solid => "solid",
      Self::Double => "double",
      Self::Hidden => "hidden",
      Self::Dashed => "dashed",
    }
  }
}
pub trait ContainerTrait {
  fn get_raw_element(&self) -> &web_sys::HtmlElement;
  fn set_by_name_impl(&self, key: &str, value: &str) {
    let style = self.get_raw_element().style();
    if style.set_property(key, value).is_err() {
      log::error(format!("failted to set_property: {} -> {}", key, value));
    }
  }
  fn set_float_percentage_parameter_impl(&self, key: &str, value: f32) {
    self.set_by_name_impl(key, &format!("{}px", percent(value)));
  }
  fn set_color_impl(&self, key: &str, color: Color) {
    self.set_by_name_impl(key, &color.to_css());
  }
  fn set_shadow_impl(&self, key: &str, dx: f32, dy: f32, blur_radius: f32, color: Color) {
    self.set_by_name_impl(
      key,
      &format!(
        "{}px {}px {}px {}",
        percent(dx),
        percent(dy),
        percent(blur_radius),
        color.to_css()
      ),
    );
  }

  // OVERALL
  fn set_padding(&self, percent: f32) {
    self.set_float_percentage_parameter_impl("padding", percent);
  }
  fn set_filter(&self, filter: &Vec<Filter>) {
    if filter.len() == 0 {
      self.set_by_name_impl("filter", "none");
    } else {
      self.set_by_name_impl(
        "filter",
        &filter
          .iter()
          .map(|x| x.value())
          .collect::<Vec<_>>()
          .join(" "),
      );
    }
  }

  // BACKGROUND
  fn set_background_color(&self, color: Color) {
    self.set_color_impl("background-color", color);
  }
  fn set_background_gradation(&self, gradation: &Gradation) {
    self.set_by_name_impl("background", &gradation.to_css());
  }
  fn set_background_shadow(&self, dx: f32, dy: f32, blur_radius: f32, color: Color) {
    self.set_shadow_impl("box-shadow", dx, dy, blur_radius, color);
  }

  // BORDER
  fn set_border_color(&self, color: Color) {
    self.set_color_impl("border-color", color);
  }
  fn set_border_radius(&self, percent: f32) {
    self.set_float_percentage_parameter_impl("border-radius", percent);
  }
  fn set_border_width(&self, percent: f32) {
    self.set_float_percentage_parameter_impl("border-width", percent);
  }
  fn set_border_style(&self, border_style: BorderStyle) {
    self.set_by_name_impl("border-style", border_style.value());
  }

  // TEXT
  fn set_text_color(&self, color: Color) {
    self.set_color_impl("color", color);
  }
  fn set_text_shadow(&self, dx: f32, dy: f32, blur_radius: f32, color: Color) {
    self.set_shadow_impl("text-shadow", dx, dy, blur_radius, color);
  }
  fn set_text_size(&self, percent: f32) {
    self.set_float_percentage_parameter_impl("font-size", percent);
  }
  fn set_text_line_height(&self, percent: f32) {
    self.set_float_percentage_parameter_impl("line-height", percent);
  }
  fn set_text_letter_spacing(&self, percent: f32) {
    self.set_float_percentage_parameter_impl("letter-spacing", percent);
  }
  fn set_text_bold(&self, is_bold: bool) {
    self.set_by_name_impl("font-weight", if is_bold { "bold" } else { "normal" });
  }
  fn set_text_italic(&self, is_italic: bool) {
    self.set_by_name_impl("font-style", if is_italic { "italic" } else { "normal" });
  }

  // EXPERIMENTAL
  fn set_background_textclip(&self) {
    // to clip to gradation
    self.set_by_name_impl("background-clip", "text");
    self.set_by_name_impl("-webkit-background-clip", "text");
    self.set_by_name_impl("color", "transparent");
    self.set_by_name_impl("text-shadow", "none");
  }
}
