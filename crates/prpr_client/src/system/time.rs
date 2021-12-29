use super::*;
use once_cell::sync::OnceCell;
static INSTANCE: OnceCell<RwLock<TimeImpl>> = OnceCell::new();
pub struct TimeImpl {
  frame: i64,
  pre_now_milli_sec: f64,
  processed_milli_sec: f64,
}
impl TimeImpl {
  pub fn read_global() -> RwLockReadGuard<'static, Self> {
    INSTANCE
      .get()
      .expect("time global is not initialized")
      .read()
      .unwrap()
  }
  pub fn write_global() -> RwLockWriteGuard<'static, Self> {
    INSTANCE
      .get()
      .expect("time global is not initialized")
      .write()
      .unwrap()
  }
  pub fn initialize_global() {
    INSTANCE
      .set(RwLock::new(Self {
        pre_now_milli_sec: js::date::now_millisec(),
        processed_milli_sec: 0.0,
        frame: 0,
      }))
      .ok();
  }
  pub fn pre_update(&mut self) {
    self.frame += 1;
    self.pre_now_milli_sec = js::date::now_millisec();
  }
  pub fn post_update(&mut self) {
    self.processed_milli_sec = js::date::now_millisec() - self.pre_now_milli_sec;
  }
}
pub struct Time {}
impl Time {
  pub fn frame() -> i64 {
    TimeImpl::read_global().frame
  }
  pub fn processed_milli_sec() -> f64 {
    TimeImpl::read_global().processed_milli_sec
  }
}
