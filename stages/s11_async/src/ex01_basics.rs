//! ## 課題 01: async fn と並行実行
//!
//! - `async fn` は呼んだだけでは実行されない。`.await` して初めて動く
//! - 順番に `.await` すると、待ち時間は足し算になる
//! - `tokio::join!` や `tokio::spawn` を使うと、待ち時間を重ねられる
//!
//! ヒント: `tokio::join!`, `tokio::spawn`, `tokio::task::JoinSet`

use std::time::Duration;
use tokio::time::sleep;

/// 商品の価格を「問い合わせる」。ネットワーク越しの API の代わり。
/// 1回の問い合わせに 100ms かかる。（この関数は実装済み）
pub async fn fetch_price(item: &str) -> u32 {
    sleep(Duration::from_millis(100)).await;
    match item {
        "apple" => 120,
        "bread" => 250,
        "milk" => 180,
        _ => 0,
    }
}

/// 2つの商品の価格を **同時に** 問い合わせて、両方を返す。
/// かかる時間は 200ms ではなく 100ms になるはず。
///
/// ヒント: `tokio::join!(a, b)` は、2つの Future を同時に進めて、両方の結果を返します。
pub async fn fetch_two(a: &str, b: &str) -> (u32, u32) {
    todo!()
}

/// 全ての商品の価格を **1つずつ順番に** 問い合わせて、合計を返す。
/// n 個なら n × 100ms かかる。（比較用）
pub async fn total_sequential(items: &[&str]) -> u32 {
    todo!()
}

/// 全ての商品の価格を **同時に** 問い合わせて、合計を返す。
/// 何個あっても 100ms で終わるはず。
///
/// `tokio::spawn` でタスクを作ってみよう。
/// 考えてみよう: なぜ引数が `&[&str]` ではなく `Vec<String>` なのか？
/// （ヒント: `tokio::spawn` のシグネチャにある `'static`）
pub async fn total_concurrent(items: Vec<String>) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Instant;

    #[tokio::test(start_paused = true)]
    async fn test_fetch_two_is_concurrent() {
        let start = Instant::now();
        assert_eq!(fetch_two("apple", "milk").await, (120, 180));
        assert_eq!(start.elapsed(), Duration::from_millis(100));
    }

    #[tokio::test(start_paused = true)]
    async fn test_total_sequential() {
        let start = Instant::now();
        assert_eq!(total_sequential(&["apple", "bread", "milk"]).await, 550);
        assert_eq!(start.elapsed(), Duration::from_millis(300));
    }

    #[tokio::test(start_paused = true)]
    async fn test_total_concurrent() {
        let items: Vec<String> = ["apple", "bread", "milk", "apple", "unknown"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let start = Instant::now();
        assert_eq!(total_concurrent(items).await, 670);
        assert_eq!(start.elapsed(), Duration::from_millis(100));
    }

    #[tokio::test(start_paused = true)]
    async fn test_total_concurrent_empty() {
        assert_eq!(total_concurrent(vec![]).await, 0);
    }
}
