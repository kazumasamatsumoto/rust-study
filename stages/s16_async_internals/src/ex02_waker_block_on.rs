//! ## 課題 02: Waker と block_on を自作する
//!
//! 課題 01 の `busy_block_on` は、`Pending` が返ってきてもすぐにまた poll する
//! 「ビジーループ」でした。待っている間も CPU を 100% 使い続けてしまいます。
//!
//! 本物の実行器は、`Pending` が返ってきたら**眠り**、Waker が呼ばれたら**起きて**
//! もう一度 poll します。この課題では、その仕組みを作ります。
//!
//! ```text
//!  block_on のスレッド                 タイマーのスレッド
//!  ───────────────────                 ──────────────────
//!  poll() → Pending
//!   (Waker を Future に渡してある)
//!  thread::park()  ← 眠る              sleep(50ms)
//!        :                             完了フラグを立てる
//!        :                    ←─────── waker.wake()  (= unpark)
//!  起きる
//!  poll() → Ready(値)
//! ```
//!
//! 使う道具:
//! - `std::thread::park()`: 今のスレッドを眠らせる。`unpark()` されるまで起きない
//!   （ただし、まれに理由なく起きることがある。これを「見せかけの起床」と呼ぶ）
//! - `Thread::unpark()`: 眠っているスレッドを起こす。**先に unpark されていたら、
//!   次の park はすぐに戻る**（この性質のおかげで、起こし損ねが起きない）
//! - `std::task::Wake` トレイト: これを実装した型を `Arc` で包むと、`Waker::from` で Waker にできる

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};
use std::time::Duration;

// ---------------------------------------------------------------------------
// 1. スレッドを起こす Waker
// ---------------------------------------------------------------------------

/// wake されたら、持っているスレッドを unpark する Waker の中身。
pub struct ThreadWaker {
    thread: Thread,
}

impl Wake for ThreadWaker {
    /// 持っているスレッドを起こす。
    fn wake(self: Arc<Self>) {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// 2. block_on
// ---------------------------------------------------------------------------

/// `fut` を完了まで実行して、結果を返す。
///
/// 1. `fut` をピン留めする（`std::pin::pin!`）
/// 2. 今のスレッド（`thread::current()`）を起こす Waker を作り、`Context` を作る
/// 3. ループ: poll して `Ready(v)` なら `v` を返す。`Pending` なら `thread::park()` で眠る
///
/// 見せかけの起床で早く起きてしまっても、もう一度 poll して `Pending` ならまた眠るだけなので、
/// このループの形なら正しく動きます。
pub fn block_on<F: Future>(fut: F) -> F::Output {
    todo!()
}

// ---------------------------------------------------------------------------
// 3. TimerFuture: 指定した時間がたったら完了する Future
// ---------------------------------------------------------------------------

/// Future と、タイマーのスレッドが共有する状態。
struct SharedState {
    /// 時間がたったら true になる
    completed: bool,
    /// 最後に poll されたときの Waker。タイマーのスレッドは、これを呼んで知らせる
    waker: Option<Waker>,
}

/// `duration` がたったら `Ready(())` になる Future。
///
/// 作ったときに、別のスレッドを1つ起動します。そのスレッドは `duration` だけ眠ったあと、
/// `completed` を true にして、保存されている Waker を呼びます。
///
/// （本物の tokio はタイマーごとにスレッドを作ったりしません。README の比較表を参照）
pub struct TimerFuture {
    shared: Arc<Mutex<SharedState>>,
}

impl TimerFuture {
    /// タイマーを作り、スレッドを起動する。
    ///
    /// スレッドの中で:
    /// 1. `thread::sleep(duration)`
    /// 2. ロックを取って `completed = true` にする
    /// 3. `waker` が保存されていれば、取り出して `wake()` する
    pub fn new(duration: Duration) -> Self {
        todo!()
    }
}

impl Future for TimerFuture {
    type Output = ();

    /// ロックを取り、
    /// - `completed` なら `Ready(())`
    /// - そうでなければ、`cx.waker().clone()` を `waker` に保存して `Pending`
    ///
    /// 考えよう: 「completed を確認する」と「Waker を保存する」を、
    /// **同じロックの中で**やらないと、どんな問題が起きるでしょうか？（README の問い）
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run_with_timeout;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::time::Instant;

    const HINT: &str = "Pending を返すときに Waker を保存しているか、タイマーのスレッドが wake を呼んでいるか、ThreadWaker が unpark しているかを確認してください。";

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

    #[test]
    fn test_block_on_ready_value() {
        let v = run_with_timeout(5, HINT, || block_on(async { 1 + 1 }));
        assert_eq!(v, 2);
    }

    #[test]
    fn test_block_on_wakes_itself() {
        // 何回か Pending を返して自分で wake する Future（課題 01 の yield_n）でも完了できる
        let v = run_with_timeout(5, HINT, || {
            block_on(async {
                crate::ex01_manual_futures::yield_n(3).await;
                "done"
            })
        });
        assert_eq!(v, "done");
    }

    #[test]
    fn test_timer_waits() {
        let elapsed = run_with_timeout(5, HINT, || {
            let start = Instant::now();
            block_on(TimerFuture::new(Duration::from_millis(50)));
            start.elapsed()
        });
        assert!(elapsed >= Duration::from_millis(50), "{elapsed:?}");
    }

    #[test]
    fn test_timers_in_sequence() {
        let elapsed = run_with_timeout(5, HINT, || {
            let start = Instant::now();
            block_on(async {
                TimerFuture::new(Duration::from_millis(30)).await;
                TimerFuture::new(Duration::from_millis(30)).await;
            });
            start.elapsed()
        });
        // 順番に await しているので、待ち時間は足し算になる
        assert!(elapsed >= Duration::from_millis(60), "{elapsed:?}");
    }

    #[test]
    fn test_block_on_does_not_busy_loop() {
        let polls = Arc::new(AtomicU32::new(0));
        let p = polls.clone();
        run_with_timeout(5, HINT, move || {
            block_on(CountPolls {
                inner: TimerFuture::new(Duration::from_millis(100)),
                polls: p,
            })
        });
        let n = polls.load(Ordering::SeqCst);
        // 理想は 2 回（最初の Pending と、起こされた後の Ready）。
        // 見せかけの起床がまれにあるので少し余裕を持たせるが、ビジーループなら何万回にもなる
        assert!(
            n <= 5,
            "{n} 回も poll されました。Pending のあと park で眠っていますか？"
        );
    }

    #[test]
    fn test_timer_already_finished_before_first_poll() {
        // 最初の poll の前にタイマーが終わっていても、正しく Ready になること
        let v = run_with_timeout(5, HINT, || {
            let timer = TimerFuture::new(Duration::from_millis(1));
            thread::sleep(Duration::from_millis(50));
            block_on(timer);
            "ok"
        });
        assert_eq!(v, "ok");
    }
}
