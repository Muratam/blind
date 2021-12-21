use super::*;

pub struct Shader {
  ctx: ArcGlContext,
  template: ShaderTemplate,
  raw_program: RawShaderProgram,
}
impl Shader {
  pub fn new(ctx: &ArcGlContext, template: ShaderTemplate) -> Option<Self> {
    if let Some(raw_program) = RawShaderProgram::new(ctx, &template) {
      Some(Self {
        ctx: ctx.clone(),
        template,
        raw_program,
      })
    } else {
      None
    }
  }
  pub fn id(&self) -> u64 {
    self.raw_program.raw_program_id()
  }
  pub fn use_program(&self) {
    self.raw_program.use_program();
  }
  pub fn raw_program(&self) -> &RawShaderProgram {
    &self.raw_program
  }
  pub fn get_uniform_block_index(&self, name: &str) -> Option<u32> {
    // 高速にできるかも
    let u_index = self
      .ctx
      .get_uniform_block_index(&self.raw_program.raw_program(), name);
    if u_index != gl::INVALID_INDEX {
      return Some(u_index);
    } else {
      return None;
    }
  }
}
