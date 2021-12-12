use super::*;
use std::ops::{Index, IndexMut};
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

pub struct RawGpuBuffer {
  buffer: web_sys::WebGlBuffer,
  size: i32,
  usage: BufferUsage,
}
impl RawGpuBuffer {
  pub fn new<T: Sized>(gl: &GlContext, data: &[T], usage: BufferUsage) -> Self {
    let result = Self::new_uninitialized::<T>(gl, data.len(), usage);
    result.write(gl, 0, data);
    result
  }
  pub fn new_uninitialized<T: Sized>(gl: &GlContext, count: usize, usage: BufferUsage) -> Self {
    let u8_size = std::mem::size_of::<T>() * count;
    Self::new_uninitialized_untyped(gl, u8_size as i32, usage)
  }
  pub fn new_uninitialized_untyped(gl: &GlContext, size: i32, usage: BufferUsage) -> Self {
    let buffer = gl.create_buffer().expect("failed to craete buffer");
    let target = usage as u32;
    gl.bind_buffer(target, Some(&buffer));
    gl.buffer_data_with_i32(target, size, usage_to_store_type(&usage));
    if SET_BIND_NONE_AFTER_WORK {
      gl.bind_buffer(target, None);
    }
    Self {
      buffer,
      size,
      usage,
    }
  }
  pub fn write<T: Sized>(&self, gl: &GlContext, offset: usize, data: &[T]) {
    use core::slice;
    let u8_size = std::mem::size_of::<T>() * data.len();
    let ptr = data.as_ptr() as *const u8;
    let u8_data: &[u8] = unsafe { slice::from_raw_parts(ptr, u8_size) };
    let u8_offset = std::mem::size_of::<T>() * offset;
    self.write_untyped(gl, u8_offset as i32, u8_data);
  }
  pub fn write_untyped(&self, gl: &GlContext, offset: i32, data: &[u8]) {
    let size = offset + data.len() as i32;
    if offset < 0 || size > self.size {
      log::error(format!(
        "invalid buffer write size: offset:{}, size:{}, reserved:{}",
        offset, size, self.size
      ));
      return;
    }
    // log::debug(format!("{:?}", data));
    let target = self.usage as u32;
    gl.bind_buffer(target, Some(&self.buffer));
    gl.buffer_sub_data_with_i32_and_u8_array(target, offset, data);
    if SET_BIND_NONE_AFTER_WORK {
      gl.bind_buffer(target, None);
    }
  }
  pub fn raw_buffer(&self) -> &web_sys::WebGlBuffer {
    &self.buffer
  }
  pub fn raw_target(&self) -> u32 {
    self.usage as u32
  }
}

pub struct RawVao {
  vao: web_sys::WebGlVertexArrayObject,
}
impl RawVao {
  pub fn new(gl: &GlContext, v_buffer: &RawGpuBuffer, i_buffer: Option<&RawGpuBuffer>) -> Self {
    let vao = gl.create_vertex_array().expect("failed to create vao");
    gl.bind_vertex_array(Some(&vao));
    gl.bind_buffer(v_buffer.raw_target(), Some(v_buffer.raw_buffer()));
    let v_type_size = 32;
    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_with_i32(0, 3, gl::FLOAT, false, v_type_size, 0);
    gl.enable_vertex_attrib_array(1);
    gl.vertex_attrib_pointer_with_i32(1, 4, gl::FLOAT, false, v_type_size, 4);
    if let Some(i_buffer) = i_buffer {
      gl.bind_buffer(i_buffer.raw_target(), Some(i_buffer.raw_buffer()));
    }
    if SET_BIND_NONE_AFTER_WORK {
      gl.bind_vertex_array(None);
      gl.bind_buffer(v_buffer.raw_target(), None);
      if let Some(i_buffer) = i_buffer {
        gl.bind_buffer(i_buffer.raw_target(), None);
      }
    }
    Self { vao }
  }
  pub fn get_raw_vao(&self) -> &web_sys::WebGlVertexArrayObject {
    &self.vao
  }
}

// pub struct RawVertexBufferAttrs {}
// impl RawVertexBufferAttrs {
//   pub fn new(gl: &GlContext, buffer: &RawGpuBuffer) {
//     let vao = gl
//       .create_vertex_array()
//       .expect("failed to create vertex array");
//     // gl.bind_vertex_array(Some(&vao));
//     // for i in vboDataArray {
//     //   vbo = gl.create_buffer();
//     //   gl.bind_buffer(gl::ARRAY_BUFFER, vbo);
//     //   gl.enable_vertex_attrib_array(attr_locs[i]);
//     //   gl.vertex_attrib_pointer_with_i32(attr_locs[i], attS[i], gl::FLOAT, false, 0, 0);
//     // }
//     // if loc >= 0 {
//     //   gl.enable_vertex_attrib_array(loc as u32);
//     //   gl.vertex_attrib_pointer_with_i32(loc as u32, 3, gl::FLOAT, false, 0, 0);
//     // }
//   }
// }

// pub struct RawBuffer<T: Sized + Default> {
//   buffer: RawUntypedGpuBuffer,
//   data: Vec<T>,
//   is_dirty: bool,
// }
// impl<T: Sized + Default> RawBuffer<T> {
//   pub fn new(gl: &GlContext, count: usize, usage: BufferUsage) -> Self {
//     let u8_size = std::mem::size_of::<T>() * count;
//     Self {
//       buffer: RawUntypedGpuBuffer::new(gl, u8_size as i32, usage),
//       data: vec![Default::default(); count],
//       is_dirty: true,
//     }
//   }
//   pub fn apply(&mut self, gl: &GlContext) {
//     if !self.is_dirty {
//       return;
//     }
//     self.is_dirty = false;
//   }
// }
// impl<T: Sized + Default> Index<usize> for RawBuffer<T> {
//   type Output = T;
//   fn index(&self, index: usize) -> &Self::Output {
//     &self.data[index]
//   }
// }
// impl<T: Sized + Default> IndexMut<usize> for RawBuffer<T> {
//   fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//     self.is_dirty = true;
//     &mut self.data[index]
//   }
// }
