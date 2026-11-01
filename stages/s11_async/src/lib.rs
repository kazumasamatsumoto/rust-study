//! # Stage 11: 非同期処理 (async/await と tokio)
//!
//! 「待ち時間が長い処理（I/O）を、たくさん同時にさばく」ための仕組みを学びます。
//!
//! 進め方:
//! 1. `README.md` と `docs/fundamentals/09_io_async.md` を読む
//! 2. `cargo run -p s11_async --example blocking_demo` で、async の中で
//!    ブロッキング処理をすると何が起きるかを見る
//! 3. `ex01_basics.rs` から順に `todo!()` を実装する
//!
//! テストは `#[tokio::test(start_paused = true)]` で「時間を止めた」状態で動きます。
//! `sleep` は実際には待たずに一瞬で終わり、経過時間だけが正確に記録されるので、
//! 「並行に動いたか」をテストで確かめられます。

#![allow(
    unused_variables,
    unused_imports,
    unused_mut,
    dead_code,
    clippy::ptr_arg
)]

pub mod ex01_basics;
pub mod ex02_coordination;
pub mod ex03_under_the_hood;
