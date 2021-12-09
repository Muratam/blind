use crate::html;
use crate::html::*;
use crate::js;
use web_sys::*;

fn setup_global_style() {
  let css = r###" * {
  padding: 0px;
  border: 0px;
  margin: 0px;
  }"###;
  let _ = append_css(&body(), css);
}

fn create_root() -> web_sys::HtmlDivElement {
  append_div(&body())
}

fn to_floating_fullscreen_layer(elem: &web_sys::HtmlElement, z_index: i64) {
  let style = elem.style();
  style.set_property("position", "absolute").ok();
  style.set_property("width", "100%").ok();
  style.set_property("height", "100%").ok();
  style.set_property("z-index", &z_index.to_string()).ok();
}

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
  setup_global_style();
  let root = create_root();
  // normal
  let for_html = html::append_div(&root);
  to_floating_fullscreen_layer(&for_html, 2);
  for_html.set_text_content(Some("Hello from Rust!"));
  for_html.style().set_property("overflow", "scroll").ok();
  // 2d canvas
  let canvas = html::append_canvas(&root);
  to_floating_fullscreen_layer(&canvas, 1);
  let context = canvas.get_2d_context();
  render_sample(&context);
  // webgl canvas
  let gl_canvas = html::append_canvas(&root);
  to_floating_fullscreen_layer(&gl_canvas, 0);
  use WebGlRenderingContext as gl;
  let gl = gl_canvas.get_webgl_context();
  gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
  gl.enable(gl::DEPTH_TEST);
  gl.depth_func(gl::LEQUAL);
  js::console::log(&context);
  js::console::log(&gl);
  js::console::log("abc");
  js::console::log(&root);
  js::console::log(1 + 2);
  let mut i = 0;
  js::start_animation_frame_loop(Box::new(move || {
    i += 1;
    if i < 200 {
      let text = format!("requestAnimationFrame has been called {} times.", i);
      let pre_text = for_html.text_content().unwrap();
      for_html.set_text_content(Some(&format!("{}{}", &pre_text, &text)));
    }
    let f = ((i % 100) as f32) / 100.0;
    gl.clear_color(f, f, f, 0.2);
    gl.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
  }));
}
