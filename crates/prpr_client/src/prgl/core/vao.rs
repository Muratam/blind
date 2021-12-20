use super::*;
use std::collections::HashMap;
pub struct Vao<T: BufferAttribute> {
  ctx: ArcGlContext,
  v_buffer: VertexBuffer<T>,
  i_buffer: Option<IndexBuffer>,
  shader_id_to_raw_vao: Mutex<HashMap<u64, RawVao>>,
}
pub trait VaoTrait {
  // returns successed
  fn bind(&self, program: &RawShaderProgram);
}
impl<T: BufferAttribute> Vao<T> {
  pub fn new(ctx: &ArcGlContext, v_buffer: VertexBuffer<T>, i_buffer: IndexBuffer) -> Self {
    Self {
      ctx: ctx.clone(),
      v_buffer,
      i_buffer: Some(i_buffer),
      shader_id_to_raw_vao: Mutex::new(HashMap::new()),
    }
  }
  pub fn new_without_index_buffer(ctx: &ArcGlContext, v_buffer: VertexBuffer<T>) -> Self {
    Self {
      ctx: ctx.clone(),
      v_buffer,
      i_buffer: None,
      shader_id_to_raw_vao: Mutex::new(HashMap::new()),
    }
  }
  pub fn draw_command(&self) -> DrawCommand {
    if let Some(i_buffer) = &self.i_buffer {
      DrawCommand::DrawIndexed {
        first: 0,
        count: i_buffer.len() as i32,
      }
    } else {
      DrawCommand::Draw {
        first: 0,
        count: self.v_buffer.len() as i32,
      }
    }
  }
  // pub fn draw_instanced_command() -> DrawCommand {}
}
impl<T: BufferAttribute> VaoTrait for Vao<T> {
  fn bind(&self, program: &RawShaderProgram) {
    let id = program.raw_program_id();
    let mut lock = self.shader_id_to_raw_vao.lock().unwrap();
    if let Some(raw_vao) = lock.get(&id) {
      self.ctx.bind_vertex_array(Some(raw_vao.get_raw_vao()));
      return;
    }
    let i_buffer = if let Some(i_buffer) = &self.i_buffer {
      Some(i_buffer.raw_buffer())
    } else {
      None
    };
    let raw_vao = RawVao::new(
      &self.ctx,
      program.raw_program(),
      Some((self.v_buffer.template(), self.v_buffer.raw_buffer())),
      i_buffer,
    );
    self.ctx.bind_vertex_array(Some(raw_vao.get_raw_vao()));
    lock.insert(id, raw_vao);
  }
}
