use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::*;

extern crate wasm_bindgen;
extern crate web_sys;

pub mod console;
mod html;
use html::*;
use prpr::*;

// #[wasm_bindgen] extern "C" { pub fn alert(s: &str); }

pub fn render_sample(context: &web_sys::CanvasRenderingContext2d) {
  use std::f64::consts::PI;
  context.begin_path();
  context.arc(75.0, 75.0, 50.0, 0.0, PI * 2.0).ok();
  context.move_to(110.0, 75.0);
  context.arc(75.0, 75.0, 35.0, 0.0, PI).ok();
  context.move_to(65.0, 65.0);
  context.arc(60.0, 65.0, 5.0, 0.0, PI * 2.0).ok();
  context.move_to(95.0, 65.0);
  context.arc(90.0, 65.0, 5.0, 0.0, PI * 2.0).ok();
  context.stroke();
}

#[wasm_bindgen(start)]
pub fn start() {
  let root = html::create_root();
  let canvas = html::append_canvas(&root);
  let context = canvas.get_2d_context();
  render_sample(&context);
  let gl_canvas = html::append_canvas(&root);
  use WebGlRenderingContext as gl;
  let gl = gl_canvas.get_webgl_context();
  gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
  gl.clear_color(0.0, 0.0, 0.0, 1.0);
  gl.enable(gl::DEPTH_TEST);
  gl.depth_func(gl::LEQUAL);
  gl.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
  let node = html::append_div(&root);
  node.set_text_content(Some("Hello from Rust!"));
  console::log(&context);
  console::log(&gl);
  console::log("abc");
  console::log(&root);
  console::log(1 + 2);
}
