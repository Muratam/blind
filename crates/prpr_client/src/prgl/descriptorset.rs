use super::*;

pub struct DescriptorSet {
  gl: Rc<GlContext>,
  raw_vao: Option<RawVao>,
}
