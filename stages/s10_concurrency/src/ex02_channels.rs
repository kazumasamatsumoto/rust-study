//! ## 課題 02: チャネルでメッセージを送る
//!
//! `std::sync::mpsc::channel()` は、送信側 `Sender` と受信側 `Receiver` のペアを作る。
//! - mpsc = multiple producer, single consumer（送信側は複数、受信側は1つ）
//! - `tx.send(値)` で送ると、値の**所有権が受信側に移る**。送った側はもう触れない
//! - `rx.recv()` は、値が届くまで待つ。全ての `Sender` が drop されると `Err` を返す
//! - `for v in rx { ... }` と書くと、全ての送信側が drop されるまで受け取り続ける
//!
//! ⚠️ よくある落とし穴: 元の `tx` を drop し忘れると、受信側は「まだ誰か送ってくるかも」と永遠に待ち続ける。

use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

/// 3段のパイプラインを作る。
///
/// ```text
/// [生成スレッド] --(i64)--> [二乗スレッド] --(i64)--> [main: 偶数だけ集める]
/// ```
///
/// - 生成スレッド: `inputs` を順番に送る
/// - 二乗スレッド: 受け取った値を二乗して次に送る
/// - 呼び出し元: 受け取った値のうち偶数だけを、届いた順に Vec に集めて返す
///
/// 各段の送信者は1つだけなので、順番は保たれる。
pub fn square_pipeline(inputs: Vec<i64>) -> Vec<i64> {
    todo!()
}

/// 文章の各行を `n_workers` 個のワーカースレッドに分担させ、単語の出現回数を数える。
/// - 各ワーカーは自分の担当分の `HashMap<String, usize>` を作り、チャネルで送る
/// - 呼び出し元は届いた HashMap を全て合算して返す
/// - 単語は `split_whitespace` で区切り、小文字にして数える
/// - `n_workers` が 0 なら 1 として扱う
///
/// ヒント: `tx.clone()` で送信側を複製して、各ワーカーに1つずつ持たせる。
pub fn word_count(lines: Vec<String>, n_workers: usize) -> HashMap<String, usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_pipeline() {
        assert_eq!(square_pipeline(vec![1, 2, 3, 4, -6]), vec![4, 16, 36]);
        assert_eq!(square_pipeline(vec![]), Vec::<i64>::new());
        let big: Vec<i64> = (0..10_000).collect();
        let expected: Vec<i64> = (0..10_000).step_by(2).map(|x| x * x).collect();
        assert_eq!(square_pipeline(big), expected);
    }

    #[test]
    fn test_word_count() {
        let lines: Vec<String> = [
            "the quick brown fox",
            "The lazy dog",
            "the fox jumps",
            "",
            "DOG dog",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        for n in [0, 1, 2, 3, 10] {
            let c = word_count(lines.clone(), n);
            assert_eq!(c.get("the"), Some(&3), "n_workers = {n}");
            assert_eq!(c.get("dog"), Some(&3), "n_workers = {n}");
            assert_eq!(c.get("fox"), Some(&2), "n_workers = {n}");
            assert_eq!(c.len(), 7, "n_workers = {n}");
        }
        assert!(word_count(vec![], 3).is_empty());
    }
}
