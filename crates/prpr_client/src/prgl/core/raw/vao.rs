use super::*;

pub struct RawVao {
  gl: Rc<GlContext>,
  vao: web_sys::WebGlVertexArrayObject,
}
impl RawVao {
  pub fn new(
    gl: &Rc<GlContext>,
    program: &web_sys::WebGlProgram,
    vs_in_template_buffer: Option<(&VsInTemplate, &RawGpuBuffer)>,
    i_buffer: Option<&RawGpuBuffer>,
  ) -> Self {
    let vao = gl.create_vertex_array().expect("failed to create vao");
    gl.bind_vertex_array(Some(&vao));
    if let Some(vs_in_template_buffer) = vs_in_template_buffer {
      let vs_in = vs_in_template_buffer.0;
      let v_buffer = vs_in_template_buffer.1;
      if v_buffer.raw_target() != gl::ARRAY_BUFFER {
        log::error("Not Vertex Buffer");
      }
      gl.bind_buffer(gl::ARRAY_BUFFER, Some(v_buffer.raw_buffer()));
      assert_eq!(vs_in.offsets.len(), vs_in.keys.len());
      assert_eq!(vs_in.values.len(), vs_in.keys.len());
      for i in 0..vs_in.offsets.len() {
        let location = gl.get_attrib_location(program, vs_in.keys[i]);
        if location < 0 {
          // log::info(format!("no vertex attribute: {}", vs_in.keys[i]));
          continue;
        }
        gl.enable_vertex_attrib_array(location as u32);
        let value = &vs_in.values[i];
        gl.vertex_attrib_pointer_with_i32(
          location as u32,
          value.get_single_primitive_count(),
          value.get_single_primitive_type() as u32,
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
      gl.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(i_buffer.raw_buffer()));
    }
    if SET_BIND_NONE_AFTER_WORK {
      gl.bind_vertex_array(None);
      gl.bind_buffer(gl::ARRAY_BUFFER, None);
      if i_buffer.is_some() {
        gl.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, None);
      }
    }
    Self {
      gl: Rc::clone(gl),
      vao,
    }
  }

  pub fn get_raw_vao(&self) -> &web_sys::WebGlVertexArrayObject {
    &self.vao
  }
}

impl Drop for RawVao {
  fn drop(&mut self) {
    self.gl.delete_vertex_array(Some(&self.vao));
  }
}
