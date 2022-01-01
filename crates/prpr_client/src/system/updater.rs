use super::*;
pub trait NeedUpdate: downcast::Any {
  fn update(&mut self);
  fn is_destroyed(&self) -> bool {
    false
  }
}
downcast::downcast!(dyn NeedUpdate);

static INSTANCE: OnceCell<UpdaterImpl> = OnceCell::new();
unsafe impl Send for UpdaterImpl {}
unsafe impl Sync for UpdaterImpl {}

struct UpdaterOwner {
  updater: RwLock<Box<dyn NeedUpdate>>,
  order: Option<usize>, // asc
  type_id: std::any::TypeId,
}
pub struct UpdaterImpl {
  reserveds: RwLock<Vec<UpdaterOwner>>,
  updaters: RwLock<Vec<UpdaterOwner>>,
}

impl UpdaterImpl {
  pub fn initialize_global() {
    INSTANCE.set(UpdaterImpl::new()).ok();
  }
  pub fn read_global() -> &'static Self {
    INSTANCE.get().expect("Updater global not initialized")
  }
  pub fn new() -> Self {
    Self {
      reserveds: RwLock::new(Vec::new()),
      updaters: RwLock::new(Vec::new()),
    }
  }
  pub fn own<T: NeedUpdate + 'static>(&self, updater: T) {
    self.own_with_order(updater, None)
  }
  pub fn own_with_order<T: NeedUpdate + 'static>(&self, updater: T, order: Option<usize>) {
    // Update は次のフレームから実行される
    if let Ok(write) = &mut self.reserveds.write() {
      write.push(UpdaterOwner {
        updater: RwLock::new(Box::new(updater)),
        order,
        type_id: std::any::TypeId::of::<T>(),
      });
    } else {
      log::error("Updater failed to own... ignored!!");
    }
  }
  pub fn execute(&self) {
    {
      let mut updater_lock = self.updaters.write().unwrap();
      let mut reserved_lock = self.reserveds.write().unwrap();
      if reserved_lock.len() > 0 {
        while let Some(popped) = reserved_lock.pop() {
          updater_lock.push(popped);
        }
        updater_lock.sort_by(|a, b| a.order.cmp(&b.order));
      }
      updater_lock.retain(|u| !u.updater.read().unwrap().is_destroyed());
    }
    for u in &mut self.updaters.read().unwrap().iter() {
      u.updater.write().unwrap().update();
    }
  }
  pub fn read_impl<T: 'static, F>(&self, mut f: F, is_any: bool)
  where
    F: FnMut(&T),
  {
    let type_id = std::any::TypeId::of::<T>();
    if let Ok(updaters) = self.updaters.read() {
      for r in updaters.iter() {
        if r.type_id != type_id {
          continue;
        }
        // 更新中である自身の情報は撮れない
        if let Ok(r) = r.updater.try_read() {
          if let Ok(r) = r.downcast_ref::<T>() {
            f(r);
            if is_any {
              return;
            }
          }
        }
      }
    } else {
      log::error("failed to read updaters (recursive read)");
    }
    if let Ok(reserveds) = self.reserveds.read() {
      for r in reserveds.iter() {
        if r.type_id != type_id {
          continue;
        }
        // 更新中である自身の情報は撮れない
        if let Ok(r) = r.updater.try_read() {
          if let Ok(r) = r.downcast_ref::<T>() {
            f(r);
            if is_any {
              return;
            }
          }
        }
      }
    } else {
      log::error("failed to read reserveds (recursive read)");
    }
  }
}

pub struct Updater {}
impl Updater {
  pub fn own<T: NeedUpdate + 'static>(updater: T) {
    UpdaterImpl::read_global().own(updater);
  }
  pub fn own_with_order<T: NeedUpdate + 'static>(updater: T, order: Option<usize>) {
    UpdaterImpl::read_global().own_with_order(updater, order);
  }
  pub fn read_all<T: 'static, F>(f: F)
  where
    F: FnMut(&T),
  {
    UpdaterImpl::read_global().read_impl(f, false);
  }
  pub fn read_any<T: 'static, F>(f: F)
  where
    F: FnMut(&T),
  {
    UpdaterImpl::read_global().read_impl(f, true);
  }
}
