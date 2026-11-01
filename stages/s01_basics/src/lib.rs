//! # Stage 01: 基礎
//!
//! 変数・型・制御フロー・関数・文字列の基本を学びます。
//!
//! 進め方:
//! 1. `ex01_variables.rs` から順に開き、`todo!()` を自分の実装に置き換える
//! 2. `cargo test -p s01_basics ex01` で1課題ずつテストする
//! 3. 全部通ったら `cargo test -p s01_basics`
//!
//! 参考: The Rust Programming Language 第3章
//! <https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html>

// 未実装の状態で出る警告を抑えています。ステージを解き終えたらこの行を消して、警告が出ないか確認しましょう。
#![allow(unused_variables, unused_imports, dead_code, clippy::ptr_arg)]

pub mod ex01_variables;
pub mod ex02_control_flow;
pub mod ex03_strings;
