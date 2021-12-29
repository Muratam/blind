// hoge_client に逃がす前段階でのサンプル
use super::*;
use prgl;

struct CasualScene {
  objects: Vec<prgl::TransformObject>,
  renderpass: Owner<prgl::RenderPass>,
  camera: prgl::Camera,
  out_color: Arc<Texture>,
}
enum CasualRenderPassOrder {
  Scene,
  PostEffect,
}
impl CasualScene {
  pub fn shader() -> ShaderTemplate {
    crate::shader_template! {
      attrs: [
        CameraAttribute, TransformAttribute, PbrAttribute,
        PbrMapping
      ],
      vs_attr: ShapeVertex,
      vs_code: {
        gl_Position = view_proj_mat * model_mat * vec4(position, 1.0);
        in_position = position;
      },
      fs_attr: { in_position: vec3 },
      fs_code: {
        out_color = vec4(texture(normal_map, vec2(0.5, 0.5)).rgb + in_position, 1.0);
      }
      out_attr: { out_color: vec4 }
    }
  }
  pub fn new() -> Self {
    // renderpass
    let camera = Camera::new();
    let mut renderpass = RenderPass::new();
    renderpass.set_clear_color(Some(Vec4::new(1.0, 1.0, 1.0, 0.0)));
    renderpass.set_clear_depth(Some(1.0));
    renderpass.add(&camera);
    let out_color = TextureRecipe::new_fullscreen(PixelFormat::R8G8B8A8);
    renderpass.set_color_target(Some(&out_color));
    let src_depth = TextureRecipe::new_fullscreen_depth();
    renderpass.set_depth_target(Some(&src_depth));
    // objects
    let shader = MayShader::new(CasualScene::shader());
    let material = PbrMaterial::new();
    let shape = Shape::new_cube();
    let mut objects = Vec::new();
    const COUNT: u32 = 4;
    for x in 0..COUNT {
      for y in 0..COUNT {
        for z in 0..COUNT {
          let mut object = TransformObject::new();
          object.pipeline.write().add(&shape);
          object.pipeline.write().add(&material);
          object.pipeline.write().add(&shader);
          object.transform.write_lock().translate = Vec3::new(
            x as f32 - (COUNT as f32) * 0.5,
            y as f32 - (COUNT as f32) * 0.5,
            z as f32 - (COUNT as f32) * 0.5,
          );
          object.transform.write_lock().scale = Vec3::ONE * 0.72;
          renderpass.add_pipeline(&object.pipeline);
          objects.push(object);
        }
      }
    }
    let renderpass = Owner::new(renderpass);
    RenderPassExecuter::global_write_lock().add(&renderpass, CasualRenderPassOrder::Scene as usize);
    Self {
      objects,
      renderpass,
      camera,
      out_color,
    }
  }
  pub fn update(&mut self) {
    let frame = Time::frame();
    let f = (frame as f32) / 100.0;
    self.camera.write_lock().camera_pos = Vec3::new(f.sin(), f.cos(), f.cos()) * 5.0;
    for object in &mut self.objects {
      object.transform.write_lock().rotation *= Quat::from_rotation_y(3.1415 * 0.01);
    }
    // adjust viewport
    let viewport = prgl::Instance::viewport();
    self.camera.write_lock().aspect_ratio = viewport.aspect_ratio();
    self.renderpass.write().set_viewport(Some(&viewport));
  }
}

crate::shader_attr! {
  mapping CasualPostEffectMapping {
    src_color: sampler2D,
  }
}
struct CasualPostEffect {
  renderpass: Owner<prgl::RenderPass>,
  out_color: Arc<Texture>,
}
impl CasualPostEffect {
  pub fn shader() -> ShaderTemplate {
    crate::shader_template! {
      attrs: [CasualPostEffectMapping],
      vs_attr: FullScreenVertex,
      vs_code: {
        gl_Position = vec4(position, 0.5, 1.0);
      },
      fs_attr: {},
      fs_code: {
        ivec2 iuv = ivec2(gl_FragCoord.x, gl_FragCoord.y);
        vec4 base = texelFetch(src_color, iuv, 0).rgba;
        vec3 rgb = base.rgb;
        if (base.a < 0.5) {
          for (int len = 1; len <= 5; len += 1) {
            for (int dx = -1; dx <= 1; dx+=1) {
              for (int dy = -1; dy <= 1; dy+=1) {
                if (texelFetch(src_color, iuv + ivec2(dx, dy) * len, 0).a > 0.5) {
                  rgb = vec3(0.0, 0.0, 0.0);
                }
              }
            }
          }
        }
        out_color = vec4(rgb, 1.0);
      }
      out_attr: { out_color: vec4 }
    }
  }
  pub fn new(src_color: &Arc<Texture>) -> Self {
    let mut renderpass = RenderPass::new();
    let mut pipeline = FullScreen::new_pipeline();
    pipeline.add(&MayShader::new(CasualPostEffect::shader()));
    pipeline.add(&Arc::new(TextureMapping::new(CasualPostEffectMapping {
      src_color: src_color.clone(),
    })));
    let out_color = TextureRecipe::new_fullscreen(PixelFormat::R8G8B8A8);
    renderpass.set_color_target(Some(&out_color));
    renderpass.own_pipeline(pipeline);
    let renderpass = Owner::new(renderpass);
    RenderPassExecuter::global_write_lock()
      .add(&renderpass, CasualRenderPassOrder::PostEffect as usize);
    Self {
      renderpass,
      out_color,
    }
  }
  pub fn update(&mut self) {
    let viewport = prgl::Instance::viewport();
    self.renderpass.write().set_viewport(Some(&viewport));
  }
}

pub struct SampleScene {
  scene: CasualScene,
  posteffect: CasualPostEffect,
  surface: Surface,
}
impl SampleScene {
  pub fn new() -> Self {
    {
      let mut x = Owner::new(1);
      *x.write() = 10;
      *x.write() = 20;
      {
        let r = x.read();
        let r2 = x.read();
        let n = *r + *r2;
      }
      let mut w = x.write();
      *w = 10;
      // let mut w2 = x.write();
      // *w = 40;
    }

    let scene = CasualScene::new();
    let posteffect = CasualPostEffect::new(&scene.out_color);
    let surface = Surface::new(&posteffect.out_color);
    Self {
      scene,
      posteffect,
      surface,
    }
  }
}
impl Updater for SampleScene {
  fn update(&mut self) {
    self.scene.update();
    self.posteffect.update();
    self.surface.update();
  }
}

pub struct SampleSystem {
  scene: SampleScene,
}
impl System for SampleSystem {
  fn new(core: &Core) -> Self {
    Self {
      scene: SampleScene::new(),
    }
  }
  fn update(&mut self, core: &Core) {
    self.scene.update();
    self.render_sample(core);
  }
}
/* TODO:
- Ownerパターンに変えたい場所を変える
  - Arc<RwLock<>>
  - Arc<>
  - write_lock()
- renderbuffer
  - MSAA: https://ics.media/web3d-maniacs/webgl2_renderbufferstoragemultisample/
  - mipmap がなぜかはいっている？
- ShaderTemplate -> void main()
- 複数のカメラで描画したい
  - 同じのを別カメラで２回やればOK
  - Selection はカメラから？
  - 指操作はカメラに紐付ける？
  - デバッグ用のが欲しくはなるかも
  - 結局ズーム操作はエミュレーションすることになるのでは
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
  - transform feedback
  - draw instanced
- State
  - Scissor
  - ReverseZ
  - Coverage Dither
  - Alpha Blend
*/

impl SampleSystem {
  fn render_sample(&mut self, core: &Core) {
    if false {
      // 多分使用することはない
      let ctx = core.main_2d_context();
      use std::f64::consts::PI;
      ctx.begin_path();
      ctx.arc(75.0, 75.0, 50.0, 0.0, PI * 2.0).ok();
      ctx.move_to(110.0, 75.0);
      ctx.stroke();
    }
    // TODO: HTML
    {
      let html_layer = core.html_layer();
      let text = format!("{} ms", Time::processed_milli_sec());
      html_layer.set_text_content(Some(&text));
    }
  }
}
