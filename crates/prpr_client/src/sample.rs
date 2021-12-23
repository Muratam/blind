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
  mapping PbrMapping {
    normal_map : sampler2D,
    roughness_map : sampler2D
  }
}
pub struct SampleSystem {
  surface: Arc<prgl::Texture>,
  renderpass: prgl::RenderPass,
  pipeline: prgl::Pipeline,
  global_ubo: Arc<prgl::UniformBuffer<Global>>,
}
/* TODO:
- キーボード入力 / タッチ入力を受け取る
- texture2darray, texture3d 対応する
- MRTしてポストプロセスをかけてみる
- RenderPassにPipelineを登録する形式にする
- VAOは最後だけに設定できる方がいい (nil -> Vao?)
- fullscreenのテンプレートほしい
- ステートの変更関数呼び出しを減らしたい
*/
impl System for SampleSystem {
  fn new(core: &Core) -> Self {
    let ctx = core.main_prgl().ctx();
    let surface = Arc::new(Texture::new_fill_color(
      ctx,
      640,
      640,
      Vec4::new(1.0, 0.0, 1.0, 0.0),
    ));
    let normal_map = Arc::new(Texture::new_fill_color(
      ctx,
      10,
      10,
      Vec4::new(0.0, 0.0, 1.0, 0.0),
    ));
    let roughness_map = normal_map.clone();
    let mut renderpass = RenderPass::new(ctx);
    // renderpass.set_color_target(Some(&surface));
    let mut pipeline = Pipeline::new(ctx);
    let template = crate::shader_template! {
      attrs: [Global, PbrMapping],
      vs_attr: ShapeFactoryVertex,
      fs_attr: { in_color: vec4 },
      out_attr: { out_color: vec4 }
      vs_code: {
        in_color = vec4(position, 1.0) + texture(roughness_map, vec2(0.5, 0.5));
        gl_Position = proj_mat * view_mat * vec4(position, 1.0);
      },
      fs_code: {
        out_color = in_color + add_color + texture(normal_map, vec2(0.5, 0.5));
      }
    };
    let vao = ShapeFactory::new(ctx).create_cube();
    pipeline.set_draw_vao(&Arc::new(vao));
    let global_ubo = UniformBuffer::new(
      ctx,
      Global {
        add_color: Vec4::new(0.5, 0.5, 0.5, 0.5),
        view_mat: Mat4::look_at_rh(Vec3::ONE * 5.0, Vec3::ZERO, Vec3::Y),
        proj_mat: Mat4::perspective_rh(3.1415 * 0.25, 1.0, 0.01, 50.0),
      },
    );
    let global_ubo = Arc::new(global_ubo);
    pipeline.add_uniform_buffer(&global_ubo);
    if let Some(shader) = Shader::new(ctx, template) {
      pipeline.set_shader(&Arc::new(shader));
    }
    pipeline.add_texture_mapping(&Arc::new(TextureMapping::new(
      ctx,
      PbrMapping {
        normal_map,
        roughness_map,
      },
    )));
    Self {
      surface,
      renderpass,
      pipeline,
      global_ubo,
    }
  }
  fn update(&mut self, core: &Core) {
    let frame = core.frame();
    let prgl = core.main_prgl();
    {
      // update world
      let v = ((frame as f32) / 100.0).sin() * 0.25 + 0.75;
      let color = Vec4::new(v, v, v, 1.0);
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
      ubo.proj_mat = Mat4::perspective_rh(3.1415 * 0.25, prgl.aspect_ratio(), 0.01, 50.0);
    }
    {
      // update draw
      self.renderpass.set_viewport(Some(&prgl.full_viewport()));
      self.renderpass.bind();
      self.pipeline.draw();
      prgl.flush();
    }
    // TODO: 2D
    self.render_sample(&core.main_2d_context());
    // TODO: HTML
    if frame < 200 {
      let html_layer = core.html_layer();
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
