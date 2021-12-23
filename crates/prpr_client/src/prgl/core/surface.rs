use super::*;
/*
// 最終描画先
pub struct Surface {
  ctx: ArcGlContext,
  viewport: Rect<i32>, // no scissor (fullscreen surface)
  shader: Shader,
}
impl Surface {
  pub fn new(ctx: &ArcGlContext) -> Self {
    crate::shader_attr! {
      struct Vertex{ pos: vec2 }
      mapping Source { source : sampler2D, }
    }
    let template = crate::shader_template! {
      attrs: [Source],
      vs_attr: Vertex,
      fs_attr: {},
      out_attr: { out_color: vec4 }
      vs_code: {
        if (gl_VertexID == 0) {
          gl_Position = vec4(-1.0, 1.0, 0.0, 1.0);
        } else if (gl_VertexID == 1) {
          gl_Position = vec4(3.0, 1.0, 0.0, 1.0);
        } else {
          gl_Position = vec4(-1.0, -3.0, 0.0, 1.0);
        }
      },
      fs_code: {
        out_color = vec4(1.0, 0.0, 0.0, 1.0);
      }
    };
    let shader = Shader::new(ctx, template);
    // let v_buffer = VertexBuffer::new(ctx, vec![
    //     Vertex { pos: Vec2::new(-1.0, 1.0)},
    //     Vertex { pos: Vec2::new(3.0, 1.0) },
    //     Vertex { pos: Vec2::new(-1.0, -3.0) },
    // ]);
    // let vao = Vao::new_without_index_buffer(ctx, v_buffer);
    Self {
      ctx: ctx.clone(),
      viewport: Rect::new(0, 0, 1, 1),
    }
  }
  pub fn set_viewport(&mut self, viewport: &Rect<i32>) {
    self.viewport = viewport.clone();
  }
  pub fn swap(&self, texture: &Arc<Texture>) {
    self.shader.use_program();
    self.ctx.bind_framebuffer(gl::FRAMEBUFFER, None);
    self.ctx.bind_vertex_array(None);
    self.ctx.viewport(
      self.viewport.x,
      self.viewport.y,
      self.viewport.width,
      self.viewport.height,
    );
    self.ctx.scissor(
      self.viewport.x,
      self.viewport.y,
      self.viewport.width,
      self.viewport.height,
    );
    CullMode::None.apply(&self.ctx);
    DrawCommand::Draw { first: 0, count: 3 }.apply(&self.ctx, PrimitiveToporogy::Triangles);
  }
}
*/
