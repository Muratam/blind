// use prpr::*;

// for entry_point
pub use wasm_bindgen::prelude::wasm_bindgen as entry_point;
pub extern crate wasm_bindgen;

// for publish
mod html;
mod js;
mod prgl;
mod system;
use prpr::math::*;
pub use system::{run, Core, System};

struct SampleSystem {
  surface: prgl::Texture,
  renderpass: prgl::RenderPass,
  pipeline: prgl::Pipeline,
}

impl System for SampleSystem {
  fn new(core: &Core) -> Self {
    let prgl = core.get_main_prgl();
    let surface = prgl.new_surface();
    let mut renderpass = prgl.new_renderpass();
    renderpass.set_color_target(&surface);
    let pipeline = prgl.new_pipeline();
    Self {
      surface,
      renderpass,
      pipeline,
    }
  }
  fn update(&mut self, core: &Core) {
    let frame = core.get_frame();
    {
      // TODO: GLの update までの流れは別途モジュール化する
      let v = ((frame as f32) / 100.0).sin() * 0.25 + 0.75;
      self
        .renderpass
        .set_clear_color(Some(Vec4::new(v, v, v, 1.0)));
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

pub fn run_sample() {
  js::console::log("create prpr world !!");
  run::<SampleSystem>();
}
