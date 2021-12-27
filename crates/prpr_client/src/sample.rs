// hoge_client に逃がす前段階でのサンプル
use super::*;
use prgl;

fn casual_shader() -> ShaderTemplate {
  crate::shader_template! {
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
  }
}

crate::shader_attr! {
  mapping ToGrayScaleMapping {
    src_texture: sampler2D
  }
}

fn grayscale_shader() -> ShaderTemplate {
  crate::shader_template! {
    attrs: [ToGrayScaleMapping],
    vs_attr: FullScreenVertex,
    vs_code: {
      gl_Position = vec4(position, 0.5, 1.0);
    },
    fs_attr: {},
    fs_code: {
      ivec2 iuv = ivec2(gl_FragCoord.x, gl_FragCoord.y);
      vec3 rgb = texelFetch(src_texture, iuv, 0).rgb;
      float g = (rgb.r + rgb.g + rgb.b) / 3.0;
      out_color = vec4(g, g, g, 1.0);
    }
    out_attr: { out_color: vec4 }
  }
}

pub struct SampleSystem {
  // 3d
  objects: Vec<prgl::TransformObject>,
  renderpass: prgl::RenderPass,
  camera: prgl::Camera,
  // post effect
  grayscale_surface: prgl::Surface,
  grayscale_pipeline: prgl::Pipeline,
}
impl System for SampleSystem {
  fn new(core: &Core) -> Self {
    let ctx = core.main_prgl().ctx();
    let shader = MayShader::new(ctx, casual_shader());
    let material = PbrMaterial::new(ctx);
    let shape = Shape::new_cube(ctx);
    let mut objects = Vec::new();
    const COUNT: u32 = 4;
    for x in 0..COUNT {
      for y in 0..COUNT {
        for z in 0..COUNT {
          let mut object = TransformObject::new(ctx);
          object.pipeline.add(&shape);
          object.pipeline.add(&material);
          object.pipeline.add(&shader);
          object.transform.write_lock().translate = Vec3::new(
            x as f32 - (COUNT as f32) * 0.5,
            y as f32 - (COUNT as f32) * 0.5,
            z as f32 - (COUNT as f32) * 0.5,
          );
          object.transform.write_lock().scale = Vec3::ONE * 0.72;
          objects.push(object);
        }
      }
    }
    let camera = Camera::new(ctx);
    let mut renderpass = RenderPass::new(ctx);
    let max_viewport = core.main_prgl().full_max_viewport();
    renderpass.set_clear_color(Some(Vec4::new(1.0, 1.0, 1.0, 1.0)));
    renderpass.set_clear_depth(Some(1.0));
    renderpass.add(&camera);
    let src_texture = Arc::new(Texture::new_uninitialized(
      ctx,
      &Texture2dDescriptor {
        width: max_viewport.width as usize,
        height: max_viewport.height as usize,
        format: PixelFormat::R8G8B8A8,
        mipmap: true,
      },
    ));
    let depth_texture = Arc::new(Texture::new_uninitialized(
      ctx,
      &Texture2dDescriptor {
        width: max_viewport.width as usize,
        height: max_viewport.height as usize,
        format: PixelFormat::Depth24,
        mipmap: false,
      },
    ));
    renderpass.set_color_target(Some(&src_texture));
    renderpass.set_depth_target(Some(&depth_texture));

    let grayscale_surface = Surface::new(core.main_prgl());
    let mut grayscale_pipeline = FullScreen::new_pipeline(ctx);
    grayscale_pipeline.add(&MayShader::new(ctx, grayscale_shader()));
    grayscale_pipeline.add(&Arc::new(TextureMapping::new(
      ctx,
      ToGrayScaleMapping { src_texture },
    )));

    Self {
      objects,
      camera,
      renderpass,
      grayscale_pipeline,
      grayscale_surface,
    }
  }

  fn update(&mut self, core: &Core) {
    let frame = core.frame();
    // update by user world
    let f = (frame as f32) / 100.0;
    self.camera.write_lock().camera_pos = Vec3::new(f.sin(), f.cos(), f.cos()) * 5.0;
    self.camera.write_lock().aspect_ratio = core.main_prgl().full_viewport().aspect_ratio();
    self
      .renderpass
      .set_viewport(Some(&core.main_prgl().full_viewport()));

    // draw start
    // TODO: use executer
    let mut cmd = prgl::Command::new(core.main_prgl().ctx());
    {
      let desc_ctx = self.renderpass.bind();
      for object in &self.objects {
        object.pipeline.draw(&mut cmd, &desc_ctx);
      }
    }
    {
      let desc_ctx = self.grayscale_surface.bind();
      self.grayscale_pipeline.draw(&mut cmd, &desc_ctx);
    }

    // the others
    self.render_sample(core);
  }
}
/* TODO:
- fullscreenのテンプレートほしい
  - VAOは最後だけに設定できる方がいい (nil -> Vao?)
  - MRTしてポストプロセスをかけてみる
- renderbuffer
  - MSAA: https://ics.media/web3d-maniacs/webgl2_renderbufferstoragemultisample/
  - mipmap がなぜかはいっている？
- RenderPassにPipelineを登録する形式にする
- 複数のカメラで描画したい
  - 同じのを別カメラで２回やればOK
  - Selection はカメラから？
  - 指操作はカメラに紐付ける？
  - デバッグ用のが欲しくはなるかも
  - 結局ズーム操作はエミュレーションすることになるのでは
- ctx 消したい(Singleton?)
- pipeline.add で同じUniformBufferな時に気をつけたい(Camera)
- キーボード入力 / タッチ入力を受け取る
  - https://rustwasm.github.io/docs/wasm-bindgen/examples/paint.html
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
- zoom-in/outの解像度耐えたい
  - pinch-in/out も
  - window.visualViewport
  - cssの方でscaleいじれば強引にいけそう
- Async Computeしたい
  - tf
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
      let html_layer = core.html_layer();
      let text = format!("{} ms", core.processed_time());
      html_layer.set_text_content(Some(&text));
    }
  }
}
