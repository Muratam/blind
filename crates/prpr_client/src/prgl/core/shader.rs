use super::*;

use std::collections::HashMap;
pub struct Shader {
  template: ShaderTemplate,
  uniform_block_indices: Vec<String>,
  uniform_texture_locations: Vec<(String, UniformTextureLocation)>,
  raw_program: RawShaderProgram,
}

impl Shader {
  pub fn new(template: ShaderTemplate) -> Option<Self> {
    if let Some(raw_program) = RawShaderProgram::new(&template) {
      let ctx = Instance::ctx();
      let mut max_uniform_block_indices: usize = 0;
      let mut map_uniform_block_indices: HashMap<String, usize> = HashMap::new();
      for name in template.uniform_blocks() {
        let u_index = ctx.get_uniform_block_index(raw_program.raw_program(), name);
        ctx.uniform_block_binding(raw_program.raw_program(), u_index, u_index);
        map_uniform_block_indices.insert(String::from(*name), u_index as usize);
        max_uniform_block_indices = max_uniform_block_indices.max(u_index as usize);
      }
      let mut uniform_block_indices = vec![String::from(""); max_uniform_block_indices + 1];
      for (k, v) in map_uniform_block_indices {
        uniform_block_indices[v] = k;
      }

      let mut map_uniform_texture_locations: HashMap<String, UniformTextureLocation> =
        HashMap::new();
      let template_uniform_textures = template.uniform_textures();
      for i in 0..template_uniform_textures.len() {
        let name = template_uniform_textures[i];
        let location = ctx.get_uniform_location(raw_program.raw_program(), name);
        if let Some(location) = location {
          map_uniform_texture_locations.insert(String::from(name), (location, i as i32));
        }
      }
      let mut uniform_texture_locations = Vec::new();
      for data in map_uniform_texture_locations {
        uniform_texture_locations.push(data);
      }
      Some(Self {
        uniform_block_indices,
        uniform_texture_locations,
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
    self.raw_program.program_id()
  }
  pub fn use_program(&self) {
    self.raw_program.use_program();
  }
  pub fn raw_program(&self) -> &RawShaderProgram {
    &self.raw_program
  }
  // 数がたいてい少ないのでHashMap使うほうが遅い
  pub fn uniform_block_index(&self, name: &str) -> Option<u32> {
    for (i, v) in self.uniform_block_indices.iter().enumerate() {
      if v == name {
        return Some(i as u32);
      }
    }
    None
  }
  // 数がたいてい少ないのでHashMap使うほうが遅い
  pub fn uniform_texture_location(&self, name: &str) -> Option<&UniformTextureLocation> {
    for data in &self.uniform_texture_locations {
      if data.0 == name {
        return Some(&data.1);
      }
    }
    None
  }
}
impl std::fmt::Display for Shader {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}{}", self.template.vs_code(), self.template.fs_code())
  }
}

pub struct MayShader {
  shader: Option<SRc<Shader>>,
}
impl MayShader {
  pub fn new(template: ShaderTemplate) -> Self {
    let shader = Shader::new(template).map(|x| SRc::new(x));
    Self { shader }
  }
}
impl std::fmt::Display for MayShader {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if let Some(shader) = &self.shader {
      write!(f, "{}", shader)
    } else {
      write!(f, "No Shader!")
    }
  }
}
impl PipelineBindable for MayShader {
  fn bind_pipeline(&self, pipeline: &mut Pipeline) {
    if let Some(shader) = &self.shader {
      pipeline.set_shader(shader);
    }
  }
}
