use super::*;
pub mod buffer;
pub use self::buffer::*;
pub mod shader;
pub use self::shader::*;
pub mod attribute;
pub use self::attribute::*;
pub mod shader_macro;
pub use self::shader_macro::*;
pub mod pipeline_state;
pub use self::pipeline_state::*;

// raw 以下のコードのstructの制約
// - GlContext を自身で持たない
// - GpuリソースをDropしない
// - web_sys::*を公開する
