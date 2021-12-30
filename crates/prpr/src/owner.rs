use super::*;

// ArcOwner しか書き込みができない
pub struct ArcOwner<T> {
  data: Arc<RwLock<T>>,
}
// ArcReader は読み込みしかできない
pub struct ArcReader<T> {
  data: Arc<RwLock<T>>,
}
// WeakReaderは参照も持たない
pub struct ArcWeakReader<T> {
  data: Weak<RwLock<T>>,
}
pub trait ReplicaTrait<T> {
  fn read(&self) -> RwLockReadGuard<'_, T>;
  fn clone_reader(&self) -> ArcReader<T>;
  fn clone_weak_reader(&self) -> ArcWeakReader<T>;
}
unsafe impl<T: Send> Send for ArcOwner<T> {}
unsafe impl<T: Send + Sync> Sync for ArcOwner<T> {}
unsafe impl<T: Send> Send for ArcReader<T> {}
unsafe impl<T: Send + Sync> Sync for ArcReader<T> {}
unsafe impl<T: Send> Send for ArcWeakReader<T> {}
unsafe impl<T: Send + Sync> Sync for ArcWeakReader<T> {}

impl<T> ArcOwner<T> {
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

impl<T> ArcWeakReader<T> {
  pub fn try_read(&self) -> Option<ArcReader<T>> {
    self.data.upgrade().map(|x| ArcReader { data: x })
  }
}

impl<T> ReplicaTrait<T> for ArcOwner<T> {
  fn read(&self) -> RwLockReadGuard<'_, T> {
    self.data.read().unwrap()
  }
  fn clone_reader(&self) -> ArcReader<T> {
    ArcReader::<T> {
      data: self.data.clone(),
    }
  }
  fn clone_weak_reader(&self) -> ArcWeakReader<T> {
    ArcWeakReader::<T> {
      data: Arc::downgrade(&self.data),
    }
  }
}

impl<T> ReplicaTrait<T> for ArcReader<T> {
  fn read(&self) -> RwLockReadGuard<'_, T> {
    self.data.read().unwrap()
  }
  fn clone_reader(&self) -> ArcReader<T> {
    ArcReader::<T> {
      data: self.data.clone(),
    }
  }
  fn clone_weak_reader(&self) -> ArcWeakReader<T> {
    ArcWeakReader::<T> {
      data: Arc::downgrade(&self.data),
    }
  }
}

impl<T> Clone for ArcReader<T> {
  fn clone(&self) -> Self {
    self.clone_reader()
  }
}
