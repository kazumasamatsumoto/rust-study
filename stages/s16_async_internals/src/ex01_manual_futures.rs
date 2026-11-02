//! ## 課題 01: Future を手で実装する
//!
//! `async fn` を書くと、コンパイラが `Future` トレイトを実装したステートマシンを作ります。
//! この課題では、それを**手で**書きます。
//!
//! ```text
//! pub trait Future {
//!     type Output;
//!     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
//! }
//! ```
//!
//! 守るべき約束（Future の契約）:
//! 1. 終わったら `Poll::Ready(値)` を返す
//! 2. まだなら `Poll::Pending` を返す。**そのときは、後で `cx.waker()` が呼ばれるようにしておく**
//! 3. `Ready` を返した後に、もう一度 poll されることは想定しなくてよい（呼んだ側の誤り）
//!
//! この課題の Future は、全て `Unpin` な型（ムーブしても壊れない型）だけで作ります。
//! そのため `self: Pin<&mut Self>` から `self.get_mut()` で普通の `&mut Self` を取り出せます。
//! なぜそれで良いのかは README の「Pin」の節で説明します。

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

// ---------------------------------------------------------------------------
// 1. Ready: すぐに完了する Future
// ---------------------------------------------------------------------------

/// 最初の poll で、持っている値を返して完了する Future。
/// `std::future::ready` と同じものです。
pub struct Ready<T>(Option<T>);

/// `Ready` を作る。
pub fn ready<T>(value: T) -> Ready<T> {
    Ready(Some(value))
}

impl<T: Unpin> Future for Ready<T> {
    type Output = T;

    /// 中身の値を取り出して `Ready` で返す。
    /// ヒント: `Option::take` で、値を取り出して `None` を残せます。
    /// `Ready` の後にもう一度 poll されたら、panic して構いません（契約違反なので）。
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// 2. YieldN: n 回 Pending を返してから完了する Future
// ---------------------------------------------------------------------------

/// n 回 `Pending` を返し、その次の poll で `Ready(())` を返す Future。
///
/// 「I/O の準備がまだできていない」状態を、n 回くりかえす真似をします。
/// `Pending` を返すときは、**毎回** `cx.waker().wake_by_ref()` を呼んで
/// 「すぐにまた poll してください」と実行器に頼むこと（呼ばないと二度と起こされません）。
pub struct YieldN {
    remaining: u32,
}

pub fn yield_n(n: u32) -> YieldN {
    YieldN { remaining: n }
}

impl Future for YieldN {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// 3. Then: 2つの Future を順に実行する（ステートマシン）
// ---------------------------------------------------------------------------

/// `first` が終わったら、その結果を `f` に渡して2つ目の Future を作り、それを実行する。
///
/// ```text
/// async {
///     let x = first.await;
///     f(x).await
/// }
/// ```
///
/// を手で書いたものです。状態は3つあります。
///
/// - `First`: 1つ目を実行中。2つ目を作るための関数 `f` を持っている
/// - `Second`: 2つ目を実行中
/// - `Done`: 完了した
///
/// `f` を `Option` で包んでいるのは、`FnOnce` を呼ぶと `f` が消費される（ムーブされる）ので、
/// `&mut self` から取り出すために `take()` を使いたいからです。
pub enum Then<A, B, F> {
    First { fut: A, f: Option<F> },
    Second { fut: B },
    Done,
}

/// `Then` を作る。
pub fn then<A, B, F>(first: A, f: F) -> Then<A, B, F>
where
    A: Future,
    B: Future,
    F: FnOnce(A::Output) -> B,
{
    Then::First {
        fut: first,
        f: Some(f),
    }
}

impl<A, B, F> Future for Then<A, B, F>
where
    A: Future + Unpin,
    B: Future + Unpin,
    F: FnOnce(A::Output) -> B + Unpin,
{
    type Output = B::Output;

    /// ループの中で `match` を使って状態を進める。
    ///
    /// - `First` なら1つ目を poll する。`Pending` ならそのまま `Pending` を返す。
    ///   `Ready(x)` なら `f(x)` で2つ目を作り、状態を `Second` に変えて、**ループを続ける**
    ///   （同じ poll の中で、すぐに2つ目も poll する）
    /// - `Second` なら2つ目を poll する。`Ready(y)` なら状態を `Done` にして `Ready(y)` を返す
    /// - `Done` で poll されたら panic してよい
    ///
    /// ヒント: 中の Future を poll するには `Pin::new(&mut fut).poll(cx)` と書きます
    /// （中の Future が `Unpin` だから使える書き方です）。
    /// 状態を書き換えるには `*this = Then::Second { fut: ... };` のように代入します。
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<B::Output> {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// 4. 何度も poll して完了させる（いちばん素朴な「実行器」）
// ---------------------------------------------------------------------------

/// `fut` を `Ready` になるまで poll し続け、(結果, poll した回数) を返す。
///
/// Waker には `Waker::noop()`（呼ばれても何もしない Waker）を使います。
/// Waker を無視して、とにかく poll し続けるので、CPU を使い続ける「ビジーループ」です。
/// 課題 02 では、これを「起こされるまで眠る」方式に改良します。
///
/// ヒント: `async { }` で作った Future は `Unpin` ではないので、
/// `std::pin::pin!(fut)` でピン留めしてから poll します。
/// `let mut fut = std::pin::pin!(fut);` とすると `fut.as_mut().poll(&mut cx)` と書けます。
pub fn busy_block_on<F: Future>(fut: F) -> (F::Output, u32) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::Wake;

    /// 何回 wake されたかを数える Waker
    struct CountingWaker(AtomicUsize);

    impl Wake for CountingWaker {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
        fn wake_by_ref(self: &Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    fn counting_waker() -> (Arc<CountingWaker>, Waker) {
        let inner = Arc::new(CountingWaker(AtomicUsize::new(0)));
        (inner.clone(), Waker::from(inner))
    }

    #[test]
    fn test_ready() {
        let mut fut = ready(42);
        let mut cx = Context::from_waker(Waker::noop());
        assert_eq!(Pin::new(&mut fut).poll(&mut cx), Poll::Ready(42));
    }

    #[test]
    fn test_yield_n_returns_pending_n_times() {
        let (count, waker) = counting_waker();
        let mut cx = Context::from_waker(&waker);
        let mut fut = yield_n(3);

        for i in 1..=3 {
            assert_eq!(Pin::new(&mut fut).poll(&mut cx), Poll::Pending, "{i} 回目");
        }
        assert_eq!(Pin::new(&mut fut).poll(&mut cx), Poll::Ready(()));
        assert_eq!(
            count.0.load(Ordering::SeqCst),
            3,
            "Pending を返すたびに wake_by_ref を呼ぶこと"
        );
    }

    #[test]
    fn test_yield_zero_is_ready_immediately() {
        let mut cx = Context::from_waker(Waker::noop());
        assert_eq!(Pin::new(&mut yield_n(0)).poll(&mut cx), Poll::Ready(()));
    }

    #[test]
    fn test_then_runs_in_order() {
        let (count, waker) = counting_waker();
        let mut cx = Context::from_waker(&waker);
        // yield_n(2) が終わったら、ready(10) を作る
        let mut fut = then(yield_n(2), |()| ready(10));

        assert_eq!(Pin::new(&mut fut).poll(&mut cx), Poll::Pending);
        assert_eq!(Pin::new(&mut fut).poll(&mut cx), Poll::Pending);
        // 3 回目: 1つ目が Ready になり、同じ poll の中で2つ目も Ready になる
        assert_eq!(Pin::new(&mut fut).poll(&mut cx), Poll::Ready(10));
        assert_eq!(count.0.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_then_passes_value() {
        let fut = then(ready(20), |x| then(yield_n(1), move |()| ready(x + 1)));
        assert_eq!(busy_block_on(fut), (21, 2));
    }

    #[test]
    fn test_busy_block_on_counts_polls() {
        assert_eq!(busy_block_on(ready("hi")), ("hi", 1));
        assert_eq!(busy_block_on(yield_n(4)), ((), 5));
    }

    #[test]
    fn test_busy_block_on_runs_async_block() {
        // async ブロックは Unpin ではないが、busy_block_on の中でピン留めするので動く
        let fut = async {
            yield_n(2).await;
            let a = ready(1).await;
            yield_n(1).await;
            a + 2
        };
        assert_eq!(busy_block_on(fut), (3, 4));
    }
}
