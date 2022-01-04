use super::*;
pub trait NeedUpdate: downcast::Any {
  fn update(&mut self);
  fn is_destroyed(&self) -> bool {
    false
  }
}
impl<T: NeedUpdate> NeedUpdate for SOwner<T> {
  fn update(&mut self) {
    self.write().update()
  }
  fn is_destroyed(&self) -> bool {
    self.read().is_destroyed()
  }
}

downcast::downcast!(dyn NeedUpdate);

static INSTANCE: OnceCell<UpdaterImpl> = OnceCell::new();
unsafe impl Send for UpdaterImpl {}
unsafe impl Sync for UpdaterImpl {}

struct UpdaterSOwner {
  // 本当は SRwLock<Box<SOwner<Impl>>>,
  updater: SRwLock<Box<dyn NeedUpdate>>,
  order: Option<usize>, // asc
  type_id: std::any::TypeId,
}
pub struct UpdaterImpl {
  reserveds: SRwLock<Vec<UpdaterSOwner>>,
  updaters: SRwLock<Vec<UpdaterSOwner>>,
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
      reserveds: SRwLock::new(Vec::new()),
      updaters: SRwLock::new(Vec::new()),
    }
  }
  pub fn own<T: NeedUpdate + 'static>(&self, updater: T) {
    self.own_with_order(updater, None)
  }
  pub fn own_with_order<T: NeedUpdate + 'static>(&self, updater: T, order: Option<usize>) {
    // Update は次のフレームから実行される
    self.reserveds.write().push(UpdaterSOwner {
      updater: SRwLock::new(Box::new(SOwner::new(updater)) as Box<dyn NeedUpdate>),
      order,
      type_id: std::any::TypeId::of::<T>(),
    })
  }
  pub fn execute(&self) {
    {
      let mut updater_lock = self.updaters.write();
      let mut reserved_lock = self.reserveds.write();
      if reserved_lock.len() > 0 {
        while let Some(popped) = reserved_lock.pop() {
          updater_lock.push(popped);
        }
        updater_lock.sort_by(|a, b| a.order.cmp(&b.order));
      }
      updater_lock.retain(|u| !u.updater.read().is_destroyed());
    }
    for u in &mut self.updaters.read().iter() {
      u.updater.write().update();
    }
  }

  pub fn read_any<T: 'static>(&self) -> Option<SReader<T>> {
    let type_id = std::any::TypeId::of::<T>();
    for r in self.updaters.read().iter() {
      if r.type_id != type_id {
        continue;
      }
      // 更新中である自身の情報は撮れない
      // updater: SRwLock<Box<dyn NeedUpdate>>,
      // 本当は SRwLock<Box<SOwner<Impl>>,
      if let Some(r) = r.updater.try_read() {
        if let Ok(r) = r.downcast_ref::<SOwner<T>>() {
          return Some(r.clone_reader());
        }
      }
    }
    let reserveds = self.reserveds.read();
    for r in reserveds.iter() {
      if r.type_id != type_id {
        continue;
      }
      // 更新中である自身の情報は撮れない
      if let Some(r) = r.updater.try_read() {
        if let Ok(r) = r.downcast_ref::<SOwner<T>>() {
          return Some(r.clone_reader());
        }
      }
    }
    return None;
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
  pub fn read_any<T: 'static>() -> Option<SReader<T>> {
    UpdaterImpl::read_global().read_any()
  }
}
