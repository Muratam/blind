use super::*;

struct PipelineExecuteInfo {
  pipeline: Weak<RwLock<Pipeline>>,
  order: usize, // asc
}
pub struct PipelineExecuter {
  pipelines: Vec<Arc<PipelineExecuteInfo>>,
  owns: Vec<Arc<RwLock<Pipeline>>>,
  need_sort: bool,
}

impl PipelineExecuter {
  pub fn new() -> Self {
    Self {
      pipelines: Vec::new(),
      need_sort: false,
      owns: Vec::new(),
    }
  }
  pub fn add(&mut self, pipeline: &Arc<RwLock<Pipeline>>, order: usize) {
    self.pipelines.push(Arc::new(PipelineExecuteInfo {
      pipeline: Arc::downgrade(pipeline),
      order,
    }));
    self.need_sort = true;
  }
  pub fn own(&mut self, pipeline: Pipeline, order: usize) {
    let pipeline = Arc::new(RwLock::new(pipeline));
    self.owns.push(pipeline.clone());
    self.add(&pipeline, order);
  }
  pub fn execute(&mut self, cmd: &mut Command, outer_ctx: &Arc<DescriptorContext>) {
    if self.need_sort {
      self.pipelines.sort_by(|a, b| a.order.cmp(&b.order));
      self.need_sort = false;
    }
    self.pipelines.retain(|p| {
      if let Some(pipeline) = p.pipeline.upgrade() {
        pipeline.read().unwrap().draw(cmd, outer_ctx);
        return true;
      } else {
        return false;
      }
    });
  }
}

// WARN: 多分別スレッドから実行できない
use once_cell::sync::OnceCell;
static INSTANCE: OnceCell<RwLock<RenderPassExecuter>> = OnceCell::new();
unsafe impl Send for RenderPassExecuter {}
unsafe impl Sync for RenderPassExecuter {}

struct RenderPassExecuteInfo {
  pass: Weak<RwLock<RenderPass>>,
  order: usize, // asc
}
pub struct RenderPassExecuter {
  passes: Vec<Arc<RenderPassExecuteInfo>>,
  owns: Vec<Arc<RwLock<RenderPass>>>,
  need_sort: bool,
}
impl RenderPassExecuter {
  pub fn global_read_lock() -> RwLockReadGuard<'static, Self> {
    INSTANCE
      .get()
      .expect("RenderPassExecuter global not initialized")
      .read()
      .unwrap()
  }
  pub fn global_write_lock() -> RwLockWriteGuard<'static, Self> {
    INSTANCE
      .get()
      .expect("RenderPassExecuter global not initialized")
      .write()
      .unwrap()
  }
  pub fn global_initialize() {
    INSTANCE.set(RwLock::new(RenderPassExecuter::new())).ok();
  }

  pub fn new() -> Self {
    Self {
      passes: Vec::new(),
      owns: Vec::new(),
      need_sort: false,
    }
  }
  pub fn add(&mut self, pass: &Arc<RwLock<RenderPass>>, order: usize) {
    self.passes.push(Arc::new(RenderPassExecuteInfo {
      pass: Arc::downgrade(pass),
      order,
    }));
    self.need_sort = true;
  }
  pub fn own(&mut self, pass: RenderPass, order: usize) {
    let pass = Arc::new(RwLock::new(pass));
    self.owns.push(pass.clone());
    self.add(&pass, order);
  }
  pub fn execute(&mut self) {
    if self.need_sort {
      self.passes.sort_by(|a, b| a.order.cmp(&b.order));
      self.need_sort = false;
    }
    let mut cmd = prgl::Command::new();
    self.passes.retain(|p| {
      if let Some(pass) = p.pass.upgrade() {
        pass
          .write()
          .unwrap()
          .draw(&mut cmd, &DescriptorContext::nil());
        return true;
      } else {
        return false;
      }
    });
  }
}
