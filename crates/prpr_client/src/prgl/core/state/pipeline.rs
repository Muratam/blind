use super::*;

#[derive(Clone, Copy)]
pub enum PrimitiveToporogy {
  Points = gl::POINTS as isize,
  LineStrip = gl::LINE_STRIP as isize,
  LineLoop = gl::LINE_LOOP as isize,
  Lines = gl::LINES as isize,
  TriangleStrip = gl::TRIANGLE_STRIP as isize,
  TriangleFan = gl::TRIANGLE_FAN as isize,
  Triangles = gl::TRIANGLES as isize,
}
pub enum DrawCommand {
  Draw { first: i32, count: i32 },
  // DrawInstanced {
  //   first: i32,
  //   count: i32,
  //   instance_count: i32,
  // },
  DrawIndexed { first: i32, count: i32 },
  // DrawIndexedInstanced {
  //   first: i32,
  //   count: i32,
  //   instance_count: i32,
  // },
}
impl DrawCommand {
  pub fn apply(&self, gl: &ArcGlContext, topology: PrimitiveToporogy) {
    let topology = topology as u32;
    match self {
      DrawCommand::Draw { first, count } => {
        gl.draw_arrays(topology, *first, *count);
      }
      DrawCommand::DrawIndexed { first, count } => {
        assert_type_eq!(u32, IndexBufferType);
        gl.draw_elements_with_i32(topology, *count, gl::UNSIGNED_INT, *first);
      }
    }
  }
}

#[derive(Clone, Copy, PartialEq)]
pub enum CullMode {
  None = 0 as isize,
  Front = gl::FRONT as isize,
  Back = gl::BACK as isize,
  All = gl::FRONT_AND_BACK as isize,
}
impl CullMode {
  pub fn apply(&self, gl: &ArcGlContext) {
    if *self == CullMode::None {
      gl.disable(gl::CULL_FACE);
    } else {
      gl.enable(gl::CULL_FACE);
      gl.cull_face(*self as u32);
    }
  }
}
// BlendState
// ColorState
// CullState
// DepthState
// CoverageState
// PolygonOffsetState
// StencilState
// Scissor
// Viewport
