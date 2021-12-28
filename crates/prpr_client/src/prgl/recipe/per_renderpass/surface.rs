use super::*;

crate::shader_attr! {
  mapping SurfaceMapping {
    src_color: sampler2D,
  }
}
pub struct Surface {
  renderpass: RenderPass,
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
  // フルサイズのテクスチャであると想定できる
  pub fn new(src_color: &Arc<Texture>) -> Self {
    let mut renderpass = RenderPass::new();
    renderpass.set_use_default_buffer(true);
    let mut pipeline = FullScreen::new_pipeline();
    pipeline.add(&MayShader::new(Self::shader()));
    pipeline.add(&Arc::new(TextureMapping::new(SurfaceMapping {
      src_color: src_color.clone(),
    })));
    renderpass.own_pipeline(pipeline);
    Self { renderpass }
  }
  pub fn update(&mut self) {
    let viewport = prgl::Instance::viewport();
    self.renderpass.set_viewport(Some(&viewport));
  }
  pub fn draw(&mut self, cmd: &mut Command) {
    self.renderpass.draw(cmd, &DescriptorContext::nil());
  }
}
