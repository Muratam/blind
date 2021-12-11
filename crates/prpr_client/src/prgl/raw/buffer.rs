use super::*;

#[derive(Clone, Copy)]
pub enum BufferUsage {
  Vertex = gl::ARRAY_BUFFER as isize,
  Index = gl::ELEMENT_ARRAY_BUFFER as isize,
  Uniform = gl::UNIFORM_BUFFER as isize,
  TransformFeedback = gl::TRANSFORM_FEEDBACK_BUFFER as isize,
  TransferSrc = gl::COPY_READ_BUFFER as isize,
  TransferDst = gl::COPY_WRITE_BUFFER as isize,
}
fn usage_to_store_type(usage: &BufferUsage) -> u32 {
  // https://developer.mozilla.org/ja/docs/Web/API/WebGLRenderingContext/bufferData
  match &usage {
    Vertex => gl::STATIC_DRAW,
    Index => gl::STATIC_DRAW,
    Uniform => gl::STREAM_DRAW,
    TransformFeedback => gl::STREAM_COPY,
    TransferSrc => gl::STATIC_DRAW,
    TransferDst => gl::STATIC_READ,
  }
}

pub struct RawBuffer {
  buffer: web_sys::WebGlBuffer,
  size: i32,
  usage: BufferUsage,
}
impl RawBuffer {
  pub fn new(gl: &GlContext, size: i32, usage: BufferUsage) -> Self {
    let buffer = gl.create_buffer().expect("failed to craete buffer");
    let target = usage as u32;
    gl.bind_buffer(target, Some(&buffer));
    gl.buffer_data_with_i32(target, size, usage_to_store_type(&usage));
    gl.bind_buffer(target, None);
    Self {
      buffer,
      size,
      usage,
    }
  }
  pub fn write(&self, gl: &GlContext, offset: i32, data: &[u8]) {
    let target = self.usage as u32;
    gl.bind_buffer(target, Some(&self.buffer));
    gl.buffer_sub_data_with_i32_and_u8_array(target, offset, data);
    gl.bind_buffer(target, None);
  }
  pub fn execute_bind_buffer_command(&self, gl: &GlContext) {
    let target = self.usage as u32;
    gl.bind_buffer(target, Some(&self.buffer));
  }
}
