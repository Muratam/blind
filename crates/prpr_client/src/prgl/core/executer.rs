use super::*;

struct PipelineExecuteInfo {
  pipeline: WeakReader<Pipeline>,
  order: usize, // asc
}
pub struct PipelineExecuter {
  pipelines: Vec<PipelineExecuteInfo>,
  owns: Vec<Owner<Pipeline>>,
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
  pub fn add(&mut self, pipeline: &Reader<Pipeline>, order: usize) {
    self.pipelines.push(PipelineExecuteInfo {
      pipeline: pipeline.clone_weak_reader(),
      order,
    });
    self.need_sort = true;
  }
  pub fn own(&mut self, pipeline: Pipeline, order: usize) {
    let pipeline = Owner::new(pipeline);
    self.add(&pipeline.clone_reader(), order);
    self.owns.push(pipeline);
  }
  pub fn execute(&mut self, cmd: &mut Command, outer_ctx: &Arc<DescriptorContext>) {
    if self.need_sort {
      self.pipelines.sort_by(|a, b| a.order.cmp(&b.order));
      self.need_sort = false;
    }
    self.pipelines.retain(|p| {
      if let Some(pipeline) = p.pipeline.try_read() {
        pipeline.read().draw(cmd, outer_ctx);
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
  pass: WeakReader<RenderPass>,
  order: usize, // asc
}
pub struct RenderPassExecuter {
  passes: Vec<RenderPassExecuteInfo>,
  owns: Vec<Owner<RenderPass>>,
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
  pub fn add(&mut self, pass: &Reader<RenderPass>, order: usize) {
    self.passes.push(RenderPassExecuteInfo {
      pass: pass.clone_weak_reader(),
      order,
    });
    self.need_sort = true;
  }
  pub fn own(&mut self, pass: RenderPass, order: usize) {
    let pass = Owner::new(pass);
    self.add(&pass.clone_reader(), order);
    self.owns.push(pass);
  }
  pub fn execute(&mut self) {
    if self.need_sort {
      self.passes.sort_by(|a, b| a.order.cmp(&b.order));
      self.need_sort = false;
    }
    let mut cmd = prgl::Command::new();
    self.passes.retain(|p| {
      if let Some(pass) = p.pass.try_read() {
        pass.read().draw(&mut cmd, &DescriptorContext::nil());
        return true;
      } else {
        return false;
      }
    });
  }
}
