#[allow(unused_imports)]
use super::*;

#[macro_export]
macro_rules! shader_attr_by_type {
  (struct $s:ident { $( $k:ident : $v:ident)* }) => {
    #[derive(Default, Debug)]
    #[repr(C)]
    pub struct $s {
      $(pub $k : $v,)*
    }

    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl $s {
      #[allow(dead_code)]
      pub fn new() -> Self { Default::default() }
      #[allow(dead_code)]
      pub fn ub_code() -> &'static str {
        concat!(
          "layout (std140) uniform ", stringify!($s), " {\n",
            $("  ", stringify!($v) ," ", stringify!($k), ";\n",)*
          "};"
        )
      }
      #[allow(dead_code)]
      pub fn uniform_block_name() -> Option<&'static str> {
        Some(Self::name_static())
      }
      #[allow(dead_code)]
      pub fn uniform_textures() -> Vec<&'static str> {
        Vec::new()
      }
      #[allow(dead_code)]
      pub fn vs_in_code() -> &'static str {
        concat!(
          $("in ", stringify!($v)," ",stringify!($k),";\n", )*
        )
      }
      #[allow(dead_code)]
      pub fn vs_out_code() -> &'static str {
        concat!(
          $("out ", stringify!($v) ," ", stringify!($k), ";\n",)*
        )
      }
      #[allow(dead_code)]
      pub fn fs_in_code() -> &'static str {
        concat!($("in ", stringify!($v) ," ", stringify!($k), ";\n",)*)
      }
      #[allow(dead_code)]
      pub fn fs_out_code() -> &'static str {
        concat!($("out ", stringify!($v) ," ", stringify!($k), ";\n",)*)
      }
      #[allow(dead_code)]
      pub fn struct_size() -> usize {
        ::std::mem::size_of::<$s>()
      }
      #[allow(dead_code)]
      pub fn offsets() -> Vec<usize> {
        let mut result = Vec::new();
        let dummy = ::core::mem::MaybeUninit::<Self>::uninit();
        let dummy_ptr = dummy.as_ptr();
        $(
          let member_ptr = unsafe{ ::core::ptr::addr_of!((*dummy_ptr).$k) };
          result.push(member_ptr as usize - dummy_ptr as usize);
        )*
        result
      }
      #[allow(dead_code)]
      pub fn name_static() -> &'static str { stringify!($s) }
      #[allow(dead_code)]
      #[allow(unused_variables)]
      pub fn keys_static() -> Vec<&'static str> {
        let mut result = Vec::new();
        $(result.push(stringify!($k));)*
        result
      }
    }
    impl $crate::prgl::BufferAttribute for $s {
      fn ub_data(&self) -> &[u8] {
        let u8_size = Self::struct_size();
        let ptr = self as *const $s as *const u8;
        unsafe { ::core::slice::from_raw_parts(ptr, u8_size) }
      }
      fn vs_in_template(&self) -> $crate::prgl::VsInTemplate {
        $crate::prgl::VsInTemplate{
          keys: Self::keys_static(),
          values: Self::new().values(),
          offsets: Self::offsets(),
          size: Self::struct_size(),
        }
      }
      fn keys(&self) -> Vec<&'static str> { Self::keys_static() }
      #[allow(unused_variables)]
      #[allow(unused_mut)]
      fn values(&self) -> Vec<$crate::prgl::ShaderPrimitiveType> {
        let mut result = Vec::new();
        $(result.push($crate::prgl::ShaderPrimitiveType::$v(self.$k));)*
        result
      }
      fn name(&self) -> &'static str { Self::name_static() }
      fn find(&self, key: &str) -> Option<$crate::prgl::ShaderPrimitiveType> {
        match key {
          $(stringify!($k) => Some($crate::prgl::ShaderPrimitiveType::$v(self.$k)),)*
          _ => None,
        }
      }
      #[allow(unused_variables)]
      #[allow(unused_mut)]
      fn from_hashmap(&mut self, map: &::std::collections::HashMap<String, $crate::prgl::ShaderPrimitiveType>) -> Vec<&'static str> {
        let mut ignored = Vec::new();
        $(
          if let Some($crate::prgl::ShaderPrimitiveType::$v(v)) = map.get(stringify!($k)) {
            self.$k = *v;
          } else {
            ignored.push(stringify!($k));
          }
        )*
        ignored
      }
      #[allow(unused_variables)]
      #[allow(unused_mut)]
      fn to_hashmap(&self) -> ::std::collections::HashMap<String, $crate::prgl::ShaderPrimitiveType> {
        let mut result = ::std::collections::HashMap::new();
        $(result.insert(String::from(stringify!($k)), $crate::prgl::ShaderPrimitiveType::$v(self.$k));)*
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
  };
  (mapping $s:ident { $( $k:ident : $v:ident)* }) => {
    // new は定義せず、全て揃ってから代入？
    pub struct $s {
      $(pub $k : $v,)*
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl $s {
      #[allow(dead_code)]
      pub fn ub_code() -> &'static str {
        concat!(
          $("uniform ", stringify!($v), " ", stringify!($k), ";\n",)*
        )
      }
      #[allow(dead_code)]
      pub fn uniform_block_name() -> Option<&'static str> {
        None
      }
      #[allow(dead_code)]
      pub fn uniform_textures() -> Vec<&'static str> {
        Self::keys_static()
      }
      #[allow(dead_code)]
      pub fn name_static() -> &'static str { stringify!($s) }
      #[allow(dead_code)]
      #[allow(unused_variables)]
      pub fn keys_static() -> Vec<&'static str> {
        let mut result = Vec::new();
        $(result.push(stringify!($k));)*
        result
      }
    }
    impl $crate::prgl::TextureMappingAttribute for $s {
      fn keys(&self) -> Vec<&'static str>{
        Self::keys_static()
      }
      #[allow(unused_variables)]
      #[allow(unused_mut)]
      fn values(&self) -> Vec<$crate::prgl::ShaderSamplerType>{
        let mut result = Vec::new();
        $(result.push($crate::prgl::ShaderSamplerType::$v(self.$k.clone_reader()));)*
        result
      }
      fn name(&self) -> &'static str {
        Self::name_static()
      }
      fn find(&self, key: &str) -> Option<$crate::prgl::ShaderSamplerType>{
        match key {
          $(stringify!($k) => Some($crate::prgl::ShaderSamplerType::$v(self.$k.clone_reader())),)*
          _ => None,
        }
      }
      #[allow(unused_variables)]
      #[allow(unused_mut)]
      fn from_hashmap(&mut self, map: &::std::collections::HashMap<String, $crate::prgl::ShaderSamplerType>) -> Vec<&'static str>{
        let mut ignored = Vec::new();
        $(
          if let Some($crate::prgl::ShaderSamplerType::$v(v)) = map.get(stringify!($k)) {
            self.$k = v.clone_reader();
          } else {
            ignored.push(stringify!($k));
          }
        )*
        ignored
      }
      #[allow(unused_variables)]
      #[allow(unused_mut)]
      fn to_hashmap(&self) -> ::std::collections::HashMap<String, $crate::prgl::ShaderSamplerType>{
        let mut result = ::std::collections::HashMap::new();
        $(result.insert(String::from(stringify!($k)), $crate::prgl::ShaderSamplerType::$v(self.$k.clone_reader()));)*
        result
      }
    }
  };
}

#[macro_export]
macro_rules! shader_attr {
  ($( $type:ident $s:ident { $( $k:ident : $v:ident $(,)?)* } $(;)?)*) => (
    $(shader_attr_by_type!{ $type $s { $( $k : $v )* } })*
  );
}
#[macro_export]
macro_rules! shader_template_element_parse_code {
  ( { $( $v: tt )* } ) => {
    concat!($(stringify!($v)," ",)*).to_string()
  };
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
    let mut definitions = String::new();
    $(
      definitions += $v::ub_code();
      definitions += "\n";
    )*
    let mut u_blocks = Vec::new();
    $(
      if let Some(block) = $v::uniform_block_name() {
        u_blocks.push(block);
      }
    )*
    let mut u_textures = Vec::new();
    $(
      for name in $v::uniform_textures() {
        u_textures.push(name);
      }
    )*
    (definitions, u_blocks, u_textures)
  }};
  (vs_code: $v:tt ) => {
    shader_template_element_parse_code!($v)
  };
  (fs_code: $v:tt ) => {
    shader_template_element_parse_code!($v)
  };
}

#[macro_export]
macro_rules! shader_template {
  ($( $k:ident : $v:tt $(,)?)*) => {{
    $crate::shader_attr! {
      struct NilBufferTemplate{}
    }
    #[derive(Default)]
    struct Template{
      version: i32,
      precision_float: &'static str,
      vs_attr: &'static str,
      fs_attr: (&'static str, &'static str), // -> vs_out_code, fs_in_code
      out_attr : &'static str, // -> fs_out_code
      attrs: (String, Vec<&'static str>, Vec<&'static str>), // -> concat!(ub_code*), uniforms, textures)
      vs_code: String,
      fs_code: String,
    }
    let mut template : Template = Default::default();
    template.version = 300;
    template.precision_float = "highp";
    $(
      template.$k = $crate::shader_template_element!($k: $v);
    )*
    let common = format!(
      "#version {} es\nprecision {} float;\n",
      template.version, template.precision_float
    );
    let mut result = $crate::prgl::ShaderTemplate::new(
      template.attrs.1,
      template.attrs.2,
      format!("{}\n{}{}\n{}",
        common, template.attrs.0, template.vs_attr, template.fs_attr.0),
      format!("{}\n{}{}\n{}",
        common, template.attrs.0, template.fs_attr.1, template.out_attr),
    );
    result.vs_code_body = template.vs_code;
    result.fs_code_body = template.fs_code;
    result
  }};
}
