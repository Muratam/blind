use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak};

// Primary しか書き込みができない
pub struct Primary<T> {
  data: Arc<RwLock<T>>,
}
// Replica は読み込みしかできない
pub struct Replica<T> {
  data: Arc<RwLock<T>>,
}
// WeakReaderは参照も持たない
pub struct WeakReplica<T> {
  data: Weak<RwLock<T>>,
}
pub trait ReplicaTrait<T> {
  fn read(&self) -> RwLockReadGuard<'_, T>;
  fn clone_replica(&self) -> Replica<T>;
  fn clone_weak_replica(&self) -> WeakReplica<T>;
}
unsafe impl<T: Send> Send for Primary<T> {}
unsafe impl<T: Send + Sync> Sync for Primary<T> {}
unsafe impl<T: Send> Send for Replica<T> {}
unsafe impl<T: Send + Sync> Sync for Replica<T> {}
unsafe impl<T: Send> Send for WeakReplica<T> {}
unsafe impl<T: Send + Sync> Sync for WeakReplica<T> {}

impl<T> Primary<T> {
  pub fn new(data: T) -> Self {
    Self {
      data: Arc::new(RwLock::new(data)),
    }
  }
  // ここを mut にすることで　Rust の借用チェッカを有効にする
  // デッドロックを防ぐことができる
  pub fn write(&mut self) -> RwLockWriteGuard<'_, T> {
    self.data.write().unwrap()
  }
}

impl<T> WeakReplica<T> {
  pub fn try_read(&self) -> Option<Replica<T>> {
    self.data.upgrade().map(|x| Replica { data: x })
  }
}

impl<T> ReplicaTrait<T> for Primary<T> {
  fn read(&self) -> RwLockReadGuard<'_, T> {
    self.data.read().unwrap()
  }
  fn clone_replica(&self) -> Replica<T> {
    Replica::<T> {
      data: self.data.clone(),
    }
  }
  fn clone_weak_replica(&self) -> WeakReplica<T> {
    WeakReplica::<T> {
      data: Arc::downgrade(&self.data),
    }
  }
}

impl<T> ReplicaTrait<T> for Replica<T> {
  fn read(&self) -> RwLockReadGuard<'_, T> {
    self.data.read().unwrap()
  }
  fn clone_replica(&self) -> Replica<T> {
    Replica::<T> {
      data: self.data.clone(),
    }
  }
  fn clone_weak_replica(&self) -> WeakReplica<T> {
    WeakReplica::<T> {
      data: Arc::downgrade(&self.data),
    }
  }
}

impl<T> Clone for Replica<T> {
  fn clone(&self) -> Self {
    self.clone_replica()
  }
}
