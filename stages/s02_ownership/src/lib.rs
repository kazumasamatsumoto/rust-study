//! # Stage 02: 所有権と借用
//!
//! Rust で最も重要で、最もつまずきやすい部分です。焦らずいきましょう。
//!
//! 3つのルール:
//! 1. 値にはそれぞれ「所有者」となる変数が1つある
//! 2. 所有者は同時に1つだけ
//! 3. 所有者がスコープを抜けると値は破棄される
//!
//! 借用のルール:
//! - `&T`（不変参照）はいくつでも作れる
//! - `&mut T`（可変参照）は同時に1つだけ、しかも `&T` と共存できない
//!
//! 参考: The Rust Programming Language 第4章
//! <https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html>

#![allow(unused_variables, unused_imports, dead_code, clippy::ptr_arg)]

pub mod ex01_move;
pub mod ex02_borrowing;
pub mod ex03_slices;
