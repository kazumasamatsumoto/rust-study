//! ## 課題 03: async の裏側
//!
//! - CPU を長時間使う処理は、async のタスクの中で直接やってはいけない
//!   → `tokio::task::spawn_blocking` で別のスレッドに逃がす
//! - `async fn` の正体は `Future` トレイトを実装した値。自分で実装してみる
//!
//! 先に `cargo run -p s11_async --example blocking_demo` を実行しておくこと。

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// わざと遅い（CPU を使い続ける）ハッシュ関数。（実装済み）
pub fn slow_hash(data: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for _ in 0..1000 {
        for b in data {
            h ^= *b as u64;
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    h
}

/// `slow_hash` を `spawn_blocking` で別スレッドで計算し、結果を返す。
///
/// 考えてみよう: なぜ引数が `&[u8]` ではなく `Vec<u8>` なのか？
pub async fn hash_in_background(data: Vec<u8>) -> u64 {
    todo!()
}

/// 複数のデータのハッシュを、それぞれ `spawn_blocking` で並列に計算する。
/// 結果は入力と同じ順番で返すこと。
pub async fn hash_all(inputs: Vec<Vec<u8>>) -> Vec<u64> {
    todo!()
}

/// 自作の Future: poll されるたびにカウントを1つ減らし、
/// 0 になったら `"liftoff"` を返して完了する。
///
/// - `remaining` が 0 なら `Poll::Ready("liftoff")`
/// - そうでなければ、1減らして `Poll::Pending`
///
/// **重要**: `Pending` を返すときは、「また poll してほしい」とランタイムに
/// 伝えなければならない。伝えないと、このタスクは二度と poll されず、永遠に止まる。
/// ヒント: `cx.waker().wake_by_ref()`
pub struct Countdown {
    pub remaining: u32,
}

impl Future for Countdown {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::task::Waker;

    #[tokio::test]
    async fn test_hash_in_background() {
        let data = b"hello async".to_vec();
        let expected = slow_hash(&data);
        assert_eq!(hash_in_background(data).await, expected);
    }

    #[tokio::test]
    async fn test_hash_all_keeps_order() {
        let inputs: Vec<Vec<u8>> = (0..8u8).map(|i| vec![i; 1000]).collect();
        let expected: Vec<u64> = inputs.iter().map(|d| slow_hash(d)).collect();
        assert_eq!(hash_all(inputs).await, expected);
    }

    /// ランタイムを使わず、手で poll してみる
    #[test]
    fn test_countdown_manual_poll() {
        let mut cd = Countdown { remaining: 2 };
        // 何もしない Waker を使って、手で Context を作る
        let mut cx = Context::from_waker(Waker::noop());
        let mut cd = Pin::new(&mut cd);
        assert_eq!(cd.as_mut().poll(&mut cx), Poll::Pending);
        assert_eq!(cd.as_mut().poll(&mut cx), Poll::Pending);
        assert_eq!(cd.as_mut().poll(&mut cx), Poll::Ready("liftoff"));
    }

    /// ランタイムに poll してもらう（wake を忘れると、このテストは終わらない）
    #[tokio::test]
    async fn test_countdown_await() {
        let r = tokio::time::timeout(
            std::time::Duration::from_secs(1),
            Countdown { remaining: 5 },
        )
        .await;
        assert_eq!(r, Ok("liftoff"), "wake_by_ref を呼び忘れていませんか？");
    }
}
