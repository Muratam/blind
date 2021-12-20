use super::*;

#[derive(PartialEq)]
pub enum ShaderType {
  VertexShader,
  FragmentShader,
}

pub struct RawShader {
  ctx: ArcGlContext,
  shader: web_sys::WebGlShader,
  shader_type: ShaderType,
}
impl RawShader {
  pub fn new(ctx: &ArcGlContext, code: &str, shader_type: ShaderType) -> Option<Self> {
    let create_flag = match &shader_type {
      ShaderType::VertexShader => gl::VERTEX_SHADER,
      ShaderType::FragmentShader => gl::FRAGMENT_SHADER,
    };
    let shader = ctx
      .create_shader(create_flag)
      .expect("failed to create shader");
    ctx.shader_source(&shader, code);
    ctx.compile_shader(&shader);
    if ctx
      .get_shader_parameter(&shader, gl::COMPILE_STATUS)
      .is_falsy()
    {
      if let Some(info_log) = ctx.get_shader_info_log(&shader) {
        log::error("failed to compile shader");
        log::error(code);
        log::error(info_log);
      }
      return None;
    }
    return Some(Self {
      ctx: ctx.clone(),
      shader,
      shader_type,
    });
  }
}
impl Drop for RawShader {
  fn drop(&mut self) {
    self.ctx.delete_shader(Some(&self.shader));
  }
}

use std::sync::atomic::{AtomicUsize, Ordering};
static RAW_SHADER_PROGRAM_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
pub struct RawShaderProgram {
  ctx: ArcGlContext,
  program: web_sys::WebGlProgram,
  program_id: u64,
}
pub struct RawShaderProgramContents {
  pub vertex_shader: Option<RawShader>,
  pub fragment_shader: Option<RawShader>,
}
impl RawShaderProgram {
  pub fn new(ctx: &ArcGlContext, template: &ShaderTemplate) -> Option<Self> {
    let vs_code = template.vs_code();
    let fs_code = template.fs_code();
    let vertex_shader = RawShader::new(ctx, vs_code.as_str(), ShaderType::VertexShader);
    let fragment_shader = RawShader::new(ctx, fs_code.as_str(), ShaderType::FragmentShader);
    Self::new_from_raw_shaders(
      ctx,
      &RawShaderProgramContents {
        vertex_shader,
        fragment_shader,
      },
    )
  }
  pub fn new_from_raw_shaders(
    ctx: &ArcGlContext,
    shaders: &RawShaderProgramContents,
  ) -> Option<Self> {
    let program = ctx
      .create_program()
      .expect("failed to create shader program");
    if let Some(shader) = &shaders.vertex_shader {
      if shader.shader_type != ShaderType::VertexShader {
        log::error("Not Vertex Shader");
        return None;
      }
      ctx.attach_shader(&program, &shader.shader);
    }
    if let Some(shader) = &shaders.fragment_shader {
      if shader.shader_type != ShaderType::FragmentShader {
        log::error("Not Fragment Shader");
        return None;
      }
      ctx.attach_shader(&program, &shader.shader);
    }
    ctx.link_program(&program);
    if ctx
      .get_program_parameter(&program, gl::LINK_STATUS)
      .is_falsy()
    {
      if let Some(info_log) = ctx.get_program_info_log(&program) {
        log::error("failed to link shader");
        log::error(info_log);
      }
      return None;
    }
    ctx.validate_program(&program);
    if ctx
      .get_program_parameter(&program, gl::VALIDATE_STATUS)
      .is_falsy()
    {
      if let Some(info_log) = ctx.get_program_info_log(&program) {
        log::error("failed to validate shader");
        log::error(info_log);
      }
      return None;
    }
    let program_id = RAW_SHADER_PROGRAM_ID_COUNTER.fetch_add(1, Ordering::SeqCst) as u64;
    return Some(Self {
      ctx: ctx.clone(),
      program,
      program_id,
    });
  }
  pub fn use_program(&self) {
    self.ctx.use_program(Some(&self.program));
  }
  pub fn raw_program(&self) -> &web_sys::WebGlProgram {
    &self.program
  }
  pub fn raw_program_id(&self) -> u64 {
    self.program_id
  }
}
impl Drop for RawShaderProgram {
  fn drop(&mut self) {
    self.ctx.delete_program(Some(&self.program));
  }
}
