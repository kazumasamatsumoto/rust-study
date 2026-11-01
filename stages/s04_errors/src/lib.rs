//! # Stage 04: エラー処理
//!
//! Rust に例外はありません。失敗しうる処理は `Option` / `Result` を返し、
//! 呼び出し側に処理を強制します。
//!
//! 参考: The Rust Programming Language 第9章

#![allow(unused_variables, unused_imports, dead_code, clippy::ptr_arg)]

pub mod ex01_option;
pub mod ex02_result;
pub mod ex03_custom_error;
