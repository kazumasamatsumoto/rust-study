//! # Stage 06: トレイトとジェネリクス
//!
//! トレイトは「型が持つべき振る舞い」の約束です。
//! ジェネリクスと組み合わせると、型ごとにコードを書き分けずに済みます。
//!
//! 解説は `README.md` にあります。先に読んでから課題に進んでください。
//!
//! 参考: The Rust Programming Language 第10章・第17章・第19章

#![allow(
    unused_variables,
    unused_imports,
    unused_mut,
    dead_code,
    clippy::ptr_arg
)]

pub mod ex01_generics;
pub mod ex02_std_traits;
pub mod ex03_trait_objects;
