use super::*;

const MAX_UNIFORM_BUFFER_BINDINGS: usize = 64;
const MAX_UNIFORM_TEXTURE_BINDINGS: usize = 64;

pub struct Command {
  depth_func: Option<DepthFunc>,
  cull_mode: Option<CullMode>,
  shader: Option<Arc<Shader>>,
  vao: Option<u64>,
  // NOTE: この２つは同じものを取らない...はず
  uniform_buffers: [Option<u64>; MAX_UNIFORM_BUFFER_BINDINGS],
  uniform_textures: [Option<u64>; MAX_UNIFORM_TEXTURE_BINDINGS],
}

impl Command {
  pub fn new() -> Self {
    Self {
      depth_func: None,
      cull_mode: None,
      shader: None,
      vao: None,
      uniform_buffers: [None; MAX_UNIFORM_BUFFER_BINDINGS],
      uniform_textures: [None; MAX_UNIFORM_TEXTURE_BINDINGS],
    }
  }
  pub fn set_depth_func(&mut self, v: DepthFunc) {
    if let Some(pre) = self.depth_func {
      if pre == v {
        return;
      }
    }
    v.apply();
    self.depth_func = Some(v);
  }
  pub fn set_cull_mode(&mut self, v: CullMode) {
    if let Some(pre) = self.cull_mode {
      if pre == v {
        return;
      }
    }
    v.apply();
    self.cull_mode = Some(v);
  }
  pub fn set_draw_command(&mut self, v: &DrawCommand, t: PrimitiveToporogy) {
    v.apply(t);
  }
  pub fn set_shader(&mut self, v: &Arc<Shader>) {
    if let Some(pre) = &self.shader {
      if pre.id() == v.id() {
        return;
      }
    }
    v.use_program();
    self.shader = Some(v.clone());
  }
  pub fn current_shader(&self) -> &Option<Arc<Shader>> {
    &self.shader
  }
  pub fn set_vao(&mut self, vao: &RawVao) {
    if let Some(pre) = self.vao {
      if pre == vao.vao_id() {
        return;
      }
    }
    let ctx = Instance::ctx();
    ctx.bind_vertex_array(Some(vao.raw_vao()));
    self.vao = Some(vao.vao_id());
  }
  pub fn set_ubo(&mut self, ubo: &RawBuffer, index: u32) {
    if index as usize >= self.uniform_buffers.len() {
      log::error("uniform buffer length exceeded");
    }
    if let Some(pre) = self.uniform_buffers[index as usize] {
      if pre == ubo.buffer_id() {
        return;
      }
    }
    let ctx = Instance::ctx();
    ctx.bind_buffer_base(gl::UNIFORM_BUFFER, index, Some(ubo.raw_buffer()));
    self.uniform_buffers[index as usize] = Some(ubo.buffer_id());
  }
  pub fn set_uniform_texture(&mut self, texture: &RawTexture, utl: &UniformTextureLocation) {
    let (location, index) = utl;
    let index = *index as usize;
    if index >= self.uniform_textures.len() {
      log::error("texture binding index exceeded");
    }
    let ctx = Instance::ctx();
    if let Some(pre) = self.uniform_textures[index] {
      if pre == texture.texture_id() {
        return;
      }
    } else {
      ctx.active_texture(RawTexture::to_slot_enum(index as i32));
    }
    texture.bind();
    // NOTE: location := WebGlUniformLocation は怪しいかも
    ctx.uniform1i(Some(location), index as i32);
    self.uniform_textures[index as usize] = Some(texture.texture_id());
  }
}
