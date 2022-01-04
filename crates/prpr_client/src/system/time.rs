use super::*;
static INSTANCE: OnceCell<MRwLock<TimeImpl>> = OnceCell::new();
pub struct TimeImpl {
  frame: i64,
  started_milli_sec: f64,
  pre_now_milli_sec: f64,
  now_milli_sec: f64,
  processed_milli_sec: f64,
}
impl TimeImpl {
  pub fn read_global() -> MDerefable<'static, Self> {
    INSTANCE
      .get()
      .expect("time global is not initialized")
      .read()
  }
  pub fn write_global() -> MDerefMutable<'static, Self> {
    INSTANCE
      .get()
      .expect("time global is not initialized")
      .write()
  }
  pub fn initialize_global() {
    INSTANCE
      .set(MRwLock::new(Self {
        started_milli_sec: js_sys::Date::now(),
        pre_now_milli_sec: 0.0,
        now_milli_sec: 0.0,
        processed_milli_sec: 0.0,
        frame: 0,
      }))
      .ok();
  }
  pub fn pre_update(&mut self) {
    self.frame += 1;
    self.pre_now_milli_sec = js_sys::Date::now() - self.started_milli_sec;
  }
  pub fn post_update(&mut self) {
    self.now_milli_sec = js_sys::Date::now() - self.started_milli_sec;
    self.processed_milli_sec = self.now_milli_sec - self.pre_now_milli_sec;
  }
}
pub struct Time {}
impl Time {
  pub fn frame() -> i64 {
    TimeImpl::read_global().frame
  }
  pub fn now_milli_sec() -> f64 {
    TimeImpl::read_global().now_milli_sec
  }
  pub fn processed_milli_sec() -> f64 {
    TimeImpl::read_global().processed_milli_sec
  }
}
