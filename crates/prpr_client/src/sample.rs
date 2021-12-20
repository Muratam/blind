// hoge_client に逃がす前段階でのサンプル
use super::*;
use prgl;
use std::sync::Arc;
crate::shader_attr! {
  struct Global {
    view_mat: mat4,
    proj_mat: mat4,
    add_color: vec4,
  }
  textures PbrTexture {
    normal_map : sampler2D,
    roughness_map : sampler2D
  }
}
pub struct SampleSystem {
  surface: prgl::Texture,
  renderpass: prgl::RenderPass,
  pipeline: prgl::Pipeline,
  global_ubo: Arc<prgl::UniformBuffer<Global>>,
}
/* TODO:
- キーボード入力 / タッチ入力を受け取る
- viewport size を可変にする
- MRTしてポストプロセスをかけてみる
- RenderPassにPipelineを登録する形式にする
*/
impl System for SampleSystem {
  fn new(core: &Core) -> Self {
    let gl = &core.get_main_prgl().gl;
    let surface = Texture::new(gl);
    let mut renderpass = RenderPass::new(gl);
    renderpass.set_color_target(&surface);
    let mut pipeline = Pipeline::new(gl);
    let template = crate::shader_template! {
      attrs: [Global],
      vs_attr: ShapeFactoryVertex,
      fs_attr: { in_color: vec4 },
      out_attr: { out_color: vec4 }
      vs_code: {
        in_color = vec4(position, 1.0);
        gl_Position = proj_mat * view_mat * vec4(position, 1.0);
      },
      fs_code: {
        out_color = in_color + add_color;
      }
    };
    let vao = ShapeFactory::new(gl).create_cube();
    pipeline.set_draw_vao(&Arc::new(vao));
    let global_ubo = UniformBuffer::new(
      gl,
      Global {
        add_color: Vec4::new(0.5, 0.5, 0.5, 0.5),
        view_mat: Mat4::look_at_rh(Vec3::ONE * 5.0, Vec3::ZERO, Vec3::Y),
        proj_mat: Mat4::perspective_rh(3.1415 * 0.25, 1.0, 0.01, 50.0),
      },
    );
    let global_ubo = Arc::new(global_ubo);
    pipeline.add_uniform_buffer(&global_ubo);
    if let Some(shader) = Shader::new(gl, template) {
      pipeline.set_shader(&Arc::new(shader));
    }
    Self {
      surface,
      renderpass,
      pipeline,
      global_ubo,
    }
  }
  fn update(&mut self, core: &Core) {
    let frame = core.get_frame();
    {
      // update world
      let v = ((frame as f32) / 100.0).sin() * 0.25 + 0.75;
      let color = Vec4::new(v, v, v, 0.0);
      self.renderpass.set_clear_color(Some(color));
      // update ubo
      let mut ubo = self.global_ubo.write_lock();
      ubo.add_color = Vec4::new(1.0 - v, 1.0 - v, 1.0 - v, 1.0);
      let rad = (frame as f32) / 100.0;
      ubo.view_mat = Mat4::look_at_rh(
        Vec3::new(rad.sin(), rad.cos(), rad.cos()) * 5.0,
        Vec3::ZERO,
        Vec3::Y,
      );
    }
    {
      // update draw
      self.renderpass.bind();
      self.pipeline.draw();
      core.get_main_prgl().swap_surface(&self.surface);
    }
    // TODO: 2D
    self.render_sample(&core.get_main_2d_context());
    // TODO: HTML
    if frame < 200 {
      let html_layer = core.get_html_layer();
      let text = format!("requestAnimationFrame has been called {} times.", frame);
      let pre_text = html_layer.text_content().unwrap();
      html_layer.set_text_content(Some(&format!("{}{}", &pre_text, &text)));
    }
  }
}

impl SampleSystem {
  fn render_sample(&mut self, ctx: &web_sys::CanvasRenderingContext2d) {
    // note use: `?;` for Result
    use std::f64::consts::PI;
    ctx.begin_path();
    ctx.arc(75.0, 75.0, 50.0, 0.0, PI * 2.0).ok();
    ctx.move_to(110.0, 75.0);
    ctx.arc(75.0, 75.0, 35.0, 0.0, PI).ok();
    ctx.move_to(65.0, 65.0);
    ctx.arc(60.0, 65.0, 5.0, 0.0, PI * 2.0).ok();
    ctx.move_to(95.0, 65.0);
    ctx.arc(90.0, 65.0, 5.0, 0.0, PI * 2.0).ok();
    ctx.stroke();
  }
}
