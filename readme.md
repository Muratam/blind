# Rust Sandbox Server

Rust の入門兼色々やるためのSandboxなサーバー

# データの種類

## Logic Data
```
DB(like Redis)
↕ : x
Server(rs)
↓ : ↑
[]Client(wasm, js, glsl, html)
x : ↕
DB(like LocalStorage)
```
- その性質上、60FPSでの同期は不可能. 厳格なロジック用のデータ。
  - このデータがあればそこから現状を再構築することができる
- S か C どちらかのみがデータを所有する
  - 値がない場合は自身のDBから読む. そこにも無ければ初期値.
  - 値に更新がある場合は自身のDBに保存する
  - S-C片方のみでしか更新ができない。更新したらそれをもう片方に伝播する
  - S > C と S < C を繋ぐのはプログラム部分(wasm(js), rs)の役割
- 初期化ですべて同期される
  - データを更新した時は反映されるのは**次のフレームから**
  - サーバー/クライアントの両ロジックで齟齬がなくなる
  - クライアントから異常なデータが来た場合 -> リロードさせる
    - データは全て再構築可能なため(正常なデータは1F前に残っている)
- 以下の順序で更新する
  1. S > C / C > S の更新(Frame: N)
  2. S > C / C > S の更新を DB / 別S-C に更新要求
  3. 2. の更新が全て正常に終わればFrame+1。異常があれば更新せずリロード
- undo / redo は Frame を戻す/進める操作。永続木。
  - 操作をコピーすれば操作再現(メイキング)やリプレイができる？

## Volatile Client Data
```
Server(rs)
↑
[]Client(wasm, js, glsl, html)
↓
DB(like localStorege)
```
- 同期されると嬉しいけど別に同期されてなくてもいい、揮発性の高いデータ
  - 60FPSに近いFPSで同期される
- undo / redo はない。最新の状態のみが重要
  - 更新を積んでいき、自Clientでは常に最新(前Frame値)が取れて、他はなるべく最新に近いものがとれる
  - リロードするとDBから読み込める
  - 同一フレーム内でのアトミック性を保つ(一部データのみ更新という不整合は発生しない)
- 以下の順序で更新する
  1. 全て更新(Frame: N)
  2. 更新を DB / Server にコミット。コミットしつつ Frame+1
  3. 次のフレームになっても 2. がまだ終わって無ければ中断して最新のフレームを通達

## Resource Data
- 大規模なデータで、非同期に読み込む必要があるもの
- データがサーバーにある場合
  - クライアントキャッシュ: md5sum を保持しておき、同一ならそれを使用
  - クライアントからサーバーに送信することもあるが基本的にはサーバーにデータがある
- 普通に Async / Await して読み込み完了まで待つかどうか制御する

## Background Thread Data
- 同期の必要はないが、メインの同期・読込ロジックとは別に常に60FPSで動く必要があるもの
  - 以下の階層構造で描画
    1. Main3D + MainEffect(WebGL Canvas)
    2. MainUI2D + Projected3D + ProjectedEffect (2D Canvas)
    3. Other HTML(グラフ描画ライブラリや文字入力テキストボックス)
  - 統一的に扱って裏で動かしたい
    - パーティクルエフェクト
      - Instancing + TF でなんとか
    - UI の Transition/Animation
    - 3Dモデルのアニメーション
    - ユーザー入力

## other
- 他のデータは重大なのなさそう

# 言語

サーバー・クライアント両者のコードをまとめて管理するのが必須。
Rustで書いて、 クライアントは wasm(+ html + glsl + js(for lib))になる


# 世界観
- UI/パーティクルは, 世界観を統一したい
  - 楽園追放で主人公がだしてたような3DのUIにしたい！！！
  - せっかくHTMLの呪縛から逃れられるしな
  - 必要な分だけ(ビルボードな)UIを起動できて結構画期的だと感じる
  - RPGツクールと同じで、優秀なUI優秀なテンプレートを作れば、広くそのまま使える
    - bootstrap とか長らくそのままよく使用されていたし
- ボドゲサーバー：少しだけperspectiveな3D空間のボドゲテーブルを意識したい
  - tooltip はすごい便利なので楽園追放UIをインタラクティブにクリックしたい
    - 表示するか非表示にするかはユーザーが選ぶ。再度クリックしたら消える
    - 何でも選べるべき。何でもマウスホバー中のものを輪郭表示すべき
  - ドッキングは便利
  - 左上のメニューは便利
  - 3D空間にあるものは、全てクリックすると楽園追放UIがでるべき
- モデル漫画エディタ
  - とりあえず原神と同じクオリティのモデル・アニメーションを作りたい
  - 服とか小物とかすごい複雑だけど簡単に作れるんやろうか...
  - そもそも既存のモデリング描画の枠組みでいいのだろうか...
    - 曲線を直線でやるのやめたい
    - -> 三角錐で表面を裏カリングで表現する
      - 底面はそのまま
      - 蓋となる三角錐を追加する。ベジェ曲線なので輪郭も得意。
        - 位置なども計算できるのでPBRも大丈夫


# 最終的にやりたいこと
- ボードゲームサーバー
  - コンテンツ
    - 超人ロックのサーバー
    - 東方超人録のサーバー
- モデル・漫画のエディタ
  - コンテンツ制御
    - 自動セリフ(要学習)
    - 漫画やイラストから学習してモデリング

# 必要なものメモ
- クライアント・サーバーどこに実態があるかを意識したくない
  - WebSocket (多くのデータがサーバー側に)
  - WebAssemblyを吐きたい
  - WebRTCでP2Pへの発展も
