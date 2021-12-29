use super::*;
pub trait Updatable {
  fn update(&mut self);
  fn is_dead(&self) -> bool {
    false
  }
}
use once_cell::sync::OnceCell;
static INSTANCE: OnceCell<RwLock<UpdaterImpl>> = OnceCell::new();
unsafe impl Send for UpdaterImpl {}
unsafe impl Sync for UpdaterImpl {}

struct UpdaterExecuteInfo {
  updater: Box<dyn Updatable>,
  order: Option<usize>, // asc
}

pub struct UpdaterImpl {
  updaters: Vec<UpdaterExecuteInfo>,
  need_sort: bool,
}
impl UpdaterImpl {
  pub fn initialize_global() {
    INSTANCE.set(RwLock::new(UpdaterImpl::new())).ok();
  }
  pub fn write_global() -> RwLockWriteGuard<'static, Self> {
    INSTANCE
      .get()
      .expect("Updater global not initialized")
      .write()
      .unwrap()
  }
  pub fn new() -> Self {
    Self {
      updaters: Vec::new(),
      need_sort: false,
    }
  }
  pub fn own<T: Updatable + 'static>(&mut self, updater: T) {
    self.own_with_order(updater, None)
  }
  pub fn own_with_order<T: Updatable + 'static>(&mut self, updater: T, order: Option<usize>) {
    self.updaters.push(UpdaterExecuteInfo {
      updater: Box::new(updater),
      order,
    });
    self.need_sort = true;
  }
  pub fn execute(&mut self) {
    if self.need_sort {
      self.updaters.sort_by(|a, b| a.order.cmp(&b.order));
      self.need_sort = false;
    }
    self.updaters.retain(|u| !u.updater.is_dead());
    for u in &mut self.updaters {
      u.updater.update();
    }
  }
}
pub struct Updater {}
impl Updater {
  pub fn own<T: Updatable + 'static>(updater: T) {
    UpdaterImpl::write_global().own(updater);
  }
  pub fn own_with_order<T: Updatable + 'static>(updater: T, order: Option<usize>) {
    UpdaterImpl::write_global().own_with_order(updater, order);
  }
}
