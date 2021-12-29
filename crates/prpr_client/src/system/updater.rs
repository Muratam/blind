use super::*;
pub trait Updater {
  fn update(&mut self);
}

// WARN: 本当はUpdaterの方に制約を課す必要がありそう
use once_cell::sync::OnceCell;
static INSTANCE: OnceCell<RwLock<UpdaterExecuter>> = OnceCell::new();
unsafe impl Send for UpdaterExecuter {}
unsafe impl Sync for UpdaterExecuter {}

struct UpdaterExecuteInfo {
  updater: Weak<RwLock<dyn Updater>>,
  order: Option<usize>, // asc
}

pub struct UpdaterExecuter {
  updaters: Vec<Arc<UpdaterExecuteInfo>>,
  owns: Vec<Arc<RwLock<dyn Updater>>>,
  need_sort: bool,
}
impl UpdaterExecuter {
  pub fn global_read_lock() -> RwLockReadGuard<'static, Self> {
    INSTANCE
      .get()
      .expect("UpdaterExecuter global not initialized")
      .read()
      .unwrap()
  }
  pub fn global_write_lock() -> RwLockWriteGuard<'static, Self> {
    INSTANCE
      .get()
      .expect("UpdaterExecuter global not initialized")
      .write()
      .unwrap()
  }
  pub fn global_initialize() {
    INSTANCE.set(RwLock::new(UpdaterExecuter::new())).ok();
  }

  pub fn new() -> Self {
    Self {
      updaters: Vec::new(),
      need_sort: false,
      owns: Vec::new(),
    }
  }
  pub fn add<T: Updater + 'static>(&mut self, updater: &Arc<RwLock<T>>, order: Option<usize>) {
    self.updaters.push(Arc::new(UpdaterExecuteInfo {
      updater: Arc::downgrade(&(updater.clone() as Arc<RwLock<dyn Updater>>)),
      order,
    }));
    self.need_sort = true;
  }
  pub fn own<T: Updater + 'static + Sized>(
    &mut self,
    updater: Arc<RwLock<T>>,
    order: Option<usize>,
  ) {
    self.owns.push(updater.clone());
    self.add(&updater, order);
  }
  pub fn execute(&mut self) {
    if self.need_sort {
      self.updaters.sort_by(|a, b| a.order.cmp(&b.order));
      self.need_sort = false;
    }
    self.updaters.retain(|u| {
      if let Some(updater) = u.updater.upgrade() {
        updater.write().unwrap().update();
        return true;
      } else {
        return false;
      }
    });
  }
}
