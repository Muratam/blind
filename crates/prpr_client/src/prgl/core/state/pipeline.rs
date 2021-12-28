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
  pub fn apply(&self, topology: PrimitiveToporogy) {
    let topology = topology as u32;
    let ctx = Instance::ctx();
    match self {
      DrawCommand::Draw { first, count } => {
        ctx.draw_arrays(topology, *first, *count);
      }
      DrawCommand::DrawIndexed { first, count } => {
        assert_type_eq!(u32, IndexBufferType);
        ctx.draw_elements_with_i32(topology, *count, gl::UNSIGNED_INT, *first);
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
  pub fn apply(&self) {
    let ctx = Instance::ctx();
    if *self == CullMode::None {
      ctx.disable(gl::CULL_FACE);
    } else {
      ctx.enable(gl::CULL_FACE);
      ctx.cull_face(*self as u32);
    }
  }
}

#[derive(Clone, Copy, PartialEq)]
pub enum DepthFunc {
  Never = gl::NEVER as isize,
  Less = gl::LESS as isize,
  Equal = gl::EQUAL as isize,
  LEqual = gl::LEQUAL as isize,
  Greater = gl::GREATER as isize,
  NotEqual = gl::NOTEQUAL as isize,
  GEqual = gl::GEQUAL as isize,
  Always = gl::ALWAYS as isize,
}
impl DepthFunc {
  pub fn apply(&self) {
    let ctx = Instance::ctx();
    if *self == DepthFunc::Always {
      ctx.disable(gl::DEPTH_TEST);
    } else {
      ctx.enable(gl::DEPTH_TEST);
      ctx.depth_func(*self as u32);
    }
  }
}
