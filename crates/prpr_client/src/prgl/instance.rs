use super::*;
use crate::html;
pub struct Instance {
  gl: Arc<GlContext>,
  max_width: i32,
  max_height: i32,
}
impl Instance {
  pub fn new(gl: gl) -> Self {
    // 一度生成したら固定
    let screen = html::screen();
    Self {
      gl: Arc::new(gl),
      max_width: screen.width().unwrap(),
      max_height: screen.height().unwrap(),
    }
  }
  // 諸々更新が終わった後このテクスチャを利用する
  pub fn swap_surface(&self, surface: &Texture) {
    // WARN: surfaceテクスチャを使う
    let gl = &self.gl;
    gl.flush();
    // client_wait_sync ?
  }
  // create gpu objects
  // pub fn new_sampler(&self) {}
  // pub fn new_texture(&self) -> Texture {
  //   Texture {}
  // }
  pub fn new_index_buffer(&self, data: Vec<IndexBufferType>) -> IndexBuffer {
    IndexBuffer::new(&self.gl, data)
  }
  pub fn new_vertex_buffer<T: BufferAttribute>(&self, data: Vec<T>) -> VertexBuffer<T> {
    VertexBuffer::new(&self.gl, data)
  }
  pub fn new_uniform_buffer<T: BufferAttribute>(&self, data: T) -> UniformBufferPtr<T> {
    Arc::new(RwLock::new(UniformBuffer::new(&self.gl, data)))
  }
  pub fn new_vao<T: BufferAttribute>(
    &self,
    v_buffer: VertexBuffer<T>,
    i_buffer: IndexBuffer,
  ) -> Arc<Vao<T>> {
    Arc::new(Vao::new(&self.gl, v_buffer, i_buffer))
  }
  pub fn new_shader(&self, template: ShaderTemplate) -> Option<Arc<Shader>> {
    if let Some(shader) = Shader::new(&self.gl, template) {
      Some(Arc::new(shader))
    } else {
      None
    }
  }
  pub fn new_surface(&self) -> Texture {
    Texture::new(&self.gl)
  }
  pub fn new_pipeline(&self) -> Pipeline {
    Pipeline::new(&self.gl)
  }
  pub fn new_renderpass(&self) -> RenderPass {
    RenderPass::new(&self.gl)
  }
  pub fn new_shape_factory(&self) -> ShapeFactory {
    ShapeFactory::new(&self.gl)
  }
}
