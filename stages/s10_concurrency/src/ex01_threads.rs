//! ## 課題 01: スレッドを作って、待つ
//!
//! - `std::thread::spawn(クロージャ)` で新しいスレッドを作る。戻り値は `JoinHandle<T>`
//! - `handle.join()` でスレッドの終了を待ち、クロージャの戻り値を受け取る
//! - `spawn` に渡すクロージャは、呼び出し元より長く生きるかもしれない。
//!   そのため、借用ではなく**所有権ごと**渡す必要がある（`move` クロージャ）
//! - `std::thread::scope` を使うと、「スコープを抜ける前に必ず全員 join する」ことが保証されるので、
//!   ローカル変数を**借用したまま**スレッドに渡せる

use std::thread;

/// 名前ごとにスレッドを1つ作り、各スレッドで `"hello, <名前>"` を作って返す。
/// 結果は **入力と同じ順番** で返すこと（スレッドが終わる順番は毎回違うことに注意）。
///
/// ヒント: `names.into_iter().map(|name| thread::spawn(move || ...))` で JoinHandle の Vec を作り、
/// 順番に join する。
pub fn greet_all(names: Vec<String>) -> Vec<String> {
    todo!()
}

/// `data` を `n_threads` 個のかたまりに分け、各スレッドで合計を計算し、最後に全部を足す。
/// - `data` の所有権を受け取っているので、かたまりごとに Vec を作ってスレッドに **move** できる
/// - `n_threads` が 0 なら 1 として扱う
///
/// ヒント: `data.chunks(size)` でかたまりに分けられる。`size` は切り上げの割り算で求める。
pub fn parallel_sum(data: Vec<u64>, n_threads: usize) -> u64 {
    todo!()
}

/// `data` の各要素に `f` を適用した結果を、元の順番のまま返す。
/// `n_threads` 個のスレッドで分担すること。`n_threads` が 0 なら 1 として扱う。
///
/// この関数は `data` を **借用** しているだけなので、`thread::spawn` ではうまくいかない。
/// `thread::scope` を使おう。
pub fn parallel_map(data: &[u64], n_threads: usize, f: fn(u64) -> u64) -> Vec<u64> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_all_keeps_order() {
        let names: Vec<String> = ["alice", "bob", "carol", "dave"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        // 何回やっても同じ順番になることを確かめる
        for _ in 0..20 {
            assert_eq!(
                greet_all(names.clone()),
                vec!["hello, alice", "hello, bob", "hello, carol", "hello, dave"]
            );
        }
        assert!(greet_all(vec![]).is_empty());
    }

    #[test]
    fn test_parallel_sum() {
        let data: Vec<u64> = (1..=10_000).collect();
        for n in [0, 1, 3, 4, 7, 16, 20_000] {
            assert_eq!(parallel_sum(data.clone(), n), 50_005_000, "n_threads = {n}");
        }
        assert_eq!(parallel_sum(vec![], 4), 0);
    }

    #[test]
    fn test_parallel_map() {
        let data: Vec<u64> = (0..1000).collect();
        let expected: Vec<u64> = data.iter().map(|x| x * x).collect();
        for n in [0, 1, 2, 7, 1000, 5000] {
            assert_eq!(
                parallel_map(&data, n, |x| x * x),
                expected,
                "n_threads = {n}"
            );
        }
        // data は借用されただけなので、まだ使える
        assert_eq!(data.len(), 1000);
        assert!(parallel_map(&[], 4, |x| x).is_empty());
    }
}
