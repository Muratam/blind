use crate::js;
use web_sys::*;
mod full_screen_layers;
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
// 三次元の理想的なワールド作成
// オーバーレイしたりしたい
pub fn create() {
  let mut layers = full_screen_layers::new();
  use WebGlRenderingContext as gl;
  let gl = layers.get_main_3d_context();
  gl.viewport(0, 0, 10, 10);
  gl.enable(gl::DEPTH_TEST);
  gl.depth_func(gl::LEQUAL);
  js::start_animation_frame_loop(Box::new(move |frame| {
    layers.check_resized();
    let ctx2d = layers.get_main_2d_context();
    render_sample(&ctx2d);
    if frame < 200 {
      let html_layer = layers.get_html_layer();
      let text = format!("requestAnimationFrame has been called {} times.", frame);
      let pre_text = html_layer.text_content().unwrap();
      html_layer.set_text_content(Some(&format!("{}{}", &pre_text, &text)));
    }
    let f = ((frame / 100 % 100) as f32) / 100.0;
    gl.clear_color(f, f, f, 0.2);
    gl.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
  }));
  // js::console::log("prpr world created !!");
}
