pub struct BitSet64 {
  data: u64,
}
impl BitSet64 {
  pub fn new() -> Self {
    Self { data: 0 }
  }
  pub fn set(&mut self, index: usize, v: bool) {
    let bit = (1 as u64) << (index as u64);
    if v {
      self.data |= bit;
    } else {
      self.data &= !bit;
    }
  }
  pub fn get(&self, index: usize) -> bool {
    let bit = (1 as u64) << (index as u64);
    (self.data & bit) > 0
  }
  pub fn any(&self) -> bool {
    self.data != 0
  }
  pub fn set_all_false(&mut self) {
    self.data = 0;
  }
  pub fn set_all_true(&mut self) {
    self.data = !0;
  }
}
