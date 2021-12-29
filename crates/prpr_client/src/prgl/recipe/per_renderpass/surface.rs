use super::*;

crate::shader_attr! {
  mapping SurfaceMapping {
    src_color: sampler2D,
  }
}
pub struct Surface {
  renderpass: Main<prgl::RenderPass>,
}
// NOTE: 利便性のために最後のキャンバス出力をコピーで済ますもの
// 最後ダイレクトに書いたほうが無駄な工程が減る
impl Surface {
  fn shader() -> ShaderTemplate {
    crate::shader_template! {
      attrs: [SurfaceMapping],
      vs_attr: FullScreenVertex,
      vs_code: { gl_Position = vec4(position, 0.5, 1.0); },
      fs_attr: {},
      fs_code: { out_color = texelFetch(src_color, ivec2(gl_FragCoord.xy), 0); }
      out_attr: { out_color: vec4 }
    }
  }
  pub fn new(src_color: &dyn ReplicaTrait<Texture>) -> Self {
    let mut renderpass = RenderPass::new();
    renderpass.set_use_default_buffer(true);
    let mut pipeline = FullScreen::new_pipeline();
    pipeline.add(&MayShader::new(Self::shader()));
    pipeline.add(&Main::new(TextureMapping::new(SurfaceMapping {
      src_color: src_color.clone_replica(),
    })));
    renderpass.own_pipeline(pipeline);
    let renderpass = Main::new(renderpass);
    RenderPassExecuter::add(&renderpass, usize::MAX);
    Self { renderpass }
  }
}

impl Updater for Surface {
  fn update(&mut self) {
    let viewport = prgl::Instance::viewport();
    self.renderpass.write().set_viewport(Some(&viewport));
  }
}
