use std::future::Future;
use std::option::Option;

/*
- インタラクティブな通信をしないシーンの読み込みフロー
  1. WebSocket

- 実は全てWebSocketでいい
  - コネクションを適宜貼って切ってする
    常駐型の型が一つもなければ勝手に切る
  - HTTPGet使ってもどうせ整合性チェックする必要があるので
  - すごい重たいリソースを非同期で読みたいこともある？ちゃんと待つ必要がある
  - WebSocketを切るとClient側にServerから通達できなくなるが問題ない
- データは一つの木構造で表現？DAG?再帰あり？
- リソース読み込みの発火がサーバーだけでできるようにすれば？
- サーバー側のロジックに特化したクラス？
  - 入力待ち -> Server実行(適宜出力(シーンなど)) -> 入力待ち -> ...
    - Server側でやることがなくなったら切ればいい(?)
  - 再起動時：データは全部残ってるので復元可能
- N人対戦
  - メインロジックはサーバーが持ってる
    - 人が抜けたら？解体or無限待ち?
  - ただのコルーチン？
- メインロジック + サブロジック
- 10音階


- クライアント側からできること
  - 描画・入力・計算
  - 同じシーンをサーバーとクライアント両方が持っていて、
    呼び出せる権限が異なる
    - 毎フレームのupdateはクライアント側からしか呼べない
    - Server側イベントの発火ができる
      - サーバー側は何でもできるので？
*/
/*
  - 読込開始すると
    - シーンを構成するバイナリのリソースを必要な分要求する
      - ロジックはwsに入っている
      - サーバー側も何が必要か把握しているので、Permissionは不要。
        クライアントは、可能な操作を送るだけ
  - ロック手順
    1. ルート階層にアクセス. 初期Package読み込み
    2. 「そのPackageで必要なリソースは予め全て把握できている」ので、読み込み完了まで待つ
      - 読み込み完了したら描画！
      - 子Packageは読み込めている？木？DAG?再帰？
      - リソースの読み込みはパッケージ単位でOK
    3.
*/

/*
  # データの管理について

  - WebSocket
    - どこかで使用された時点で有効化する。どこでも使用されなくなったら無効化
    - 起動時と終了時だけはBlockingしてよい
    - 何も通信が無いなら無駄なコネクションは切りたい

  Store 型毎に定義するのがベスト
  - ResourceStore
    - サイズの大きい読み込み専用リソース
    - Serverに保存. ClientStorageにキャッシュできる
      - md5sum を問い合わせて、同じであればClientStorageのものを使用.
        サーバー側はリクエストに応じてtimestamp-md5sumペアを揮発保存していく
    - HTTP Get / Fetch のみ (WebSocket は不要)
    - Async / Sync どちらでも OK.
      - Async の場合, 他とリクエストを勝手にまとめて効率化できるかも？
    - データは生でサーバーのSSDに配置される.
      そのディレクトリ名ファイル名を指定して取得(どこかで必ず指定が必要なため)
    - ResourcePermission
      - (リソースの)認証を管理
      - Serverに保存. Server でだけ更新できる.
      - client.allow("/world/") で, そのクライアントに権限を付与できる
      - default_deny("/") で、デフォルトで取得禁止を付与できる
  - LogicStore
    -

  - SyncStore<>
    - commit が同期的に行われる
    - pub latest(), pub staging(), pub commit() -> bool
  - AsyncStore<>
    - pub latest(), pub staging(), async commit()




  ClientStorage <-> ClientRuntime <-> ServerRuntime <-> ServerStorage の Policy 記述
  # 更新可能性
  `Cs<Cr-Sr-Ss` : Cr で Sr を更新できない

  - ex) ロック
    - Client-Unique-Hash 生成: @C ゲスト認証用
    - ゲスト名: @C
    - 部屋: @S
    -

*/

enum VersionControlStatus {
  Latest, //
  Staging,
}

pub struct VersionControl<T> {
  latest: T,
  staging: Option<T>,
  status: VersionControlStatus,
}

// 値のバリデーションが必要な場合に
// クライアント側から不正な値が送られてくることがありえる
pub trait Validation {
  fn validate(&self, pre: &Self) -> bool;
}

impl<T: Default> VersionControl<T> {
  pub fn load_or_default() -> VersionControl<T> {
    VersionControl {
      latest: Default::default(),
      staging: None,
      status: VersionControlStatus::Latest,
    }
  }
}

impl<T> VersionControl<T> {
  // 様々な理由で読み込めないことがありうる。
  // 未初期化とinvalid値を区別する必要あり？
  // サーバー側ロジックでは、cliendIdが必要
  pub fn load() -> Option<VersionControl<T>> {
    unimplemented!();
  }
  pub fn load_async() -> Future<Option<VersionControl<T>>> {
    unimplemented!();
  }

  // 最新の安定データを取得(SReader)
  pub fn latest(&self) -> &T {
    &self.latest
  }

  // 変数全てをOptionalにするReflectionができるならそれでいいかも (変更点のみ通知できてお得)
  //
  pub fn staging(&mut self) -> &mut Option<T> {
    &mut self.staging
  }

  // 更新はClientもServerもできる。明示的にsyncとは記述せず裏で回す？
  // 自分が Slave ->
  // 自分が Master ->
  // - validation 失敗 ->
  // - validation 成功 ->
  // 更新タイミング：毎フレーム(WebSocket)
  // Validation + Store + Commit
  fn sync(&self) {}
}

// pub struct HistoryStore<T> {}
// get, frame(), undo, redo, ...

// 最新のみ必要
#[derive(Debug, Default)]
struct PlayerInfo {
  pub name: String,
}

// 履歴全部残す系
#[derive(Debug, Default)]
struct Chat {
  pub message: String,
}

fn example() {
  // データロード(保持内容は全て読み込む)
  // データ同期は明示的に呼ばない. 内部で自動的に行われる
  // クライアント側(クライアントIdは一意なので不要)
  let mut player = VersionControl::<PlayerInfo>::load_or_default();
  println!("{:?}", player.latest());
  {
    let mut staging = player.staging();
    staging.name = "iikanji".to_string();
    println!("{:?}", staging);
  }
  player.sync();
}
