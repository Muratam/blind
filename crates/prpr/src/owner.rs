// S: Single Thread の S
pub type SRc<T> = std::rc::Rc<T>;
pub type SWeak<T> = std::rc::Weak<T>;
pub type SRwLock<T> = std::cell::RefCell<T>;
pub type SDerefable<'a, T> = std::cell::Ref<'a, T>;
pub type SDerefMutable<'a, T> = std::cell::RefMut<'a, T>;
// M: Multi Thread の M
pub type MRc<T> = std::sync::Arc<T>;
pub type MWeak<T> = std::sync::Weak<T>;
pub type MRwLock<T> = (std::sync::RwLock<T>,);
pub type MDerefable<'a, T> = std::sync::RwLockReadGuard<'a, T>;
pub type MDerefMutable<'a, T> = std::sync::RwLockWriteGuard<'a, T>;

// 共通化用コード
pub trait ModOwnerSReadWrite<T> {
  fn read(&self) -> SDerefable<'_, T>;
  fn write(&self) -> SDerefMutable<'_, T>;
  fn try_read(&self) -> Option<SDerefable<'_, T>>;
  fn try_write(&self) -> Option<SDerefMutable<'_, T>>;
}
pub trait ModOwnerMReadWrite<T> {
  fn new(data: T) -> Self;
  fn read(&self) -> MDerefable<'_, T>;
  fn write(&self) -> MDerefMutable<'_, T>;
  fn try_read(&self) -> Option<MDerefable<'_, T>>;
  fn try_write(&self) -> Option<MDerefMutable<'_, T>>;
}
impl<T> ModOwnerSReadWrite<T> for SRwLock<T> {
  fn read(&self) -> SDerefable<'_, T> {
    self.borrow()
  }
  fn write(&self) -> SDerefMutable<'_, T> {
    self.borrow_mut()
  }
  fn try_read(&self) -> Option<SDerefable<'_, T>> {
    self.try_borrow().ok()
  }
  fn try_write(&self) -> Option<SDerefMutable<'_, T>> {
    self.try_borrow_mut().ok()
  }
}
impl<T> ModOwnerMReadWrite<T> for MRwLock<T> {
  fn new(data: T) -> Self {
    (std::sync::RwLock::new(data),)
  }
  fn read(&self) -> MDerefable<'_, T> {
    self.0.read().unwrap()
  }
  fn write(&self) -> MDerefMutable<'_, T> {
    self.0.write().unwrap()
  }
  fn try_read(&self) -> Option<MDerefable<'_, T>> {
    self.0.try_read().ok()
  }
  fn try_write(&self) -> Option<MDerefMutable<'_, T>> {
    self.0.try_write().ok()
  }
}

pub trait ModOwnerDowngrade<T> {
  fn downgrade(&self) -> T;
}
impl<T> ModOwnerDowngrade<SWeak<T>> for SRc<T> {
  fn downgrade(&self) -> SWeak<T> {
    SRc::downgrade(self)
  }
}
impl<T> ModOwnerDowngrade<MWeak<T>> for MRc<T> {
  fn downgrade(&self) -> MWeak<T> {
    MRc::downgrade(self)
  }
}

// Owner - Reader - WeakReader
pub struct SOwner<T> {
  data: SRc<SRwLock<T>>,
}
// SReader は読み込みしかできない
pub struct SReader<T> {
  data: SRc<SRwLock<T>>,
}
// WeakReaderは参照も持たない
pub struct SWeakReader<T> {
  data: SWeak<SRwLock<T>>,
}
pub trait SReaderTrait<T> {
  fn read(&self) -> SDerefable<T>;
  fn clone_reader(&self) -> SReader<T>;
  fn clone_weak_reader(&self) -> SWeakReader<T>;
}

impl<T> SOwner<T> {
  pub fn new(data: T) -> Self {
    Self {
      data: SRc::new(SRwLock::new(data)),
    }
  }
  // ここを mut にすることで　Rust の借用チェッカを有効にする
  // デッドロックを防ぐことができる
  pub fn write(&mut self) -> SDerefMutable<'_, T> {
    self.data.write()
  }
}

impl<T> SWeakReader<T> {
  pub fn try_read(&self) -> Option<SReader<T>> {
    self.data.upgrade().map(|x| SReader { data: x })
  }
}

impl<T> SReaderTrait<T> for SOwner<T> {
  fn read(&self) -> SDerefable<'_, T> {
    self.data.read()
  }
  fn clone_reader(&self) -> SReader<T> {
    SReader::<T> {
      data: self.data.clone(),
    }
  }
  fn clone_weak_reader(&self) -> SWeakReader<T> {
    SWeakReader::<T> {
      data: self.data.downgrade(),
    }
  }
}

impl<T> SReaderTrait<T> for SReader<T> {
  fn read(&self) -> SDerefable<'_, T> {
    self.data.read()
  }
  fn clone_reader(&self) -> SReader<T> {
    SReader::<T> {
      data: self.data.clone(),
    }
  }
  fn clone_weak_reader(&self) -> SWeakReader<T> {
    SWeakReader::<T> {
      data: self.data.downgrade(),
    }
  }
}

impl<T> Clone for SReader<T> {
  fn clone(&self) -> Self {
    self.clone_reader()
  }
}

impl<T: Default> Default for SOwner<T> {
  fn default() -> Self {
    Self::new(Default::default())
  }
}

// not implemented
// pub struct MOwner<T> {
//   data: MRc<MRwLock<T>>,
// }
// pub struct MReader<T> {
//   data: MRc<MRwLock<T>>,
// }
// pub struct MWeakReader<T> {
//   data: MWeak<MRwLock<T>>,
// }
// pub trait MReaderTrait<T> {
//   fn read(&self) -> MDerefable<T>;
//   fn clone_reader(&self) -> MReader<T>;
//   fn clone_weak_reader(&self) -> MWeakReader<T>;
// }
// unsafe impl<T: Send> Send for MOwner<T> {}
// unsafe impl<T: Send + Sync> Sync for MOwner<T> {}
// unsafe impl<T: Send> Send for MReader<T> {}
// unsafe impl<T: Send + Sync> Sync for MReader<T> {}
// unsafe impl<T: Send> Send for MWeakReader<T> {}
// unsafe impl<T: Send + Sync> Sync for MWeakReader<T> {}
