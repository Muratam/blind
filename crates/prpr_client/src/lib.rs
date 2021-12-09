use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::*;

extern crate wasm_bindgen;
extern crate web_sys;

mod html;
mod js;
use html::*;
use prpr::*;

mod scene {
  use crate::*;
  #[allow(unused_must_use)]
  fn render_sample(ctx: &web_sys::CanvasRenderingContext2d) {
    use std::f64::consts::PI;
    ctx.begin_path();
    ctx.arc(75.0, 75.0, 50.0, 0.0, PI * 2.0);
    ctx.move_to(110.0, 75.0);
    ctx.arc(75.0, 75.0, 35.0, 0.0, PI);
    ctx.move_to(65.0, 65.0);
    ctx.arc(60.0, 65.0, 5.0, 0.0, PI * 2.0);
    ctx.move_to(95.0, 65.0);
    ctx.arc(90.0, 65.0, 5.0, 0.0, PI * 2.0);
    ctx.stroke();
  }
  // 三次元の理想的なシーン作成
  // オーバーレイしたりしたい
  pub fn create_fullscreen_3d() {
    let root = html::create_root();
    let canvas = html::append_canvas(&root);
    let context = canvas.get_2d_context();
    render_sample(&context);
    let gl_canvas = html::append_canvas(&root);
    use WebGlRenderingContext as gl;
    let gl = gl_canvas.get_webgl_context();
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
    gl.enable(gl::DEPTH_TEST);
    gl.depth_func(gl::LEQUAL);
    let node = html::append_div(&root);
    node.set_text_content(Some("Hello from Rust!"));
    js::console::log(&context);
    js::console::log(&gl);
    js::console::log("abc");
    js::console::log(&root);
    js::console::error(1 + 2);
    let mut i = 0;
    js::start_animation_frame_loop(Box::new(move || {
      i += 1;
      let text = format!("requestAnimationFrame has been called {} times.", i);
      node.set_text_content(Some(&text));
      let f = ((i % 100) as f32) / 100.0;
      gl.clear_color(f, f, f, 1.0);
      gl.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }));
  }
}

#[wasm_bindgen(start)]
pub fn start() {
  scene::create_fullscreen_3d();
}
