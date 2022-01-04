use super::*;

static INSTANCE: OnceCell<WholeScreen> = OnceCell::new();
unsafe impl Send for WholeScreen {}
unsafe impl Sync for WholeScreen {}

pub struct WholeScreen {
  max_width: i32,
  max_height: i32,
  width: SRwLock<i32>,
  height: SRwLock<i32>,
  is_size_changed: SRwLock<bool>,
}
impl WholeScreen {
  pub fn get() -> &'static Self {
    INSTANCE
      .get()
      .expect("system::WholeScreen is not initialized")
  }
  pub fn initialize() {
    // 一度生成したら固定
    let screen = js::html::screen();
    let instance = Self {
      max_width: screen.width().unwrap(),
      max_height: screen.height().unwrap(),
      width: SRwLock::new(1),
      height: SRwLock::new(1),
      is_size_changed: SRwLock::new(true),
    };
    INSTANCE.set(instance).ok();
  }
  pub fn max_width() -> i32 {
    Self::get().max_width
  }
  pub fn max_height() -> i32 {
    Self::get().max_height
  }
  pub fn width() -> i32 {
    *Self::get().width.read()
  }
  pub fn height() -> i32 {
    *Self::get().height.read()
  }
  pub fn is_size_changed() -> bool {
    *Self::get().is_size_changed.read()
  }
  pub fn viewport() -> math::Rect<i32> {
    let width = Self::width();
    let height = Self::height();
    math::Rect::new(
      (Self::max_width() - width) / 2,
      (Self::max_height() - height) / 2,
      width,
      height,
    )
  }
  pub fn max_viewport() -> math::Rect<i32> {
    math::Rect::new(0, 0, Self::max_width(), Self::max_height())
  }
  pub fn update_size(width: i32, height: i32) {
    let pre_width = Self::width();
    let pre_height = Self::height();
    if pre_width == width && pre_height == height {
      return;
    }
    *Self::get().width.write() = width;
    *Self::get().height.write() = height;
    *Self::get().is_size_changed.write() = true;
  }
}
