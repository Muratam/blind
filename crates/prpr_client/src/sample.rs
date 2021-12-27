// hoge_client に逃がす前段階でのサンプル
use super::*;
use prgl;
use std::sync::Arc;
pub struct SampleSystem {
  surface: prgl::Surface,
  camera: prgl::Camera,
  object: prgl::TransformObject,
}

impl System for SampleSystem {
  fn new(core: &Core) -> Self {
    let ctx = core.main_prgl().ctx();
    let template = crate::shader_template! {
      attrs: [
        CameraAttribute, TransformAttribute, PbrAttribute,
        PbrMapping
      ],
      vs_attr: ShapeVertex,
      vs_code: {
        in_color = vec4(position, 1.0);
        gl_Position = view_proj_mat * model_mat * vec4(position, 1.0);
      },
      fs_attr: { in_color: vec4 },
      fs_code: {
        out_color = in_color + texture(normal_map, vec2(0.5, 0.5));
      }
      out_attr: { out_color: vec4 }
    };
    let mut object = TransformObject::new(ctx);
    object.add(&Shape::new_cube(ctx));
    object.add(&PbrMaterial::new(ctx));
    object.add(&MayShader::new(ctx, template));
    let mut surface = Surface::new(core.main_prgl()); // 自分で生成？
    let camera = Camera::new(ctx);
    surface.add(&camera); // screen ？
    Self {
      surface,
      object,
      camera,
    }
  }

  fn update(&mut self, core: &Core) {
    let frame = core.frame();

    // update by user world
    let rad = (frame as f32) / 100.0;
    let v = rad.sin() * 0.25 + 0.75;
    let color = Vec4::new(v, v, v, 1.0);
    self.surface.set_clear_color(Some(color));
    self.camera.write_lock().camera_pos = Vec3::new(rad.sin(), rad.cos(), rad.cos()) * 5.0;
    self.object.transform.write_lock().translate = Vec3::new(
      (frame as f32).sin() * 0.01,
      ((frame + 1) as f32).sin() * 0.01,
      ((frame + 2) as f32).sin() * 0.01,
    );

    // update by screen
    self.surface.update(core.main_prgl()); // 消したい
    self.camera.write_lock().aspect_ratio = self.surface.aspect_ratio(); // by screen

    // draw start
    let desc_ctx = self.surface.bind();
    self.object.pipeline.draw(&desc_ctx);

    // the others
    self.render_sample(core);
  }
}
/* TODO:
- キーボード入力 / タッチ入力を受け取る
  - https://rustwasm.github.io/docs/wasm-bindgen/examples/paint.html
- RenderPassにPipelineを登録する形式にする
  - ステートの変更関数呼び出しを減らしたい
- fullscreenのテンプレートほしい
  - VAOは最後だけに設定できる方がいい (nil -> Vao?)
  - MRTしてポストプロセスをかけてみる
- texture2darray, texture3d 対応する
  - texture として扱いたい？
    - https://ics.media/web3d-maniacs/webgl2_texture2darray/
    - https://ics.media/web3d-maniacs/webgl2_texture3d/
  - texStorage2D
    - https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/copyBufferSubData
    - https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D
  - https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/WebGL_best_practices#teximagetexsubimage_uploads_esp._videos_can_cause_pipeline_flushes
- client_wait_sync ?
  - https://ics.media/entry/19043/
  - https://inside.pixiv.blog/petamoriken/5853
  - 描画だけをメインスレッドにすればいいかも
  - https://rustwasm.github.io/wasm-bindgen/examples/wasm-in-web-worker.html
- renderbuffer
  - MSAA: https://ics.media/web3d-maniacs/webgl2_renderbufferstoragemultisample/
- zoom-in/outの解像度耐えたい
  - pinch-in/out も
  - window.visualViewport
  - cssの方でscaleいじれば強引にいけそう
- Async Computeしたい
  - tf
- 複数のカメラで描画したい
  - 同じのを別カメラで２回やればOK
  - Selection はカメラから？
  - 指操作はカメラに紐付ける？
  - デバッグ用のが欲しくはなるかも
  - 結局ズーム操作はエミュレーションすることになるのでは
- ctx 消したい(Singleton?)
*/

impl SampleSystem {
  fn render_sample(&mut self, core: &Core) {
    // TODO: 2D
    {
      let ctx = core.main_2d_context();
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
    // TODO: HTML
    {
      let frame = core.frame();
      let html_layer = core.html_layer();
      if frame > 1000 {
        html_layer.set_text_content(None);
      }
      let frame = frame % 200;
      let text = format!("{} ", frame);
      let pre_text = html_layer.text_content().unwrap();
      html_layer.set_text_content(Some(&format!("{}{}", &pre_text, &text)));
    }
  }
}
