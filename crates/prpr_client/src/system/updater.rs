use super::*;
pub trait NeedUpdate {
  fn update(&mut self);
  fn is_destroyed(&self) -> bool {
    false
  }
}
static INSTANCE: OnceCell<UpdaterImpl> = OnceCell::new();
unsafe impl Send for UpdaterImpl {}
unsafe impl Sync for UpdaterImpl {}

struct UpdaterOwner {
  updater: ArcOwner<Box<dyn NeedUpdate>>,
  order: Option<usize>, // asc
}
pub struct UpdaterImpl {
  reserveds: Mutex<Vec<UpdaterOwner>>,
  updaters: Mutex<Vec<UpdaterOwner>>,
  readers: RwLock<Vec<ArcWeakReader<Box<dyn NeedUpdate>>>>,
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
      reserveds: Mutex::new(Vec::new()),
      updaters: Mutex::new(Vec::new()),
      readers: RwLock::new(Vec::new()),
    }
  }
  pub fn own<T: NeedUpdate + 'static>(&self, updater: T) {
    self.own_with_order(updater, None)
  }
  pub fn own_with_order<T: NeedUpdate + 'static>(&self, updater: T, order: Option<usize>) {
    // Update は次のフレームから実行される
    self.reserveds.lock().unwrap().push(UpdaterOwner {
      updater: ArcOwner::new(Box::new(updater)),
      order,
    });
  }
  pub fn execute(&self) {
    let mut updater_lock = self.updaters.lock().unwrap();
    {
      let mut reader_lock = self.readers.write().unwrap();
      {
        let mut reserved_lock = self.reserveds.lock().unwrap();
        if reserved_lock.len() > 0 {
          while let Some(popped) = reserved_lock.pop() {
            reader_lock.push(popped.updater.clone_weak_reader());
            updater_lock.push(popped);
          }
          updater_lock.sort_by(|a, b| a.order.cmp(&b.order));
        }
      }
      updater_lock.retain(|u| !u.updater.read().is_destroyed());
      reader_lock.retain(|u| u.try_read().is_some());
    }
    for u in &mut updater_lock.iter_mut() {
      u.updater.write().update();
    }
  }
  pub fn find_any_in_whole<T: 'static>(&self) -> Option<ArcReader<Box<T>>> {
    fn type_id_inner(type_id: &std::any::TypeId) -> u64 {
      let ptr = type_id as *const std::any::TypeId as *const u64;
      unsafe { *ptr }
    }
    log::info(format!(
      "{}\n{}\n{}",
      type_id_inner(&std::any::TypeId::of::<ArcWeakReader<Box<dyn NeedUpdate>>>(),),
      type_id_inner(&std::any::TypeId::of::<Box<dyn NeedUpdate>>(),),
      type_id_inner(&std::any::TypeId::of::<T>(),)
    ));
    let mut ids = String::from("");
    for u in self.readers.read().unwrap().iter() {
      let any: &dyn std::any::Any = u;
      ids += &format!("{}\n", type_id_inner(&any.type_id()));
      if any.is::<ArcWeakReader<Box<T>>>() {
        if let Some(weak) = any.downcast_ref::<ArcWeakReader<Box<T>>>() {
          return weak.try_read();
        } else {
          return None;
        }
      }
    }
    log::info(&ids);
    None
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
  pub fn find_any_in_whole<T: 'static>() -> Option<ArcReader<Box<T>>> {
    UpdaterImpl::read_global().find_any_in_whole()
  }
}
