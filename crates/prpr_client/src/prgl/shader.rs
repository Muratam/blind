// TODO: いい感じにコンパイル時にCPU-GPUの割当を完全にしたい
use super::*;
#[derive(PartialEq)]
pub enum ShaderType {
  VertexShader,
  FragmentShader,
}
pub struct RawShader {
  shader: web_sys::WebGlShader,
  shader_type: ShaderType,
}

impl RawShader {
  pub fn new(gl: &WebGlContext, code: &str, shader_type: ShaderType) -> Option<Self> {
    let create_flag = match &shader_type {
      ShaderType::VertexShader => gl::VERTEX_SHADER,
      ShaderType::FragmentShader => gl::FRAGMENT_SHADER,
    };
    let shader = gl
      .create_shader(create_flag)
      .expect("failed to create shader");
    gl.shader_source(&shader, code);
    gl.compile_shader(&shader);
    let ok = gl
      .get_shader_parameter(&shader, gl::COMPILE_STATUS)
      .as_bool();
    if ok.is_none() || !ok.unwrap() {
      if let Some(info_log) = gl.get_shader_info_log(&shader) {
        log::error("failed to compile shader");
        log::error(code);
        log::error(info_log);
      }
      return None;
    }
    return Some(Self {
      shader,
      shader_type,
    });
  }
}

pub struct RawShaderProgram {
  program: web_sys::WebGlProgram,
}
impl RawShaderProgram {
  pub fn new(gl: &WebGlContext, shaders: &Vec<RawShader>) -> Option<Self> {
    let program = gl
      .create_program()
      .expect("failed to create shader program");
    for shader in shaders {
      gl.attach_shader(&program, &shader.shader);
    }
    gl.link_program(&program);
    let ok = gl
      .get_program_parameter(&program, gl::LINK_STATUS)
      .as_bool();
    if ok.is_none() || !ok.unwrap() {
      if let Some(info_log) = gl.get_program_info_log(&program) {
        log::error("failed to link shader");
        log::error(info_log);
      }
      return None;
    }
    return Some(Self { program });
  }
  pub fn use_program(&self, gl: &WebGlContext) {
    gl.use_program(Some(&self.program));
  }
}
