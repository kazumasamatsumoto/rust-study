//! # Stage 13: unsafe と FFI
//!
//! コンパイラが安全性を検証できない操作を、**自分の責任で**行う方法を学びます。
//! そして、それを安全な API の中に閉じ込める方法を学びます。
//!
//! - ex01: 生ポインタ、`unsafe fn`、SAFETY コメント
//! - ex02: `MaybeUninit` を使った固定長スタック（unsafe を安全な API で包む）
//! - ex03: FFI — C の標準ライブラリの関数を呼ぶ、C から Rust の関数を呼んでもらう
//!
//! 先に `README.md` と `docs/fundamentals/03_memory.md`・`04_pointers.md` を読んでください。
//!
//! **この Stage のルール**: `unsafe` ブロックを書くたびに、その直前に
//! `// SAFETY: ...` というコメントで「なぜこれが安全なのか」を書くこと。
//! clippy の `undocumented_unsafe_blocks` という lint で、書き忘れを検出しています。

// 未実装の状態で出る警告を抑えています。ステージを解き終えたらこの行を消して、警告が出ないか確認しましょう。
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]
#![warn(clippy::undocumented_unsafe_blocks)]

pub mod ex01_raw_pointers;
pub mod ex02_fixed_stack;
pub mod ex03_ffi;
