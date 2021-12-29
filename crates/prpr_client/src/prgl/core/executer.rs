use super::*;

struct PipelineExecuteInfo {
  pipeline: WeakReplica<Pipeline>,
  order: usize, // asc
}
pub struct PipelineExecuter {
  pipelines: Vec<PipelineExecuteInfo>,
  owns: Vec<Primary<Pipeline>>,
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
  pub fn add(&mut self, pipeline: &dyn ReplicaTrait<Pipeline>, order: usize) {
    self.pipelines.push(PipelineExecuteInfo {
      pipeline: pipeline.clone_weak_replica(),
      order,
    });
    self.need_sort = true;
  }
  pub fn own(&mut self, pipeline: Pipeline, order: usize) {
    let pipeline = Primary::new(pipeline);
    self.add(&pipeline, order);
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
static INSTANCE: OnceCell<RwLock<RenderPassExecuterImpl>> = OnceCell::new();
unsafe impl Send for RenderPassExecuterImpl {}
unsafe impl Sync for RenderPassExecuterImpl {}

struct RenderPassExecuteInfo {
  pass: WeakReplica<RenderPass>,
  order: usize, // asc
}
pub struct RenderPassExecuterImpl {
  passes: Vec<RenderPassExecuteInfo>,
  owns: Vec<Primary<RenderPass>>,
  need_sort: bool,
}
impl RenderPassExecuterImpl {
  pub fn initialize_global() {
    INSTANCE
      .set(RwLock::new(RenderPassExecuterImpl::new()))
      .ok();
  }
  pub fn write_global() -> RwLockWriteGuard<'static, Self> {
    INSTANCE
      .get()
      .expect("RenderPassExecuter global not initialized")
      .write()
      .unwrap()
  }
  pub fn new() -> Self {
    Self {
      passes: Vec::new(),
      owns: Vec::new(),
      need_sort: false,
    }
  }
  pub fn add(&mut self, pass: &dyn ReplicaTrait<RenderPass>, order: usize) {
    self.passes.push(RenderPassExecuteInfo {
      pass: pass.clone_weak_replica(),
      order,
    });
    self.need_sort = true;
  }
  pub fn own(&mut self, pass: RenderPass, order: usize) {
    let pass = Primary::new(pass);
    self.add(&pass, order);
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
pub struct RenderPassExecuter {}
impl RenderPassExecuter {
  pub fn add(pass: &dyn ReplicaTrait<RenderPass>, order: usize) {
    RenderPassExecuterImpl::write_global().add(pass, order);
  }
  pub fn own(pass: RenderPass, order: usize) {
    RenderPassExecuterImpl::write_global().own(pass, order);
  }
}
