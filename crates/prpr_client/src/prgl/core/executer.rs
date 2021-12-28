use super::*;

struct PipelineExecuteInfo {
  pipeline: Weak<RwLock<Pipeline>>,
  priority: usize, // asc
}
pub struct PipelineExecuter {
  pipelines: Vec<Arc<PipelineExecuteInfo>>,
  owneds: Vec<Arc<RwLock<Pipeline>>>,
  need_sort: bool,
}

impl PipelineExecuter {
  pub fn new() -> Self {
    Self {
      pipelines: Vec::new(),
      need_sort: false,
      owneds: Vec::new(),
    }
  }
  pub fn add(&mut self, pipeline: &Arc<RwLock<Pipeline>>, priority: usize) {
    self.pipelines.push(Arc::new(PipelineExecuteInfo {
      pipeline: Arc::downgrade(pipeline),
      priority,
    }));
    self.need_sort = true;
  }
  pub fn own(&mut self, pipeline: Pipeline, priority: usize) {
    let pipeline = Arc::new(RwLock::new(pipeline));
    self.owneds.push(pipeline.clone());
    self.add(&pipeline, priority);
  }
  pub fn execute_draw(&mut self, cmd: &mut Command, outer_ctx: &Arc<DescriptorContext>) {
    if self.need_sort {
      self.pipelines.sort_by(|a, b| a.priority.cmp(&b.priority));
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
