use super::*;
// 後ろの方が優先度が高いがちにしている
#[derive(Clone, Copy, PartialEq)]
pub enum Why {
  ByUser,        // ユーザー操作
  ByTrasition,   // 別の状態に切り替わるので
  ByAnimation,   // (恒久的に)アニメーションするので
  ByCustomStyle, // ここではそういうスタイルなので
  ByStyle,       // そういうスタイルなので
  ByOriginal,    // このオブジェクトを表現するために必要なので
}
pub trait WhyTrait
where
  Self: Sized,
{
  fn concat(&self, other: &Self) -> Self;
}

pub struct Whys<T: WhyTrait + Clone> {
  whys: [Option<T>; 6],
  calculated: Option<T>,
}
impl<T: WhyTrait + Clone> Whys<T> {
  pub fn new() -> Self {
    Self {
      whys: [None, None, None, None, None, None],
      calculated: None,
    }
  }
  pub fn set(&mut self, data: Option<T>, why: Why) {
    self.whys[why as usize] = data;
    let mut result: Option<T> = None;
    for why in &self.whys {
      if let Some(a) = &result {
        if let Some(b) = why {
          result = Some(a.concat(b));
        }
      } else {
        result = why.clone();
      }
    }
    self.calculated = result;
  }
  pub fn get(&self, why: Why) -> Option<T> {
    if let Some(got) = &self.whys[why as usize] {
      Some(got.clone())
    } else {
      None
    }
  }
  pub fn calc(&self) -> Option<T> {
    if let Some(calc) = &self.calculated {
      Some(calc.clone())
    } else {
      None
    }
  }
}
impl<T: WhyTrait + Clone + Default> Whys<T> {
  pub fn calc_or_default(&self) -> T {
    self.calc().unwrap_or_default()
  }
}
