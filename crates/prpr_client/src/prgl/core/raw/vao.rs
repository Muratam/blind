use super::*;

pub struct RawVao {
  vao: web_sys::WebGlVertexArrayObject,
  vao_id: u64,
}
use std::sync::atomic::{AtomicUsize, Ordering};
static RAW_VAO_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl RawVao {
  pub fn new(
    program: &web_sys::WebGlProgram,
    vs_in_template_buffer: Option<(&VsInTemplate, &RawBuffer)>,
    i_buffer: Option<&RawBuffer>,
  ) -> Self {
    let ctx = Instance::ctx();
    let vao = ctx.create_vertex_array().expect("failed to create vao");
    ctx.bind_vertex_array(Some(&vao));
    if let Some(vs_in_template_buffer) = vs_in_template_buffer {
      let vs_in = vs_in_template_buffer.0;
      let v_buffer = vs_in_template_buffer.1;
      if v_buffer.raw_target() != gl::ARRAY_BUFFER {
        log::error("Not Vertex Buffer");
      }
      ctx.bind_buffer(gl::ARRAY_BUFFER, Some(v_buffer.raw_buffer()));
      assert_eq!(vs_in.offsets.len(), vs_in.keys.len());
      assert_eq!(vs_in.values.len(), vs_in.keys.len());
      for i in 0..vs_in.offsets.len() {
        let location = ctx.get_attrib_location(program, vs_in.keys[i]);
        if location < 0 {
          // log::info(format!("no vertex attribute: {}", vs_in.keys[i]));
          continue;
        }
        ctx.enable_vertex_attrib_array(location as u32);
        let value = &vs_in.values[i];
        ctx.vertex_attrib_pointer_with_i32(
          location as u32,
          value.single_primitive_count(),
          value.single_primitive_type() as u32,
          false,
          vs_in.size as i32,
          vs_in.offsets[i] as i32,
        );
      }
    }
    if let Some(i_buffer) = i_buffer {
      if i_buffer.raw_target() != gl::ELEMENT_ARRAY_BUFFER {
        log::error("Not Index Buffer");
      }
      ctx.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(i_buffer.raw_buffer()));
    }
    if SET_BIND_NONE_AFTER_WORK {
      ctx.bind_vertex_array(None);
      ctx.bind_buffer(gl::ARRAY_BUFFER, None);
      if i_buffer.is_some() {
        ctx.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, None);
      }
    }
    let vao_id = RAW_VAO_ID_COUNTER.fetch_add(1, Ordering::SeqCst) as u64;
    Self { vao, vao_id }
  }

  pub fn raw_vao(&self) -> &web_sys::WebGlVertexArrayObject {
    &self.vao
  }
  pub fn vao_id(&self) -> u64 {
    self.vao_id
  }
}

impl Drop for RawVao {
  fn drop(&mut self) {
    let ctx = Instance::ctx();
    ctx.delete_vertex_array(Some(&self.vao));
  }
}
