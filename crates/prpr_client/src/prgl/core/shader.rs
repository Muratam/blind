use super::*;

pub struct Shader {
  template: ShaderTemplate,
  raw_program: RawShaderProgram,
}
impl Shader {
  pub fn new(gl: &ArcGlContext, template: ShaderTemplate) -> Option<Self> {
    if let Some(raw_program) = RawShaderProgram::new(gl, &template) {
      Some(Self {
        template,
        raw_program,
      })
    } else {
      None
    }
  }
  pub fn use_program(&self) {
    self.raw_program.use_program();
  }
  pub fn raw_program(&self) -> &RawShaderProgram {
    &self.raw_program
  }
}
