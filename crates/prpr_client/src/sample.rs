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
        mat4 mvp_mat = view_proj_mat * model_mat;
        gl_Position = mvp_mat * vec4(position, 1.0);
        mat4 it_mvp_mat = transpose(inverse(mvp_mat));
        in_normal = (it_mvp_mat * vec4(normal, 0.0)).xyz;
      },
      fs_attr: { in_normal: vec3 },
      fs_code: {
        // texture(normal_map, vec2(0.5, 0.5)).rgb
        out_color = vec4(in_normal + 0.5, 1.0);
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
    let shape2 = Shape::new_sphere(5, 5);
    let mut objects = Vec::new();
    const COUNT: u32 = 5;
    for x in 0..COUNT {
      for y in 0..COUNT {
        for z in 0..COUNT {
          let mut object = TransformObject::new();
          if (x ^ y ^ z) & 1 == 0 {
            object.pipeline.write().add(&shape1);
          } else {
            object.pipeline.write().add(&shape2);
          }
          object.pipeline.write().add(&material);
          object.pipeline.write().add(&shader);
          object.transform.write().translate = Vec3::new(
            x as f32 - (COUNT as f32) * 0.5,
            y as f32 - (COUNT as f32) * 0.5,
            z as f32 - (COUNT as f32) * 0.5,
          );
          object.transform.write().scale = Vec3::ONE * 0.72;
          renderpass.add_pipeline(&object.pipeline);
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
impl system::Updatable for CasualScene {
  fn update(&mut self) {
    let frame = Time::frame();
    let f = (frame as f32) / 100.0;
    if input::Mouse::state(input::MouseState::IsDown) {
      self.camera.write().rotate_self_fixed(Vec2::new(
        input::Mouse::dx() as f32 * 0.01,
        -input::Mouse::dy() as f32 * 0.01,
      ));
    }
    self.camera.write().dolly(Vec3::new(
      input::Mouse::wheel_dx() as f32 * 0.005,
      0.0,
      -input::Mouse::wheel_dy() as f32 * 0.005,
    ));

    for object in &mut self.objects {
      object.transform.write().rotation *= Quat::from_rotation_y(0.0001_f32.to_degrees());
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
        gl_Position = vec4(position, 0.5, 1.0);
      },
      fs_attr: {},
      fs_code: {
        ivec2 iuv = ivec2(gl_FragCoord.x, gl_FragCoord.y);
        vec4 base = texelFetch(src_color, iuv, 0).rgba;
        vec3 rgb = base.rgb;
        if (base.a < 0.5) {
          rgb = vec3(1.0, 1.0, 1.0);
          for (int len = 1; len <= 5; len += 1) {
            for (int dx = -1; dx <= 1; dx+=1) {
              for (int dy = -1; dy <= 1; dy+=1) {
                vec4 fetch = texelFetch(src_color, iuv + ivec2(dx, dy) * len, 0);
                if (fetch.a > 0.5) {
                  rgb = vec3(0.0, 0.0, 0.0);
                }
              }
            }
          }
        } else {
          float gray = (base.r + base.g + base.b) * 0.333;
          rgb = vec3(1.0,1.0,1.0) * gray;
        }
        out_color = vec4(rgb, 1.0);
      }
      out_attr: { out_color: vec4 }
    }
  }
  pub fn new(src_color: &dyn ReplicaTrait<Texture>) -> Self {
    let mut renderpass = RenderPass::new();
    let mut pipeline = FullScreen::new_pipeline();
    pipeline.add(&MayShader::new(CasualPostEffect::shader()));
    pipeline.add(&ArcOwner::new(TextureMapping::new(
      CasualPostEffectMapping {
        src_color: src_color.clone_reader(),
      },
    )));
    let out_color = TextureRecipe::new_fullscreen(PixelFormat::R8G8B8A8);
    renderpass.set_color_target(Some(&out_color));
    renderpass.own_pipeline(pipeline);
    let renderpass = ArcOwner::new(renderpass);
    RenderPassExecuter::add(&renderpass, CasualRenderPassOrder::PostEffect as usize);
    Self {
      renderpass,
      out_color,
    }
  }
}
impl Updatable for CasualPostEffect {
  fn update(&mut self) {
    let viewport = system::WholeScreen::viewport();
    self.renderpass.write().set_viewport(Some(&viewport));
  }
}
fn apply_style(f_box: &prhtml::FloatingBox) {
  let gradation = prhtml::Gradation::Linear(
    0.0,
    vec![Vec4::new(0.4, 0.8, 0.9, 0.2), Vec4::new(0.4, 0.8, 0.9, 0.8)],
  );
  f_box.set_padding(1.5);
  f_box.set_filter(&vec![prhtml::Filter::Blur(0.1)]);
  f_box.set_border_color(Vec4::new(0.4, 0.8, 0.9, 0.8));
  f_box.set_border_radius(1.4);
  f_box.set_border_width(0.4);
  f_box.set_border_style(prhtml::BorderStyle::Solid);
  f_box.set_background_shadow(0.5, 0.5, 0.5, Vec4::new(0.4, 0.8, 0.9, 0.4));
  f_box.set_background_gradation(&gradation);
  f_box.set_text_size(2.4);
  f_box.set_text_line_height(2.5);
  f_box.set_text_letter_spacing(0.1);
  f_box.set_text_color(Vec4::new(0.1, 0.1, 0.1, 0.8));
  f_box.set_text_shadow(0.5, 0.5, 1.0, Vec4::new(0.2, 0.4, 0.45, 0.8));
  f_box.set_text_bold(true);
  f_box.set_text_italic(true);
}
struct Float1 {
  elem: prhtml::FloatingBox,
}
impl Updatable for Float1 {
  fn update(&mut self) {
    let text = format!("{} ms", Time::processed_milli_sec());
    self.elem.set_text_debug(&text);
    self.elem.update();
  }
}
struct Float2 {
  elem: prhtml::FloatingBox,
}
impl Updatable for Float2 {
  fn update(&mut self) {
    self.elem.set_text_debug("hello");
    self.elem.update();
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
  {
    let mut elem = prhtml::FloatingBox::new();
    elem.set_position(math::Vec2::new(-0.35, 0.35));
    elem.set_size(math::Vec2::ONE * 0.125);
    apply_style(&elem);
    Updater::own(Float1 { elem });
  }
  {
    let mut elem = prhtml::FloatingBox::new();
    elem.set_position(math::Vec2::new(0.0, -0.25));
    elem.set_size(math::Vec2::new(0.8, 0.30));
    apply_style(&elem);
    Updater::own(Float2 { elem });
  }
}
/* TODO:
- ShaderTemplate -> void main()
- pipeline.add で同じUniformBufferな時に気をつけたい(Camera)
- particle
  - draw instanced
  - transform feedback
  - overlay
- html
  - box-based system
    - 中心が0.0, 右上が正
    - width が自動縮小の場合：伸びる最大アスペクト比を指定
    -
  - table? fontawesome? iframe?(map?) bulma input? / slider? tooltip?
  - top menu? chart.js?
  - API -  WebMIDI, WebAudio, Video

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
  - ReverseZ
  - Coverage Dither
  - Alpha Blend
- client_wait_sync ?
  - https://ics.media/entry/19043/
  - https://inside.pixiv.blog/petamoriken/5853
  - 描画だけをメインスレッドにすればいいかも？
  - https://rustwasm.github.io/wasm-bindgen/examples/wasm-in-web-worker.html
*/
