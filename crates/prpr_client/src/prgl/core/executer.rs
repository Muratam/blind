use super::*;

struct RenderPassExecuteInfo {
  pass: Weak<RwLock<RenderPass>>,
  priority: i32,
}

pub struct Executer {
  passes: Vec<RenderPassExecuteInfo>,
  is_dirty: bool,
}

impl Executer {
  pub fn new() -> Self {
    Self {
      passes: Vec::new(),
      is_dirty: false,
    }
  }
  pub fn add(&mut self, pass: &Arc<RwLock<RenderPass>>, priority: i32) {
    self.passes.push(RenderPassExecuteInfo {
      pass: Arc::downgrade(pass),
      priority,
    });
    self.is_dirty = true;
  }
  pub fn execute(&mut self) {
    let mut cmd = Command::new();
  }
}
