//! # Stage 09: モジュール設計とテスト
//!
//! 「動くコード」から「チームで育てられるコード」へ進む Stage です。
//!
//! - ex01: モジュールと可視性（`mod` / `pub` / `pub(crate)` / `use` / 再エクスポート）
//! - ex02: テストしやすい設計（依存をトレイトで注入する）
//! - ex03: テストを**書く**（実装は用意済み。ただしバグが潜んでいる）
//!
//! 統合テストは `tests/public_api.rs` にあります。
//!
//! ```text
//! cargo test -p s09_modules_testing              # ユニット + 統合 + doc テスト
//! cargo test -p s09_modules_testing --lib        # ユニットテストだけ
//! cargo test -p s09_modules_testing --test public_api   # 統合テストだけ
//! cargo test -p s09_modules_testing --doc        # doc テストだけ
//! ```
//!
//! 解説は `README.md` を読んでください。

#![allow(unused_variables, unused_imports, dead_code, clippy::ptr_arg)]

pub mod ex01_visibility;
pub mod ex02_dependency_injection;
pub mod ex03_write_tests;

// 再エクスポート: 利用者は `s09_modules_testing::Celsius` と短く書ける。
// 内部のモジュール構成を変えても、利用者のコードは壊れない。
pub use ex01_visibility::temperature::Celsius;
