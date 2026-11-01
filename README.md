# 🦀 rust-study

ゼロから「立派な Rust エンジニア」になるための学習リポジトリ。
Claude Code をメンターにして、課題 → テスト → レビューのサイクルで鍛えます。

鍛えるのは3つの力です。

- **書く力**: `todo!()` を埋めて、テストを通す
- **理解する力（CS基礎）**: ビット、メモリ、ポインタ、文字コードなど、コンピュータの中で何が起きているのかを知る
- **選ぶ力（選球眼）**: なぜこの技術・設計なのか、他の案と比べてどうなのかを言葉にする

## 進め方

各ステージは **解説を読む → コードを書く → 問いに答える** の3ステップで進めます。

1. `stages/sXX/README.md` の解説を読む（冒頭の「前提知識」に挙げた CS基礎のドキュメントも読む）
2. 課題の `todo!()` を実装する
3. README の最後にある「設計の問い」に、`answers/sXX.md` で答える

```bash
# 1. ステージの課題ファイルを開いて todo!() を実装する
#    例: stages/s01_basics/src/ex01_variables.rs

# 2. 課題ごとにテスト
cargo test -p s01_basics ex01

# 3. ステージ全体をテスト
cargo test -p s01_basics

# 4. 品質チェック（実務でも毎回やるやつ）
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
```

ステージをクリアしたら:

1. `lib.rs` の `#![allow(...)]` 行を消して、警告が出ないことを確認
2. `progress.txt` にステージ名（例: `s01_basics`）を追記
3. `answers/sXX.md` に設計の問いの回答を書く
4. ブランチを切って PR を作成 → CI が通るか確認
5. Claude Code に「PR をレビューして」と頼む（コードと回答の両方をレビューします）

> **ルール**: 詰まったら答えではなく **ヒント** を聞くこと。
> 「ヒント1をください」→ それでも無理なら「ヒント2」…と段階的に。

## ロードマップ

### Part 1: 言語の基礎

| Stage | テーマ | 状態 |
|---|---|---|
| [s01_basics](stages/s01_basics) | 変数・型・制御フロー・文字列 | ⬜ |
| [s02_ownership](stages/s02_ownership) | 所有権・借用・スライス | ⬜ |
| [s03_structs_enums](stages/s03_structs_enums) | 構造体・列挙型・パターンマッチ | ⬜ |
| [s04_errors](stages/s04_errors) | Option / Result / 独自エラー型 | ⬜ |
| [s05_collections](stages/s05_collections) | Vec / HashMap / イテレータ・クロージャ | ⬜ |
| [s06_traits](stages/s06_traits) | トレイト・ジェネリクス・トレイトオブジェクト | ⬜ |
| [s07_lifetimes](stages/s07_lifetimes) | ライフタイム | ⬜ |
| [s08_smart_pointers](stages/s08_smart_pointers) | Box / Rc / RefCell / Drop | ⬜ |

### Part 2: 実践

| Stage | テーマ | 状態 |
|---|---|---|
| [s09_modules_testing](stages/s09_modules_testing) | モジュール設計・統合テスト・doc テスト | ⬜ |
| [s10_concurrency](stages/s10_concurrency) | スレッド・チャネル・Arc / Mutex | ⬜ |
| [s11_async](stages/s11_async) | async/await と tokio | ⬜ |

### CS基礎トラック（並行して進める）

Rust の設計の背景にある理論を、動かせる実験と一緒に学びます。→ [docs/fundamentals/](docs/fundamentals/)

```bash
cargo run -p fundamentals --example integers   # 整数とオーバーフロー
cargo run -p fundamentals --example memory_layout  # スタックとヒープ、ムーブの正体
```

| # | テーマ | 関係するステージ |
|---|---|---|
| 01 | 整数の表現とオーバーフロー（2進数、2の補数） | s01 |
| 02 | 浮動小数点数（IEEE 754、NaN、なぜ f64 は Ord でないのか） | s01 |
| 03 | メモリ: スタックとヒープ、ムーブの正体 | s02 |
| 04 | ポインタと、メモリ安全性のバグ | s02 |
| 05 | 文字コードと UTF-8 | s01, s02 |
| 06 | 計算量とデータ構造 | s02, s05 |
| 07 | 静的ディスパッチと動的ディスパッチ（単相化、vtable） | s06 |
| 08 | プロセス・スレッド・データ競合 | s10 |
| 09 | I/O モデルと非同期処理（C10K、epoll、Future） | s11 |
| 10 | コンパイルとリンク、最適化 | s09 |

### 選球眼トラック（並行して進める）

| ドキュメント | 内容 |
|---|---|
| [docs/judgment/01_when_to_use_rust.md](docs/judgment/01_when_to_use_rust.md) | Rust を選ぶべき時・選ぶべきでない時 |
| [docs/judgment/02_choosing_crates.md](docs/judgment/02_choosing_crates.md) | クレートの選び方・定番クレートが選ばれる理由 |
| [docs/judgment/03_adr.md](docs/judgment/03_adr.md) | 技術判断を ADR として記録する |

### Part 3: プロジェクト（自分で設計して作る）

Part 3 では、要件だけが渡されます。設計と技術選定は自分で行い、主な判断ごとに ADR を書きます。→ [projects/](projects/)

| Project | 内容 | 学べること |
|---|---|---|
| [p01_logscope](projects/p01_logscope/SPEC.md) | ログファイル解析 CLI | clap, ファイル I/O, エラー設計 |
| [p02_kvstore](projects/p02_kvstore/SPEC.md) | 永続化付きキーバリューストア | serde, ファイルフォーマット設計, ベンチマーク |
| [p03_http_server](projects/p03_http_server/SPEC.md) | マルチスレッド HTTP サーバ | TCP, スレッドプール, プロトコル実装 |
| [p04_web_api](projects/p04_web_api/SPEC.md) | axum による REST API + DB | async, sqlx, 実務的な構成 |


## 参考資料

- [The Rust Programming Language（日本語版）](https://doc.rust-jp.rs/book-ja/)
- [Rust by Example（日本語版）](https://doc.rust-jp.rs/rust-by-example-ja/)
- [標準ライブラリのドキュメント](https://doc.rust-lang.org/std/) — `rustup doc --std` でオフラインでも読めます
