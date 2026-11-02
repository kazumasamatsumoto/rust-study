//! # Stage 15: 性能
//!
//! 「速いコード」を書く前に、「速さを正しく測る」方法を学びます。
//! そのうえで、アロケーションを減らす・計算量を下げる・データの持ち方を変える、
//! という典型的な改善を、テストで正しさを保ったまま行います。
//!
//! 解説は `README.md` にあります。先に読んでから課題に進んでください。
//!
//! この Stage の課題には、遅い実装（`*_slow`）が最初から書かれています。
//! あなたの仕事は、同じ結果を返す速い実装を書くことです。
//!
//! - **正しさ** はテストで確かめます（`cargo test -p s15_performance`）
//! - **アロケーションの回数** もテストで確かめます（[`alloc_counter`] を使って数えています）
//! - **速さ** はテストでは確かめません。実行するたびにばらつくので、テストにすると
//!   通ったり落ちたりする「不安定なテスト」になるからです。
//!   代わりに、実装が終わったら次の実験で比べてください:
//!
//! ```text
//! cargo run -p s15_performance --example compare --release
//! ```

#![allow(
    unused_variables,
    unused_imports,
    unused_mut,
    dead_code,
    clippy::ptr_arg
)]

pub mod alloc_counter;
pub mod ex01_allocations;
pub mod ex02_complexity;
pub mod ex03_hashing;

// テストのときだけ、アロケーションを数えるアロケータを使う。
// （ライブラリを使う側のプログラムには影響しない）
#[cfg(test)]
#[global_allocator]
static ALLOC: alloc_counter::CountingAlloc = alloc_counter::CountingAlloc;
