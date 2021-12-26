use super::*;

use std::collections::HashMap;
pub struct Shader {
  ctx: ArcGlContext,
  template: ShaderTemplate,
  uniform_block_indices: HashMap<String, u32>,
  uniform_texture_locations: HashMap<String, UniformTextureLocation>,
  raw_program: RawShaderProgram,
}
impl Shader {
  pub fn new(ctx: &ArcGlContext, template: ShaderTemplate) -> Option<Self> {
    if let Some(raw_program) = RawShaderProgram::new(ctx, &template) {
      let mut uniform_block_indices: HashMap<String, u32> = HashMap::new();
      for name in template.uniform_blocks() {
        let u_index = ctx.get_uniform_block_index(raw_program.raw_program(), name);
        ctx.uniform_block_binding(raw_program.raw_program(), u_index, u_index);
        uniform_block_indices.insert(String::from(*name), u_index);
      }
      let mut uniform_texture_locations: HashMap<String, UniformTextureLocation> = HashMap::new();
      let template_uniform_textures = template.uniform_textures();
      for i in 0..template_uniform_textures.len() {
        let name = template_uniform_textures[i];
        let location = ctx.get_uniform_location(raw_program.raw_program(), name);
        if let Some(location) = location {
          uniform_texture_locations.insert(String::from(name), (location, i as i32));
        }
      }
      Some(Self {
        uniform_block_indices,
        uniform_texture_locations,
        ctx: ctx.clone(),
        template,
        raw_program,
      })
    } else {
      None
    }
  }
  pub fn vs_code(&self) -> String {
    self.template.vs_code()
  }
  pub fn fs_code(&self) -> String {
    self.template.fs_code()
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
  pub fn uniform_block_index(&self, name: &str) -> Option<u32> {
    self.uniform_block_indices.get(name).map(|x| *x)
  }
  pub fn uniform_texture_location(&self, name: &str) -> Option<&UniformTextureLocation> {
    self.uniform_texture_locations.get(name)
  }
}
