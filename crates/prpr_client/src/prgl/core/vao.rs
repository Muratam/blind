use super::*;
use std::collections::HashMap;
pub struct Vao<T: BufferAttribute> {
  gl: Arc<GlContext>,
  v_buffer: VertexBuffer<T>,
  i_buffer: Option<IndexBuffer>,
  shader_id_to_raw_vao: HashMap<u64, RawVao>,
}
pub trait VaoTrait {
  // returns successed
  fn bind(&mut self, program: &RawShaderProgram);
}
// 裏でも更新する
pub type VaoDynPtr = Arc<RwLock<dyn VaoTrait>>;
pub type VaoPtr<T> = Arc<RwLock<Vao<T>>>;
impl<T: BufferAttribute> Vao<T> {
  pub fn new(gl: &Arc<GlContext>, v_buffer: VertexBuffer<T>, i_buffer: IndexBuffer) -> Self {
    Self {
      gl: Arc::clone(gl),
      v_buffer,
      i_buffer: Some(i_buffer),
      shader_id_to_raw_vao: HashMap::new(),
    }
  }
  pub fn new_without_index_buffer(gl: &Arc<GlContext>, v_buffer: VertexBuffer<T>) -> Self {
    Self {
      gl: Arc::clone(gl),
      v_buffer,
      i_buffer: None,
      shader_id_to_raw_vao: HashMap::new(),
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
  fn bind(&mut self, program: &RawShaderProgram) {
    let id = program.raw_program_id();
    if let Some(raw_vao) = self.shader_id_to_raw_vao.get(&id) {
      self.gl.bind_vertex_array(Some(raw_vao.get_raw_vao()));
    }
    let i_buffer = if let Some(i_buffer) = &self.i_buffer {
      Some(i_buffer.raw_buffer())
    } else {
      None
    };
    let raw_vao = RawVao::new(
      &self.gl,
      program.raw_program(),
      Some((self.v_buffer.template(), self.v_buffer.raw_buffer())),
      i_buffer,
    );
    self.gl.bind_vertex_array(Some(raw_vao.get_raw_vao()));
    self.shader_id_to_raw_vao.insert(id, raw_vao);
  }
}
