//! # Stage 14: 型による設計
//!
//! Stage 03 で「不正な状態を表現できなくする」という考え方に触れました。
//! この Stage では、それを本格的な道具（newtype、typestate、PhantomData、
//! sealed trait、`#[non_exhaustive]`）で実践します。
//!
//! 目標は「**間違った使い方をすると、コンパイルが通らない API**」を設計できるようになることです。
//!
//! 解説は `README.md` にあります。先に読んでから課題に進んでください。
//!
//! テストの実行:
//! ```text
//! cargo test -p s14_type_design            # ユニットテスト + 統合テスト + doc テスト
//! cargo test -p s14_type_design --doc      # doc テストだけ
//! ```
//!
//! この Stage には `compile_fail` と書かれた doc テストがあります。
//! これは「このコードは**コンパイルが通らないこと**」を確かめるテストです。
//! 型で禁止したいことが、本当に禁止されているかを確認しています。

#![allow(
    unused_variables,
    unused_imports,
    unused_mut,
    dead_code,
    clippy::ptr_arg
)]

pub mod ex01_newtypes;
pub mod ex02_typestate;
pub mod ex03_money;
