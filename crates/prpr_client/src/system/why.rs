// 複数の指定を受ける可能性があるもののための属性
const WHY_COUNT: usize = 5;
// 後ろの方が優先度が高いがちにしている
#[derive(Clone, Copy, PartialEq)]
pub enum Why {
  ByUser,      // ユーザー操作
  ByTrasition, // 別の状態に切り替わるので
  ByAnimation, // (恒久的に)アニメーションするので
  ByStyle,     // そういうスタイルなので
  ByOriginal,  // このオブジェクトを表現するために必要なので
}
pub trait WhyTrait
where
  Self: Sized,
{
  fn concat(&self, other: Self) -> Self;
}

pub struct Whys<T: WhyTrait + Clone + Copy> {
  whys: [Option<T>; WHY_COUNT],
}
impl<T: WhyTrait + Clone + Copy> Whys<T> {
  pub fn new() -> Self {
    Self {
      whys: [None; WHY_COUNT],
    }
  }
  pub fn set(&mut self, data: Option<T>, why: Why) {
    self.whys[why as usize] = data;
  }
  pub fn get(&self, why: Why) -> Option<T> {
    self.whys[why as usize]
  }
  pub fn calc(&self) -> Option<T> {
    let mut result: Option<T> = None;
    for why in self.whys {
      if let Some(a) = result {
        if let Some(b) = why {
          result = Some(a.concat(b));
        }
      } else {
        result = why;
      }
    }
    result
  }
}
