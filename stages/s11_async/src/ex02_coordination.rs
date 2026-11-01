//! ## 課題 02: タイムアウト・リトライ・チャネル
//!
//! 実務の非同期処理で必ず出てくる3つのパターンです。
//!
//! ヒント: `tokio::select!`, `tokio::time::sleep`, `tokio::sync::mpsc`

use std::future::Future;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

/// fut が `limit` 以内に終われば `Some(結果)`、終わらなければ `None` を返す。
///
/// `tokio::select!` で「fut」と「sleep(limit)」を競争させてみよう。
/// 先に終わった方の分岐が実行され、負けた方は **キャンセル（drop）** されます。
///
/// （実務では `tokio::time::timeout` という関数がそのまま使えます。
///   ここでは中身を理解するために自分で書きます）
pub async fn with_timeout<F, T>(fut: F, limit: Duration) -> Option<T>
where
    F: Future<Output = T>,
{
    todo!()
}

/// 失敗するかもしれない処理 `op` を、最大 `max_attempts` 回まで試す。
/// - 成功したらすぐにその値を返す
/// - 失敗したら待ってから再挑戦する。待ち時間は 100ms, 200ms, 400ms, ... と倍々にする
///   （指数バックオフ。相手のサーバが弱っているときに、負荷をかけすぎないため）
/// - 最後まで失敗したら、最後のエラーを返す
///
/// `op` は呼ぶたびに新しい Future を返す関数です。
pub async fn retry<F, Fut, T, E>(mut op: F, max_attempts: u32) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    todo!()
}

/// 生産者・消費者パターン。
/// - 生産者タスクを `tokio::spawn` で作り、1 から n までの数をチャネルに送る
/// - この関数（消費者）は、受け取った数を全て足して返す
///
/// 考えてみよう: 生産者が送り終わったことを、消費者はどうやって知るのか？
/// （ヒント: 送信側 `Sender` が全て drop されると、`recv()` は `None` を返す）
pub async fn sum_via_channel(n: u64) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use tokio::time::Instant;

    async fn slow_value(ms: u64) -> &'static str {
        sleep(Duration::from_millis(ms)).await;
        "done"
    }

    #[tokio::test(start_paused = true)]
    async fn test_with_timeout_ok() {
        let r = with_timeout(slow_value(50), Duration::from_millis(100)).await;
        assert_eq!(r, Some("done"));
    }

    #[tokio::test(start_paused = true)]
    async fn test_with_timeout_expired() {
        let start = Instant::now();
        let r = with_timeout(slow_value(500), Duration::from_millis(100)).await;
        assert_eq!(r, None);
        // 500ms 待たずに、100ms で諦めていること
        assert_eq!(start.elapsed(), Duration::from_millis(100));
    }

    #[tokio::test(start_paused = true)]
    async fn test_retry_succeeds_on_third_attempt() {
        let calls = Cell::new(0);
        let start = Instant::now();
        let r: Result<u32, &str> = retry(
            || {
                calls.set(calls.get() + 1);
                let n = calls.get();
                async move { if n < 3 { Err("busy") } else { Ok(n) } }
            },
            5,
        )
        .await;
        assert_eq!(r, Ok(3));
        assert_eq!(calls.get(), 3);
        // 失敗後に 100ms + 200ms 待っている
        assert_eq!(start.elapsed(), Duration::from_millis(300));
    }

    #[tokio::test(start_paused = true)]
    async fn test_retry_gives_up() {
        let calls = Cell::new(0);
        let start = Instant::now();
        let r: Result<(), String> = retry(
            || {
                calls.set(calls.get() + 1);
                let n = calls.get();
                async move { Err(format!("fail {n}")) }
            },
            3,
        )
        .await;
        assert_eq!(r, Err("fail 3".to_string()));
        assert_eq!(calls.get(), 3);
        // 最後の失敗の後は待たない: 100ms + 200ms
        assert_eq!(start.elapsed(), Duration::from_millis(300));
    }

    #[tokio::test]
    async fn test_sum_via_channel() {
        assert_eq!(sum_via_channel(0).await, 0);
        assert_eq!(sum_via_channel(10).await, 55);
        assert_eq!(sum_via_channel(10_000).await, 50_005_000);
    }
}
