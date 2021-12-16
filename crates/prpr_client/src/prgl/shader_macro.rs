#[allow(non_camel_case_types)]
pub type float = f32;
#[allow(non_camel_case_types)]
pub type vec2 = glam::Vec2;
#[allow(non_camel_case_types)]
pub type vec3 = glam::Vec3;
#[allow(non_camel_case_types)]
pub type vec4 = glam::Vec4;
#[allow(non_camel_case_types)]
pub type uint = u32;
#[allow(non_camel_case_types)]
pub type uvec2 = (u32, u32);
#[allow(non_camel_case_types)]
pub type uvec3 = (u32, u32, u32);
#[allow(non_camel_case_types)]
pub type uvec4 = (u32, u32, u32, u32);
#[allow(non_camel_case_types)]
pub type mat4 = glam::Mat4; // align: 16B

#[macro_export]
macro_rules! shader_type_str {
  (uint) => {
    "uint"
  };
  (uvec2) => {
    "uvec2"
  };
  (uvec3) => {
    "uvec3"
  };
  (uvec4) => {
    "uvec4"
  };
  (float) => {
    "float"
  };
  (vec2) => {
    "vec2"
  };
  (vec3) => {
    "vec3"
  };
  (vec4) => {
    "vec4"
  };
  (mat4) => {
    "mat4"
  };
}
#[derive(Debug)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum ShaderType {
  uint(uint),
  uvec2(uvec2),
  uvec3(uvec3),
  uvec4(uvec4),
  float(float),
  vec2(vec2),
  vec3(vec3),
  vec4(vec4),
  mat4(mat4),
}
pub fn shader_vb_location(key: &str) -> u64 {
  use std::hash::{Hash, Hasher};
  let mut hasher = std::collections::hash_map::DefaultHasher::new();
  key.hash(&mut hasher);
  hasher.finish() & 0xfff
}

pub trait ShaderAttr
where
  Self: Sized,
{
  fn new() -> Self;
  // for shader code generation
  fn ub_code() -> &'static str;
  fn vs_in_code() -> String;
  fn vs_out_code() -> &'static str;
  fn fs_in_code() -> &'static str;
  fn fs_out_code() -> &'static str;
  // for bidndings
  fn ub_data(&self) -> &[u8];
  fn struct_size() -> usize;
  fn offsets() -> Vec<usize>;
  fn keys() -> Vec<&'static str>;
  fn values(&self) -> Vec<ShaderType>;
  // for dynamic loading
  fn find(&self, key: &str) -> Option<ShaderType>;
  fn from_hashmap(map: &std::collections::HashMap<String, ShaderType>)
    -> (Self, Vec<&'static str>); // self + not found keys
  fn to_hashmap(&self) -> std::collections::HashMap<String, ShaderType>;
}

#[macro_export]
macro_rules! shader_attr {
  ($( struct $s:ident { $( $k:ident : $v:ident $(,)?)* } $(;)?)*) => (
    $(
      #[derive(Default, Debug)]
      #[repr(C)]
      pub struct $s {
        $(pub $k : $v,)*
      }

      #[allow(unused_variables)]
      #[allow(unused_mut)]
      impl ShaderAttr for $s {
        fn new() -> Self { Default::default() }
        fn ub_code() -> &'static str {
          concat!(
            "layout (std140) uniform ", stringify!($s), " {\n",
              $("  ", shader_type_str!($v) ," ", stringify!($k), ";\n",)*
            "};"
          )
        }
        fn vs_in_code() -> String {
          format!(concat!(
            $("layout (location = {}) in ",shader_type_str!($v)," ",stringify!($k),";\n",)*
          ), $(shader_vb_location(stringify!($k)),)*)
        }
        fn vs_out_code() -> &'static str {
          concat!(
            $("out ", shader_type_str!($v) ," ", stringify!($k), ";\n",)*
          )
        }
        fn fs_in_code() -> &'static str {
          concat!($("in ", shader_type_str!($v) ," ", stringify!($k), ";\n",)*)
        }
        fn fs_out_code() -> &'static str {
          concat!($("out ", shader_type_str!($v) ," ", stringify!($k), ";\n",)*)
        }
        fn ub_data(&self) -> &[u8] {
          let u8_size = Self::struct_size();
          let ptr = self as *const $s as *const u8;
          unsafe { ::core::slice::from_raw_parts(ptr, u8_size) }
        }
        fn struct_size() -> usize {
          ::std::mem::size_of::<$s>()
        }
        fn offsets() -> Vec<usize> {
          let mut result = Vec::new();
          let dummy = ::core::mem::MaybeUninit::<Self>::uninit();
          let dummy_ptr = dummy.as_ptr();
          $(
            let member_ptr = unsafe{ ::core::ptr::addr_of!((*dummy_ptr).$k) };
            result.push(member_ptr as usize - dummy_ptr as usize);
          )*
          result
        }
        fn keys() -> Vec<&'static str> {
          let mut result = Vec::new();
          $(result.push(stringify!($k));)*
          result
        }
        fn values(&self) -> Vec<ShaderType> {
          let mut result = Vec::new();
          $(result.push(ShaderType::$v(self.$k));)*
          result
        }
        fn find(&self, key: &str) -> Option<ShaderType> {
          match key {
            $(stringify!($k) => Some(ShaderType::$v(self.$k)),)*
            _ => None,
          }
        }
        // for dynamic loading
        fn from_hashmap(map: &::std::collections::HashMap<String, ShaderType>) -> (Self, Vec<&'static str>) {
          let mut result = Self::new();
          let mut ignored = Vec::new();
          $(
            if let Some(ShaderType::$v(v)) = map.get(stringify!($k)) {
              result.$k = *v;
            } else {
              ignored.push(stringify!($k));
            }
          )*
          (result, ignored)
        }
        fn to_hashmap(&self) -> ::std::collections::HashMap<String, ShaderType> {
          let mut result = ::std::collections::HashMap::new();
          $(result.insert(String::from(stringify!($k)), ShaderType::$v(self.$k));)*
          result
        }
      }
      impl ::std::fmt::Display for $s {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
          write!(f, "struct {} {{\n", stringify!($s))?;
          $(write!(f, "  {}: {} = {:?}, \n", stringify!($k), stringify!($v), self.$k)?;)*
          write!(f, "}}")
        }
      }
    )*
  );
}
pub struct ShaderTemplate {
  vs_code_template: String,
  fs_code_template: String,
  pub vs_code_impl: String,
  pub fs_code_impl: String,
}
impl ShaderTemplate {
  pub fn new(vs_code_template: String, fs_code_template: String) -> Self {
    Self {
      vs_code_template,
      fs_code_template,
      vs_code_impl: String::from(""),
      fs_code_impl: String::from(""),
    }
  }
  pub fn vs_code(&self) -> String {
    format!("{}{}", self.vs_code_template, self.vs_code_impl)
  }
  pub fn fs_code(&self) -> String {
    format!("{}{}", self.fs_code_template, self.fs_code_impl)
  }
}

impl ::std::fmt::Display for ShaderTemplate {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    write!(
      f,
      "// vs define\n{}\n// vs impl\n{}\n\n// fs define\n{}\n// fs impl\n{}",
      self.vs_code_template, self.vs_code_impl, self.fs_code_template, self.fs_code_impl
    )
  }
}

#[macro_export]
macro_rules! shader_template_element {
  (version: $v:expr) => {
    ($v)
  };
  (precision_float: highp) => {
    "highp"
  };
  (precision_float: mediump) => {
    "mediump"
  };
  (precision_float: lowp) => {
    "lowp"
  };
  (vs_attr: $v:ident) => {
    $v::vs_in_code()
  };
  (fs_attr: $v:ident) => {{
    ($v::vs_out_code(), $v::fs_in_code())
  }};
  (fs_attr: $v:tt) => {{
    shader_attr!{struct FsAttr $v}
    (FsAttr::vs_out_code(), FsAttr::fs_in_code())
  }};
  (out_attr: $v:ident) => {
    $v::fs_out_code()
  };
  (out_attr: $v:tt) => {{
    shader_attr!{struct OutAttr $v}
    OutAttr::fs_out_code()
  }};
  (attrs: [$($v:ident),*]) => {{
    let mut s = String::new();
    $(s += $v::ub_code(); s += "\n";)*
    s
  }};
  (vs_code: $v:tt) => {
    concat!("void main() ", stringify!($v)).to_string()
  };
  (fs_code: $v:tt) => {
    concat!("void main() ", stringify!($v)).to_string()
  };
}
#[macro_export]
macro_rules! shader_template {
  ($( $k:ident : $v:tt $(,)?)*) => {{
    shader_attr! {
      struct NilBufferTemplate{}
    }
    struct Template{
      version: i32,
      precision_float: &'static str,
      vs_attr: String, // -> vs_in_code
      fs_attr: (&'static str, &'static str), // -> vs_out_code, fs_in_code
      out_attr : &'static str, // -> fs_out_code
      attrs: String, // -> ub_code[]
      vs_code: String,
      fs_code: String,
    }
    let mut template = Template{
      version: 300,
      precision_float: "highp",
      vs_attr: String::from(""),
      fs_attr: ("", ""),
      out_attr : "",
      attrs: String::from(""),
      vs_code: String::from(""),
      fs_code: String::from(""),
    };
    $(
      template.$k = shader_template_element!($k: $v);
    )*
    let common = format!(
      "#version {} es\nprecision {} float;\n",
      template.version, template.precision_float
    );
    let mut result = ShaderTemplate::new(
      format!("{}{}{}{}",
        common, template.attrs, template.vs_attr, template.fs_attr.0),
      format!("{}{}{}{}",
        common, template.attrs, template.fs_attr.1, template.out_attr),
    );
    result.vs_code_impl = template.vs_code;
    result.fs_code_impl = template.fs_code;
    result
  }};
}

/*
// usage

mod shader;
use shader::*;

shader_attr! {
  struct View {
    view_matrix: mat4,
    projection_matirx: mat4,
    model_matrix: mat4
  }
  struct Material {
    albedo_color: vec3,
    metallic: float,
    emission_color: vec3,
    roughness: float,
    ids: uvec4,
  }
  struct SampleVertex {
    position: vec3,
    normal: vec3,
    uv: vec2,
  }
  struct SampleFragment {
    position: vec3,
    normal: vec3,
    uv: vec2,
  }
}
fn main() {
  let code = shader_template! {
    // version: 300,
    // precision_float: highp,
    // tx_attr:
    attrs: [View, Material],
    vs_attr: SampleVertex,
    fs_attr: {
      in_position: vec3,
      in_color: vec4,
      in_uv: vec2
    },
    out_attr: {
      out_color0: vec4,
      out_color1: vec4,
    },
    vs_code: {
      in_position = position;
      in_color = normal;
      in_uv = uv;
      if (true) {
        in_uv = uv;
      }
      gl_Position = vec4();
    },
    fs_code: {
      out_color0 = vec4();
    }
  };
  println!("{}", code);
  // sample();
}
fn sample() {
  let mut view = View::new();
  view.projection_matirx = mat4::IDENTITY;
  println!("{}", view);
  println!("{}", View::ub_code());
  println!("{}", View::vs_in_code());
  println!("{:?}", View::new().ub_data());
  let mut material = Material::new();
  material.albedo_color = vec3::ZERO;
  println!("{}", material);
  println!("{}", Material::ub_code());
  println!("{}", Material::vs_in_code());
  println!("{:?}", Material::new().ub_data());
  println!("{:?}", Material::keys());
  println!("{:?}", Material::new().values());
  let hashmap = Material::new().to_hashmap();
  println!("{:?}", hashmap);
  println!("{:?}", Material::from_hashmap(&hashmap));
  println!("{:?}", Material::new().find("roughness"));
  println!("{}", SampleVertex::vs_in_code());
  println!("size: {}", SampleVertex::struct_size());
  println!("offsets: {:?}", SampleVertex::offsets());
  println!("{}", SampleFragment::vs_out_code());
  println!("{}", SampleFragment::fs_in_code());
  println!("{}", SampleFragment::fs_out_code());
}
*/
