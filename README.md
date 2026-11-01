# 🦀 rust-study

ゼロから「立派な Rust エンジニア」になるための学習リポジトリ。
Claude Code をメンターにして、課題 → テスト → レビューのサイクルで鍛えます。

## 進め方

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
3. ブランチを切って PR を作成 → CI が通るか確認
4. Claude Code に「PR をレビューして」と頼む

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
| s05_collections | Vec / HashMap / イテレータ・クロージャ | 🔒 |
| s06_traits | トレイト・ジェネリクス・トレイトオブジェクト | 🔒 |
| s07_lifetimes | ライフタイム | 🔒 |
| s08_smart_pointers | Box / Rc / RefCell / Drop | 🔒 |

### Part 2: 実践

| Stage | テーマ | 状態 |
|---|---|---|
| s09_modules_testing | モジュール設計・統合テスト・doc テスト | 🔒 |
| s10_concurrency | スレッド・チャネル・Arc / Mutex | 🔒 |
| s11_async | async/await と tokio | 🔒 |

### Part 3: プロジェクト（自分で設計して作る）

| Project | 内容 | 学べること |
|---|---|---|
| p01_logscope | ログファイル解析 CLI | clap, ファイル I/O, エラー設計 |
| p02_kvstore | 永続化付きキーバリューストア | serde, ファイルフォーマット設計, ベンチマーク |
| p03_http_server | マルチスレッド HTTP サーバ | TCP, スレッドプール, プロトコル実装 |
| p04_web_api | axum による REST API + DB | async, sqlx, 実務的な構成 |

🔒 のステージは、前のステージをクリアした時点で Claude が作成します。

## 参考資料

- [The Rust Programming Language（日本語版）](https://doc.rust-jp.rs/book-ja/)
- [Rust by Example（日本語版）](https://doc.rust-jp.rs/rust-by-example-ja/)
- [標準ライブラリのドキュメント](https://doc.rust-lang.org/std/) — `rustup doc --std` でオフラインでも読めます
