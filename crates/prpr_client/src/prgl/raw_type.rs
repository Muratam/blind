use prpr::math::*;

// 実態として必要な情報を全て詰め込んだもの
pub const SWAP_COUNT: i64 = 2;
struct RawBuffer {
  need_swap: bool,
}
struct RawTexture {
  need_swap: bool,
}
struct RawSampler {}
struct RawExtensions {}
struct RawShader {}
enum RawDrawCommand {}
struct RawPipeline {
  is_enabled_depth_test: bool,
  draw_command: RawDrawCommand,
  // gl.depth_func(gl::LEQUAL);
}
