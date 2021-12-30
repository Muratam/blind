use super::*;
struct XorShift128Impl {
  pub x: u32,
  pub y: u32,
  pub z: u32,
  pub w: u32,
}
impl XorShift128Impl {
  fn new(seed: u32) -> Self {
    Self {
      x: seed,
      y: seed << 8,
      z: seed << 16,
      w: seed << 24,
    }
  }
  fn new_fixed() -> Self {
    Self {
      x: 123456789,
      y: 362436069,
      z: 521288629,
      w: 88675123,
    }
  }
  fn next(&mut self) -> u32 {
    let t = self.x ^ (self.x << 11);
    self.x = self.y;
    self.y = self.z;
    self.z = self.w;
    self.w = (self.w ^ (self.w >> 19)) ^ (t ^ (t >> 8));
    self.w
  }
}
pub struct XorShift128 {
  data: std::sync::Mutex<XorShift128Impl>,
}
impl XorShift128 {
  pub fn initialize_global(seed: u32) {
    INSTANCE.set(Self::new(seed)).ok();
  }
  pub fn global() -> &'static Self {
    INSTANCE.get().expect("XorShift128 global not initialized")
  }
  pub fn new_fixed() -> Self {
    Self {
      data: std::sync::Mutex::new(XorShift128Impl::new_fixed()),
    }
  }
  pub fn new(seed: u32) -> Self {
    Self {
      data: std::sync::Mutex::new(XorShift128Impl::new(seed)),
    }
  }
  pub fn next(&self) -> u32 {
    self.data.lock().unwrap().next()
  }
  pub fn uniform(&self) -> f64 {
    self.next() as f64 / u32::MAX as f64
  }
  pub fn asciis(&self, len: usize) -> String {
    let mut data = self.data.lock().unwrap();
    let mut result: Vec<u8> = Vec::new();
    for _ in 0..len {
      let n = data.next();
      result.push(((n % (0x7f - 0x20)) + 0x20) as u8);
    }
    result.into_iter().map(char::from).collect()
  }
}

static INSTANCE: OnceCell<XorShift128> = OnceCell::new();
unsafe impl Send for XorShift128 {}
unsafe impl Sync for XorShift128 {}
