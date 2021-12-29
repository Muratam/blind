use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak};

// Owner しか書き込みができない
pub struct Owner<T: ?Sized> {
  data: Arc<RwLock<T>>,
}
// Reader は読み込みしかできない
pub struct Reader<T: ?Sized> {
  data: Arc<RwLock<T>>,
}
// WeakReaderは参照も持たない
pub struct WeakReader<T: ?Sized> {
  data: Weak<RwLock<T>>,
}
unsafe impl<T: ?Sized + Send> Send for Owner<T> {}
unsafe impl<T: ?Sized + Send + Sync> Sync for Owner<T> {}
unsafe impl<T: ?Sized + Send> Send for Reader<T> {}
unsafe impl<T: ?Sized + Send + Sync> Sync for Reader<T> {}
unsafe impl<T: ?Sized + Send> Send for WeakReader<T> {}
unsafe impl<T: ?Sized + Send + Sync> Sync for WeakReader<T> {}

impl<T> Owner<T> {
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
  pub fn read(&self) -> RwLockReadGuard<'_, T> {
    self.data.read().unwrap()
  }
  pub fn clone_reader(&self) -> Reader<T> {
    Reader::<T> {
      data: self.data.clone(),
    }
  }
  pub fn clone_weak_reader(&self) -> WeakReader<T> {
    WeakReader::<T> {
      data: Arc::downgrade(&self.data),
    }
  }
}
impl<T> WeakReader<T> {
  pub fn try_read(&self) -> Option<Reader<T>> {
    self.data.upgrade().map(|x| Reader { data: x })
  }
}

impl<T> Reader<T> {
  pub fn read(&self) -> RwLockReadGuard<'_, T> {
    self.data.read().unwrap()
  }
  pub fn clone_reader(&self) -> Reader<T> {
    Reader::<T> {
      data: self.data.clone(),
    }
  }
  pub fn clone_weak_reader(&self) -> WeakReader<T> {
    WeakReader::<T> {
      data: Arc::downgrade(&self.data),
    }
  }
}

impl<T> Clone for Reader<T> {
  fn clone(&self) -> Self {
    self.clone_reader()
  }
}
