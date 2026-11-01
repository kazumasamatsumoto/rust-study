//! # Stage 07: ライフタイム
//!
//! 「この参照は、いつまで有効なのか」をコンパイラと共有するための仕組みです。
//! 解説は `stages/s07_lifetimes/README.md` を先に読んでください。
//!
//! 参考: The Rust Programming Language 第10章3節
//! <https://doc.rust-jp.rs/book-ja/ch10-03-lifetime-syntax.html>

#![allow(unused_variables, unused_imports, dead_code, clippy::ptr_arg)]

pub mod ex01_annotations;
pub mod ex02_struct_refs;
pub mod ex03_borrowed_vs_owned;
