use super::*;

crate::shader_attr! {
  struct SurfaceOffset {
    surface_offset: vec2,
    surface_dummy: vec2,
  }
  mapping SurfaceMapping {
    src_color: sampler2D,
  }
}
pub struct Surface {
  ubo: ArcOwner<UniformBuffer<SurfaceOffset>>,
  renderpass: ArcOwner<prgl::RenderPass>,
  mapping: ArcOwner<TextureMapping<SurfaceMapping>>,
}
// NOTE: 利便性のために最後のキャンバス出力をコピーで済ますもの
// 最後ダイレクトに書いたほうが無駄な工程が減る
impl Surface {
  fn shader() -> ShaderTemplate {
    crate::shader_template! {
      attrs: [SurfaceMapping, SurfaceOffset],
      vs_attr: FullScreenVertex,
      vs_code: { void main() {gl_Position = vec4(position, 0.5, 1.0);} },
      fs_attr: {},
      fs_code: { void main() {out_color = texelFetch(src_color, ivec2(gl_FragCoord.xy + surface_offset.xy), 0);} }
      out_attr: { out_color: vec4 }
    }
  }
  pub fn new() -> Self {
    let mut renderpass = RenderPass::new();
    renderpass.set_use_default_buffer(true);
    let mut pipeline = FullScreen::new_pipeline();
    pipeline.add(&MayShader::new(Self::shader()));
    let mapping = ArcOwner::new(TextureMapping::new(SurfaceMapping {
      src_color: TextureRecipe::new_dummy().clone_reader(),
    }));
    pipeline.add(&mapping);
    let ubo = ArcOwner::new(UniformBuffer::new(SurfaceOffset {
      surface_offset: Vec2::ZERO,
      surface_dummy: Vec2::ZERO,
    }));
    pipeline.add(&ubo);
    renderpass.own_pipeline(pipeline);
    let renderpass = ArcOwner::new(renderpass);
    RenderPassExecuter::add(&renderpass, usize::MAX);
    Self {
      ubo,
      renderpass,
      mapping,
    }
  }
  pub fn set_texture(&mut self, src_color: &dyn ArcReaderTrait<Texture>) {
    self.mapping.write().src_color = src_color.clone_reader();
  }
}

impl NeedUpdate for Surface {
  fn update(&mut self) {
    let mut viewport = system::WholeScreen::viewport();
    self.ubo.write().surface_offset = Vec2::new(viewport.x as f32, viewport.y as f32);
    viewport.x = 0;
    viewport.y = 0;
    self.renderpass.write().set_viewport(Some(&viewport));
  }
}
