use strattera_server::*;
// use std::future::Future;
// use std::option::Option;

pub struct Store<T> {
  latest: T,
  staging: T
}
// blind

impl <T: Default> Store<T> {
  pub fn load_or_default() -> Store<T> {
    Store{
      latest : Default::default(),
      staging : Default::default(),
    }
  }
}
impl <T> Store<T> {
  // 様々な理由で読み込めないことがありうるので、ハンドリングしたいはず
  // pub fn load() -> dyn Option<Store<T>> {
  //   unimplemented!();
  // }
  // pub async fn load_async() -> dyn Future<Option<Store<T>>> {
  //   unimplemented!();
  // }
  // pub fn load_on_server(client_id: i64) -> Store<T> { load() }
  pub fn get(&self) -> &T { &self.latest }
  pub fn staging(&mut self) -> &mut T { &mut self.staging }
}

// pub struct HistoryStore<T> {}
// get, frame(), undo, redo, ...

// 最新のみ必要
#[derive(Debug, Default)]
struct PlayerInfo {
  pub name : String
}

// 履歴全部残す系
#[derive(Debug, Default)]
struct Chat {
  pub message: String
}

fn example() {
  // // データロード(保持内容は全て読み込む)
  // // データ同期は明示的に呼ばない. 内部で自動的に行われる
  // // クライアント側(クライアントIdは一意なので不要)
  let mut player = Store::<PlayerInfo>::load_or_default();
  println!("{:?}", player.get());
  {
    let mut staging = player.staging();
    staging.name = "iikanji".to_string();
    println!("{:?}", staging);
  }
}

fn main() {
  example();
  let config = ServerConfig{..Default::default()};
  serve(&config);
}
