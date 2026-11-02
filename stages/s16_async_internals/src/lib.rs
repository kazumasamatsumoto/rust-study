//! # Stage 16: async の仕組みを自作して理解する
//!
//! Stage 11 では tokio を「使う」側でした。この Stage では、tokio が裏でやっていることを
//! **外部クレートを一切使わずに**、標準ライブラリだけで自作します。
//!
//! - 課題 01: Future を手で実装する（async fn が生成するステートマシンを手で書く）
//! - 課題 02: Waker と block_on を自作する（「起こされるまで眠る」仕組み）
//! - 課題 03: 小さな実行器（executor）を自作する（複数のタスクを並行に動かす）
//!
//! 進め方:
//! 1. `README.md` を読む（前提知識の CS基礎 09 と Stage 11 も）
//! 2. 実験を実行する:
//!    `cargo run -p s16_async_internals --example future_size`
//!    `cargo run -p s16_async_internals --example self_ref`
//! 3. `ex01_manual_futures.rs` から順に `todo!()` を実装する。
//!    **課題 03 は課題 02 の `TimerFuture` を使うので、02 を先に解くこと**
//!
//! 課題 02・03 のテストは、実装を間違えると永遠に止まる（ハングする）ことがあります。
//! そうならないよう、テストは別スレッドで実行して、5 秒で打ち切るようにしてあります。

#![allow(
    unused_variables,
    unused_imports,
    unused_mut,
    dead_code,
    clippy::ptr_arg
)]

pub mod ex01_manual_futures;
pub mod ex02_waker_block_on;
pub mod ex03_executor;

/// テスト用: `f` を別スレッドで実行し、`secs` 秒以内に終わらなければ失敗させる。
///
/// Waker の呼び忘れなどでハングしたときに、テストが永遠に止まらないようにするためのもの。
#[cfg(test)]
pub(crate) fn run_with_timeout<T: Send + 'static>(
    secs: u64,
    hint: &str,
    f: impl FnOnce() -> T + Send + 'static,
) -> T {
    use std::sync::mpsc;
    use std::time::Duration;

    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(f());
    });
    match rx.recv_timeout(Duration::from_secs(secs)) {
        Ok(v) => v,
        Err(mpsc::RecvTimeoutError::Timeout) => {
            panic!("{secs} 秒たっても終わりませんでした。{hint}")
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            panic!("実行中に panic しました（todo!() のままか、実装の中で panic しています）")
        }
    }
}
