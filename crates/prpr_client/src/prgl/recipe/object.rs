use super::*;
#[derive(Default)]
pub struct Object {
  // Transformは親子関係をつけたい
  pub transform: TransformWhy,
  pub pipeline: ArcOwner<Pipeline>,
  // gl_Position は書くので、それをTransformFeedbackする
  // - Selection ができる
  // - オフスクリーンやUI上に書くときは？
  // リアルな描画を目指しているわけではないので、影はいらない！（まるぽちでいい）
  // pub bounding_sphere: Option<Sphere>,  // 設定できると効率アップ
}
