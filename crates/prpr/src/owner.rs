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

impl<T> Owner<T> {
  pub fn new(data: T) -> Self {
    Self {
      data: Arc::new(RwLock::new(data)),
    }
  }
  pub fn read(&self) -> RwLockReadGuard<'_, T> {
    self.data.read().unwrap()
  }
  // ここを mut にすることで　Rust の借用チェッカを有効にする
  // デッドロックを防ぐことができる
  pub fn write(&mut self) -> RwLockWriteGuard<'_, T> {
    self.data.write().unwrap()
  }
}
impl<T> Reader<T> {
  pub fn read(&self) -> RwLockReadGuard<'_, T> {
    self.data.read().unwrap()
  }
}
impl<T> WeakReader<T> {
  pub fn try_read(&self) -> Option<Reader<T>> {
    self.data.upgrade().map(|x| Reader { data: x })
  }
}

pub trait ReaderClonable<T: ?Sized> {
  fn clone_reader(&self) -> Reader<T>;
  fn clone_weak_reader(&self) -> WeakReader<T>;
}
impl<T> ReaderClonable<T> for Owner<T> {
  fn clone_reader(&self) -> Reader<T> {
    Reader::<T> {
      data: self.data.clone(),
    }
  }
  fn clone_weak_reader(&self) -> WeakReader<T> {
    WeakReader::<T> {
      data: Arc::downgrade(&self.data),
    }
  }
}
impl<T> ReaderClonable<T> for Reader<T> {
  fn clone_reader(&self) -> Reader<T> {
    Reader::<T> {
      data: self.data.clone(),
    }
  }
  fn clone_weak_reader(&self) -> WeakReader<T> {
    WeakReader::<T> {
      data: Arc::downgrade(&self.data),
    }
  }
}
