use super::*;
use once_cell::sync::OnceCell;
static TIME_GLOBAL_INSTANCE: OnceCell<RwLock<TimeGlobal>> = OnceCell::new();
pub struct TimeGlobal {
  frame: i64,
  pre_now_milli_sec: f64,
  processed_milli_sec: f64,
}
impl TimeGlobal {
  pub fn read() -> RwLockReadGuard<'static, Self> {
    TIME_GLOBAL_INSTANCE
      .get()
      .expect("time global not initialized")
      .read()
      .unwrap()
  }
  pub fn write() -> RwLockWriteGuard<'static, Self> {
    TIME_GLOBAL_INSTANCE
      .get()
      .expect("time global not initialized")
      .write()
      .unwrap()
  }
  pub fn initialize() {
    TIME_GLOBAL_INSTANCE
      .set(RwLock::new(TimeGlobal {
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
    TimeGlobal::read().frame
  }
  pub fn processed_milli_sec() -> f64 {
    TimeGlobal::read().processed_milli_sec
  }
}
