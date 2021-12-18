use super::*;

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

// BlendState
// ColorState
// CullState
// DepthState
// CoverageState
// PolygonOffsetState
// StencilState
// Scissor
// Viewport
