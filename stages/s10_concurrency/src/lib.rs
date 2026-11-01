//! # Stage 10: 並行処理
//!
//! Rust の「恐れのない並行性」(fearless concurrency) を体験する Stage です。
//!
//! - ex01: スレッドの生成と join、`move` クロージャ、scoped threads
//! - ex02: チャネルによるメッセージパッシング
//! - ex03: `Arc<Mutex<T>>` とアトミックによる共有状態、デッドロックの回避
//!
//! 先に `docs/fundamentals/08_concurrency.md` を読んでください。
//! テストがいつまでも終わらない場合は、デッドロックか、join し忘れたスレッドを疑いましょう。

#![allow(unused_variables, unused_imports, dead_code, clippy::ptr_arg)]

pub mod ex01_threads;
pub mod ex02_channels;
pub mod ex03_shared_state;
