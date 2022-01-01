// hoge_client に逃がす前段階でのサンプル
use super::*;
use prgl::*;
use prhtml;

struct CasualScene {
  objects: Vec<TransformObject>,
  renderpass: ArcOwner<RenderPass>,
  camera: Camera,
  out_color: ArcOwner<Texture>,
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
        void main() {
          mat4 mvp_mat = view_proj_mat * model_mat;
          gl_Position = mvp_mat * vec4(position, 1.0);
          mat4 it_mvp_mat = transpose(inverse(mvp_mat));
          in_normal = (it_mvp_mat * vec4(normal, 0.0)).xyz;
        }
      },
      fs_attr: { in_normal: vec3 },
      fs_code: {
        void main() {
          // texture(normal_map, vec2(0.5, 0.5)).rgb
          out_color = vec4(in_normal + 0.5, 1.0);
        }
      }
      out_attr: { out_color: vec4 }
    }
  }
  pub fn new() -> Self {
    // renderpass
    let mut camera = Camera::new();
    camera.write().camera_pos = Vec3::X * 5.0;
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
    let shape1 = Shape::new_cube();
    let shape2 = Shape::new_sphere(20, 20);
    let mut objects = Vec::new();
    const COUNT: u32 = 10;
    for x in 0..COUNT {
      for y in 0..COUNT {
        for z in 0..COUNT {
          let mut object = TransformObject::new();
          if rand::XorShift128::global().uniform() < 0.5 {
            object.pipeline().add(&shape1);
          } else {
            object.pipeline().add(&shape2);
          }
          object.pipeline().add(&material);
          object.pipeline().add(&shader);
          object.set_translate(
            Vec3::new(
              x as f32 - (COUNT as f32) * 0.5,
              y as f32 - (COUNT as f32) * 0.5,
              z as f32 - (COUNT as f32) * 0.5,
            ),
            Why::ByUser,
          );
          object.set_scale(Vec3::ONE * 0.72, Why::ByUser);
          renderpass.add(&object);
          objects.push(object);
        }
      }
    }
    let renderpass = ArcOwner::new(renderpass);
    RenderPassExecuter::add(&renderpass, CasualRenderPassOrder::Scene as usize);
    Self {
      objects,
      renderpass,
      camera,
      out_color,
    }
  }
}
impl NeedUpdate for CasualScene {
  fn update(&mut self) {
    if input::Mouse::state(input::MouseState::IsDown) {
      self.camera.write().rotate_self_fixed(Vec2::new(
        input::Mouse::dx() as f32 * 0.01,
        -input::Mouse::dy() as f32 * 0.01,
      ));
    }
    self.camera.write().dolly_with_mask(
      Vec3::new(
        input::Mouse::wheel_dx() as f32 * 0.005,
        0.0,
        -input::Mouse::wheel_dy() as f32 * 0.005,
      ),
      [true, false, true],
    );
    let f = Time::frame() as f32;
    for object in &mut self.objects {
      object.set_rotation(Quat::from_rotation_y(f * 0.01), Why::ByAnimation);
      object.set_translate(
        Vec3::new(0.0, 0.15 * (f * 0.013).sin(), 0.0),
        Why::ByAnimation,
      );
      object.set_scale(Vec3::ONE * (1.0 + 0.01 * (f * 0.1).sin()), Why::ByAnimation);
    }
    // adjust viewport
    let viewport = system::WholeScreen::viewport();
    self.camera.write().aspect_ratio = viewport.aspect_ratio();
    self.renderpass.write().set_viewport(Some(&viewport));
  }
}

crate::shader_attr! {
  mapping CasualPostEffectMapping {
    src_color: sampler2D,
  }
}
struct CasualPostEffect {
  renderpass: ArcOwner<RenderPass>,
  out_color: ArcOwner<Texture>,
}
impl CasualPostEffect {
  pub fn shader() -> ShaderTemplate {
    crate::shader_template! {
      attrs: [CasualPostEffectMapping],
      vs_attr: FullScreenVertex,
      vs_code: {
        void main () {
          gl_Position = vec4(position, 0.5, 1.0);
        }
      },
      fs_attr: {},
      fs_code: {
        void main() {
          ivec2 iuv = ivec2(gl_FragCoord.x, gl_FragCoord.y);
          vec4 base = texelFetch(src_color, iuv, 0).rgba;
          vec3 rgb = base.rgb;
          if (base.a < 0.5) {
            int ok = 0;
            for (int len = 1; len <= 5; ++len) {
              for (int dx = -1; dx <= 1; dx++) {
                for (int dy = -1; dy <= 1; dy++) {
                  vec4 fetch = texelFetch(src_color, iuv + ivec2(dx, dy) * len, 0);
                  if (fetch.a > 0.5) {
                    rgb = vec3(0.0, 0.0, 0.0);
                    ok = 1;
                  }
                }
              }
            }
            if (ok == 0) {
              discard;
            }
          } else {
            float gray = (base.r + base.g + base.b) * 0.333;
            rgb = vec3(0.7,1.2,1.8) * gray;
          }
          out_color = vec4(rgb, 1.0);
        }
      }
      out_attr: { out_color: vec4 }
    }
  }
  pub fn new(src_color: &dyn ReplicaTrait<Texture>) -> Self {
    let mut renderpass = RenderPass::new();
    let mut pipeline = FullScreen::new_pipeline();
    let shader = MayShader::new(CasualPostEffect::shader());
    // system::log::info(format!("{}", shader));
    pipeline.add(&shader);
    pipeline.add(&ArcOwner::new(TextureMapping::new(
      CasualPostEffectMapping {
        src_color: src_color.clone_reader(),
      },
    )));
    let out_color = TextureRecipe::new_fullscreen(PixelFormat::R8G8B8A8);
    renderpass.set_color_target(Some(&out_color));
    renderpass.set_clear_color(Some(Vec4::new(0.3, 0.3, 0.3, 1.0)));
    renderpass.own_pipeline(pipeline);
    let renderpass = ArcOwner::new(renderpass);
    RenderPassExecuter::add(&renderpass, CasualRenderPassOrder::PostEffect as usize);
    Self {
      renderpass,
      out_color,
    }
  }
}
impl NeedUpdate for CasualPostEffect {
  fn update(&mut self) {
    let viewport = system::WholeScreen::viewport();
    self.renderpass.write().set_viewport(Some(&viewport));
  }
}

fn apply_style(pane: &prhtml::Pane) {
  let gradation = prhtml::Gradation::Linear(
    0.0,
    vec![Vec4::new(0.4, 0.8, 0.9, 0.2), Vec4::new(0.4, 0.8, 0.9, 0.8)],
  );
  pane.set_padding(1.5);
  pane.set_align(prhtml::Align::Center);
  // pane.set_filter_blur(Some(0.1), Why::ByUser);
  pane.set_border_color(Vec4::new(0.4, 0.8, 0.9, 0.8));
  pane.set_border_radius(1.4);
  pane.set_border_width(0.4);
  pane.set_border_style(prhtml::BorderStyle::Solid);
  pane.set_box_shadow(0.5, 0.5, 0.5, Vec4::new(0.4, 0.8, 0.9, 0.4));
  pane.set_background_gradation(&gradation);
  pane.set_text_size(2.4);
  // pane.set_text_line_height(2.5);
  // pane.set_text_letter_spacing(0.1);
  pane.set_text_color(Vec4::new(0.1, 0.1, 0.1, 0.8));
  pane.set_text_shadow(0.5, 0.5, 1.0, Vec4::new(0.2, 0.4, 0.45, 0.8));
  pane.set_text_bold(true);
  // pane.set_text_italic(true);
  // pane.set_cursor(prhtml::Cursor::Pointer);
}

struct Pane1 {
  pane: prhtml::Pane,
  text: prhtml::Text,
}
impl Pane1 {
  fn new() -> Self {
    let mut pane = prhtml::Pane::new(prhtml::PaneFitPoint::LeftTop, 12.5, 12.5);
    pane.set_max_width(Some(12.5));
    pane.set_min_width(Some(12.5));
    pane.set_translate(Vec2::Y, Why::ByCustomStyle);
    apply_style(&pane);
    let text = prhtml::Text::new(&pane, "");
    Self { pane, text }
  }
}
impl NeedUpdate for Pane1 {
  fn update(&mut self) {
    let text = format!(
      "{} ms\n {} s",
      Time::processed_milli_sec(),
      (Time::now_milli_sec() / 1000.0) as i32,
    );
    self.text.set_text(&text);
    let f = Time::now_milli_sec() as f32 * 3.141592 * 120.0 / 60000.0 * 2.0;
    self.pane.set_scale(1.0 + 0.02 * f.sin(), Why::ByAnimation);
    self
      .pane
      .set_filter_huerotate(Some(f.to_degrees()), Why::ByUser);
    self.pane.update();
  }
}

struct Pane2 {
  pane: prhtml::Pane,
  text1: prhtml::Text,
  // hr1: prhtml::Hr,
}
impl Pane2 {
  fn new() -> Self {
    let mut pane = prhtml::Pane::new(prhtml::PaneFitPoint::Bottom, 80.0, 30.0);
    pane.set_max_width(Some(120.0));
    pane.set_translate(-Vec2::Y, Why::ByStyle);
    apply_style(&pane);
    pane.set_padding_x(3.0);
    prhtml::Fa::owned(&mut pane, "fab fa-twitter");
    let text1 = prhtml::Text::new(&pane, "");
    // text1.set_href(Some("https://example.com"));
    // let hr1 = prhtml::Hr::new(&pane); //, 2.4);
    prhtml::Text::owned(&mut pane, &rand::XorShift128::global().asciis(40));
    prhtml::Text::owned(&mut pane, "\n");
    prhtml::IFrame::owned(&mut pane, None, Some(30.0), "https://www.openstreetmap.org/export/embed.html?bbox=-0.004017949104309083%2C51.47612752641776%2C0.00030577182769775396%2C51.478569861898606&layer=mapnik");
    Self { pane, text1 }
  }
}
impl NeedUpdate for Pane2 {
  fn update(&mut self) {
    self.text1.set_text(&format!(
      "hello! {} frame\n mouse:({}, {}) \nwheel:({}, {})\n",
      Time::frame(),
      input::Mouse::x(),
      input::Mouse::y(),
      input::Mouse::wheel_dx(),
      input::Mouse::wheel_dy(),
    ));
    self.pane.update();
  }
}

pub fn sample_world() {
  js::console::log("create prpr world !!");
  let scene = CasualScene::new();
  let posteffect = CasualPostEffect::new(&scene.out_color);
  let surface = Surface::new(&posteffect.out_color);
  Updater::own(scene);
  Updater::own(posteffect);
  Updater::own(surface);
  Updater::own(Pane1::new());
  Updater::own(Pane2::new());
}
/* TODO:
- pipeline.add で同じUniformBufferな時に気をつけたい(Camera)
  - dummy texture 作っておきたい(bindが変わるので)
- particle
  - draw instanced
  - transform feedback
  - overlay
- z-index
  - 正: Pane
  - 負: 3Dシーンのものよりは上だが Paneよりは下のもの
- html
  - API -  WebMIDI, WebAudio, Video
  - 3Dシーン上に配置したい
  - Tween 実装したい（消える予定のUpdater）
    - clicked ?
  - cssの力は頼らない
    - 実態と乖離するので。
    - 本当に必要なのは hover + click + mousedown + scroll
    - 3Dシーンでも同じものが同じ仕組みで設定できるように
- Condition-Variable でいい感じにイベントドリブンにもできないか
  - NeedUpdateを消せないか
  - Findできないのつらい
- transform の子
- transform-object にパイプラインが一つしかない
  - shadow や selection で困る
// - 9FitPane
// - Table / FlexBox(Area)
// - img, video, audio,
// - button, input, textarea
//   meter(slider, progress),
//   select datalist fieldset optgroup+option
//   bulma-input? / slider? tooltip?
//   Material Design?
// - menu / tab
// SVG(いろいろなコンテンツ用)
// - chart-js
// - 文書用： Hr, A, H1~H6, BlockQuote, Li, Ol, Pre, Ul, Code

- ShaderTemplate持ち運べるように
- texture2darray, texture3d 対応する
  - texture として扱いたい？
    - https://ics.media/web3d-maniacs/webgl2_texture2darray/
    - https://ics.media/web3d-maniacs/webgl2_texture3d/
  - texStorage2D
    - https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/copyBufferSubData
    - https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D
  - https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/WebGL_best_practices#teximagetexsubimage_uploads_esp._videos_can_cause_pipeline_flushes
- renderbuffer
  - MSAA: https://ics.media/web3d-maniacs/webgl2_renderbufferstoragemultisample/
  - https://github.com/WebGLSamples/WebGL2Samples/blob/master/samples/fbo_multisample.html
  - mipmap がなぜかはいっている？
  - https://webglreport.com/?v=2 (MAX INFO)
- State
  - Scissor
  - Coverage Dither
  - Alpha Blend
- client_wait_sync ?
  - https://ics.media/entry/19043/
  - https://inside.pixiv.blog/petamoriken/5853
  - 描画だけをメインスレッドにすればいいかも？
  - https://rustwasm.github.io/wasm-bindgen/examples/wasm-in-web-worker.html
*/
