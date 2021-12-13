// use とか mod とかできる？
// シェーダの関数を読み込める？
// ubo用attributeオブジェクト作成
pub type view_attr = shader_attr!{
  view_matrix: Mat4,
  projection_matirx: Mat4
};
// ubo情報オブジェクトを作成できる
let view_ubo = shader_ubo!(view_attr);
pub type obj_attr = shader_attr!{
  model_matrix: Mat4
};
pub type glb_attr = shader_attr!{
  add_color: Vec4,
};
// これは外部ファイルになっていて動的かも？
// -> HashMapでデータの読み込みができればOKそう。
pub type mat_attr = shader_attr!{
  albedo_color: Vec4,
  roughness: f32,
};

// 実体化はできない。
// シェーダにはUBOの型を残さないので間違えてバインドすることは許可？
pub type standard_attrs = shader_attrs![
  glb_attr, obj_attr, view_attr, mat_attr
];

pub type vs_attr = shader_attr!{
  position: Vec3,
  color: Vec4,
  uv: Vec2
};
// これも外部ファイルになっていて動的かも？
// -> HashMapでデータの読み込みができればOKそう。
//   : normalのみ, positionのみ, ..., から取り出す
// シェーダにはVBOの型を残さないので間違えてバインドすることは許可？
let vbo = shader_vbo!(vs_attr);
pub type tx_attr = shader_attr!{
  normal: Sampler2D,
  albedo: Sampler2D
};
// これも外部ファイルになっていて動的かも？
// -> HashMapでデータの読み込みができればOKそう。
// シェーダにはTEXTUREの型を残さないので間違えてバインドすることは許可？
let txo = shader_txo!(tx_attr);
pub type common_code = shader_code!{
  float id(float x) { return x; }
};
pub type vs_code = shader_code!{
  common_code!;
  void main() {
    fs_color = vs_color;
    fs_position = vs_position:
    fs_uv = vs_uv;
    gl_Position = vec4(vs_position, id(1.0));
  }
};
// シェーダ情報オブジェクトを作成
let mut shader = shader! {
  version: 300, // 省略可能
  precision_float: highp, // 省略可能
  attrs: standard_attrs,
  tx_attr: tx_attr,
  vs_attr: vs_attr,
  fs_attr: { // shader_attr! マクロで一応共通化可能
    position: Vec3,
    color: Vec4,
    uv: Vec2
  },
  out_attr: { // shader_attr! マクロで一応共通化可能
    color0: Vec4,
    // color1: Vec4,
  },
  vs_code: vs_code,
  fs_code: { // shader_code! マクロで共通化可能
    common_code!;
    void main() {
      out_color0 = fs_color + add_color + texture2D(tx_normal, fs_uv);
    }
  },
}
