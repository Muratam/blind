// hoge_client に逃がす前段階でのサンプル
use crate::*;
use std::cell::RefCell;
use std::rc::Rc;

crate::shader_attr! {
  struct Vertex {
    color: vec4,
    position: vec3,
  }
  struct Global {
    add_color: vec4,
  }
}
pub struct SampleSystem {
  surface: prgl::Texture,
  renderpass: prgl::RenderPass,
  pipeline: prgl::Pipeline,
  global_ubo: prgl::UniformBufferPtr<Global>,
}

impl System for SampleSystem {
  fn new(core: &Core) -> Self {
    let prgl = core.get_main_prgl();
    let surface = prgl.new_surface();
    let mut renderpass = prgl.new_renderpass();
    renderpass.set_color_target(&surface);
    let mut pipeline = prgl.new_pipeline();
    let u_data = Global {
      add_color: Vec4::new(0.5, 0.5, 0.5, 0.5),
    };
    let template = crate::shader_template! {
      attrs: [Global],
      vs_attr: Vertex,
      fs_attr: { in_color: vec4 },
      out_attr: { out_color: vec4 }
      vs_code: {
        in_color = color;
        gl_Position = vec4(position, 1.0);
      },
      fs_code: {
        out_color = in_color + add_color;
      }
    };
    let v_data = vec![
      Vertex {
        position: Vec3::Y,
        color: Vec4::X + Vec4::W,
      },
      Vertex {
        position: Vec3::X,
        color: Vec4::Y + Vec4::W,
      },
      Vertex {
        position: -Vec3::X,
        color: Vec4::Z + Vec4::W,
      },
      Vertex {
        position: -Vec3::Y,
        color: Vec4::ONE,
      },
    ];
    let i_data = vec![0, 1, 2, 2, 3, 1];
    let i_size = i_data.len() as i32;
    let i_buffer = prgl.new_index_buffer(i_data);
    let v_buffer = prgl.new_vertex_buffer(v_data);
    let vao = prgl.new_vao(v_buffer, i_buffer);
    pipeline.set_vao(&vao);
    let global_ubo = prgl.new_uniform_buffer(u_data);
    pipeline.add_uniform_buffer(&global_ubo);
    if let Some(shader) = prgl.new_shader(template) {
      pipeline.set_shader(&shader);
    }
    pipeline.set_draw_indexed(0, i_size);
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
      let color = Vec4::new(v, v, v, 1.0);
      self.renderpass.set_clear_color(Some(color));
      let mut ubo = self.global_ubo.borrow_mut();
      let mut ubo = ubo.data_mut();
      ubo.add_color = Vec4::new(1.0, v, 0.0, 1.0);
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
