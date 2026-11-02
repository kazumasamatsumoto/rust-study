//! 実験: ビジーループと「眠って待つ」の比較
//! **課題 01 と 02 を解いた後に**実行してください（解く前は todo!() で panic します）。
//!
//! cargo run -p s16_async_internals --example busy_vs_park --release
//!
//! 同じ 100ms のタイマーを、
//! - 課題 01 の busy_block_on（Waker を無視して poll し続ける）
//! - 課題 02 の block_on（Pending なら park で眠り、wake されたら起きる）
//!
//! で待ち、poll された回数と経過時間を比べます。

use s16_async_internals::ex01_manual_futures::busy_block_on;
use s16_async_internals::ex02_waker_block_on::{TimerFuture, block_on};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

/// poll された回数を数える Future のラッパー
struct CountPolls<F> {
    inner: F,
    polls: Arc<AtomicU32>,
}

impl<F: Future + Unpin> Future for CountPolls<F> {
    type Output = F::Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<F::Output> {
        let this = self.get_mut();
        this.polls.fetch_add(1, Ordering::SeqCst);
        Pin::new(&mut this.inner).poll(cx)
    }
}

fn timer_100ms(polls: &Arc<AtomicU32>) -> CountPolls<TimerFuture> {
    CountPolls {
        inner: TimerFuture::new(Duration::from_millis(100)),
        polls: polls.clone(),
    }
}

fn main() {
    let polls = Arc::new(AtomicU32::new(0));
    let start = Instant::now();
    busy_block_on(timer_100ms(&polls));
    println!(
        "busy_block_on: poll {:>10} 回, {:?}",
        polls.load(Ordering::SeqCst),
        start.elapsed()
    );

    let polls = Arc::new(AtomicU32::new(0));
    let start = Instant::now();
    block_on(timer_100ms(&polls));
    println!(
        "block_on     : poll {:>10} 回, {:?}",
        polls.load(Ordering::SeqCst),
        start.elapsed()
    );
}
