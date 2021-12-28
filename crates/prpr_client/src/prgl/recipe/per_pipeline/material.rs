use super::*;

crate::shader_attr! {
  struct PbrAttribute {
    albedo_color: vec3
    roughness: float
  }
  mapping PbrMapping {
    normal_map: sampler2D,
  }
}
pub struct PbrMaterial {
  ubo: Arc<UniformBuffer<PbrAttribute>>,
  mapping: Arc<TextureMapping<PbrMapping>>,
}
impl PbrMaterial {
  pub fn new() -> Self {
    let default_normal_map = Arc::new(Texture::new_rgba_map(4, 4, |_, _| {
      Vec4::new(0.0, 0.0, 1.0, 0.0)
    }));
    Self {
      ubo: Arc::new(UniformBuffer::new(PbrAttribute {
        albedo_color: Vec3::ONE,
        roughness: 0.0,
      })),
      mapping: Arc::new(TextureMapping::new(PbrMapping {
        normal_map: default_normal_map,
      })),
    }
  }
}
impl PipelineBindable for PbrMaterial {
  fn bind_pipeline(&self, pipeline: &mut Pipeline) {
    pipeline.add_texture_mapping(&self.mapping);
    pipeline.add_uniform_buffer(&self.ubo);
  }
}
