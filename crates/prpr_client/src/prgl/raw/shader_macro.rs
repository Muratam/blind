#[allow(unused_imports)]
use super::*;

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
      impl $s {
        fn new() -> Self { Default::default() }
        fn ub_code() -> &'static str {
          concat!(
            "layout (std140) uniform ", stringify!($s), " {\n",
              $("  ", $crate::shader_type_str!($v) ," ", stringify!($k), ";\n",)*
            "};"
          )
        }
        fn vs_in_code() -> &'static str {
          concat!(
            $("in ", $crate::shader_type_str!($v)," ",stringify!($k),";\n", )*
          )
        }
        fn vs_out_code() -> &'static str {
          concat!(
            $("out ", $crate::shader_type_str!($v) ," ", stringify!($k), ";\n",)*
          )
        }
        fn fs_in_code() -> &'static str {
          concat!($("in ", $crate::shader_type_str!($v) ," ", stringify!($k), ";\n",)*)
        }
        fn fs_out_code() -> &'static str {
          concat!($("out ", $crate::shader_type_str!($v) ," ", stringify!($k), ";\n",)*)
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
        fn name() -> &'static str { stringify!($s) }
        fn keys() -> Vec<&'static str> {
          let mut result = Vec::new();
          $(result.push(stringify!($k));)*
          result
        }
        fn values(&self) -> Vec<ShaderPrimitiveType> {
          let mut result = Vec::new();
          $(result.push(ShaderPrimitiveType::$v(self.$k));)*
          result
        }
        fn find(&self, key: &str) -> Option<ShaderPrimitiveType> {
          match key {
            $(stringify!($k) => Some(ShaderPrimitiveType::$v(self.$k)),)*
            _ => None,
          }
        }
        // for dynamic loading
        fn from_hashmap(map: &::std::collections::HashMap<String, ShaderPrimitiveType>) -> (Self, Vec<&'static str>) {
          let mut result = Self::new();
          let mut ignored = Vec::new();
          $(
            if let Some(ShaderPrimitiveType::$v(v)) = map.get(stringify!($k)) {
              result.$k = *v;
            } else {
              ignored.push(stringify!($k));
            }
          )*
          (result, ignored)
        }
        fn to_hashmap(&self) -> ::std::collections::HashMap<String, ShaderPrimitiveType> {
          let mut result = ::std::collections::HashMap::new();
          $(result.insert(String::from(stringify!($k)), ShaderPrimitiveType::$v(self.$k));)*
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
    (
      VsInTemplate {
        keys: $v::keys(),
        values: $v::new().values(),
        offsets: $v::offsets(),
        size: $v::struct_size(),
      },
      $v::vs_in_code()
    )
  };
  (fs_attr: $v:ident) => {{
    ($v::vs_out_code(), $v::fs_in_code())
  }};
  (fs_attr: $v:tt) => {{
    $crate::shader_attr!{struct FsAttr $v}
    (FsAttr::vs_out_code(), FsAttr::fs_in_code())
  }};
  (out_attr: $v:ident) => {
    $v::fs_out_code()
  };
  (out_attr: $v:tt) => {{
    $crate::shader_attr!{struct OutAttr $v}
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
    $crate::shader_attr! {
      struct NilBufferTemplate{}
    }
    struct Template{
      version: i32,
      precision_float: &'static str,
      vs_attr: (VsInTemplate, &'static str),
      fs_attr: (&'static str, &'static str), // -> vs_out_code, fs_in_code
      out_attr : &'static str, // -> fs_out_code
      attrs: String, // -> ub_code[]
      vs_code: String,
      fs_code: String,
    }
    let mut template = Template{
      version: 300,
      precision_float: "highp",
      vs_attr: (
        VsInTemplate {
          keys: Vec::new(),
          values: Vec::new(),
          offsets: Vec::new(),
          size: 0,
        },
        ""
      ),
      fs_attr: ("", ""),
      out_attr : "",
      attrs: String::from(""),
      vs_code: String::from(""),
      fs_code: String::from(""),
    };
    $(
      template.$k = $crate::shader_template_element!($k: $v);
    )*
    let common = format!(
      "#version {} es\nprecision {} float;\n",
      template.version, template.precision_float
    );
    let mut result = ShaderTemplate::new(
      template.vs_attr.0,
      format!("{}{}{}{}",
        common, template.attrs, template.vs_attr.1, template.fs_attr.0),
      format!("{}{}{}{}",
        common, template.attrs, template.fs_attr.1, template.out_attr),
    );
    result.vs_code_impl = template.vs_code;
    result.fs_code_impl = template.fs_code;
    result
  }};
}
