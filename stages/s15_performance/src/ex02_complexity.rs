//! ## 課題 02: 計算量を下げる・clone を減らす
//!
//! アロケーションを減らすより、もっと効果が大きいのが「計算量（CS基礎 06）を下げる」ことです。
//! O(n²) を O(n) にすると、データが 1 万件なら 1 万倍近く速くなりえます。
//! 細かい最適化より先に、まずここを疑いましょう。

use std::collections::{HashMap, HashSet};

// ---------------------------------------------------------------------------
// (1) 重複の検出
// ---------------------------------------------------------------------------

/// 遅い実装: 全ての組を比べる O(n²)。
pub fn has_duplicates_slow(v: &[u64]) -> bool {
    for i in 0..v.len() {
        for j in (i + 1)..v.len() {
            if v[i] == v[j] {
                return true;
            }
        }
    }
    false
}

/// 同じ値が 2 回以上出てくれば true。O(n)（平均）で書くこと。
/// 入力の並び順を変えてはいけない（`&[u64]` なので、そもそも変えられない）。
pub fn has_duplicates(v: &[u64]) -> bool {
    todo!()
}

// ---------------------------------------------------------------------------
// (2) 2 つの一覧の共通部分
// ---------------------------------------------------------------------------

/// 遅い実装: a の各要素について b を全部調べる O(n × m)。
pub fn common_slow(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut out = Vec::new();
    for x in a {
        if b.contains(x) && !out.contains(x) {
            out.push(*x);
        }
    }
    out
}

/// a と b の両方に含まれる値を、**a に最初に出てきた順**に、重複なしで返す。
/// O(n + m)（平均）で書くこと。
pub fn common(a: &[u64], b: &[u64]) -> Vec<u64> {
    todo!()
}

// ---------------------------------------------------------------------------
// (3) 単語の頻度: clone と to_string を減らす
// ---------------------------------------------------------------------------

/// 遅い実装: 単語ごとに String を作り、最後に全体を clone して並べ替えている。
pub fn most_frequent_slow(text: &str) -> Option<(String, usize)> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for w in text.split_whitespace() {
        *counts.entry(w.to_string()).or_insert(0) += 1;
    }
    let mut all: Vec<(String, usize)> = counts.clone().into_iter().collect();
    all.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    all.first().cloned()
}

/// 空白で区切った単語のうち、最も多く出てくるものと、その回数を返す。
/// 同じ回数のものが複数あれば、辞書順で最も小さいもの。単語がなければ None。
/// 大文字と小文字は区別する。
///
/// 条件:
/// - 戻り値の単語は `&str`（text を借りたまま）。String を作らない
/// - 全体を並べ替えない（最大のものを 1 つ探すだけなら O(n) で済む）
/// - アロケーションは HashMap の確保・拡張の分だけ（テストでは「単語の数」より十分少ないことを確かめる）
pub fn most_frequent(text: &str) -> Option<(&str, usize)> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alloc_counter::count_allocs;

    #[test]
    fn test_has_duplicates() {
        for v in [
            vec![],
            vec![1],
            vec![1, 2, 3],
            vec![1, 2, 1],
            vec![5, 5],
            (0..1000).collect::<Vec<u64>>(),
        ] {
            assert_eq!(has_duplicates(&v), has_duplicates_slow(&v), "input: {v:?}");
        }
    }

    #[test]
    fn test_has_duplicates_large() {
        // O(n²) だと、この大きさでは数秒以上かかる
        let mut v: Vec<u64> = (0..200_000).collect();
        assert!(!has_duplicates(&v));
        v.push(123_456);
        assert!(has_duplicates(&v));
    }

    #[test]
    fn test_common() {
        let cases: Vec<(Vec<u64>, Vec<u64>)> = vec![
            (vec![], vec![1, 2]),
            (vec![1, 2, 3], vec![]),
            (vec![3, 1, 2, 3, 1], vec![1, 3, 9]),
            (vec![1, 2, 3], vec![4, 5, 6]),
            (vec![7, 7, 7], vec![7]),
        ];
        for (a, b) in cases {
            assert_eq!(common(&a, &b), common_slow(&a, &b), "a={a:?} b={b:?}");
        }
    }

    #[test]
    fn test_common_large() {
        let a: Vec<u64> = (0..100_000).collect();
        let b: Vec<u64> = (50_000..150_000).rev().collect();
        let c = common(&a, &b);
        assert_eq!(c.len(), 50_000);
        assert_eq!(c[0], 50_000);
        assert_eq!(*c.last().unwrap(), 99_999);
    }

    #[test]
    fn test_most_frequent() {
        for text in [
            "",
            "   ",
            "a",
            "b a b a",
            "the cat and the dog and the bird",
            "Rust rust RUST rust",
            "x y z",
        ] {
            let fast = most_frequent(text).map(|(w, n)| (w.to_string(), n));
            assert_eq!(fast, most_frequent_slow(text), "input: {text:?}");
        }
    }

    #[test]
    fn test_most_frequent_allocs() {
        let words = ["alpha", "beta", "gamma", "delta", "epsilon"];
        let text: String = (0..10_000)
            .map(|i| words[i % words.len()])
            .collect::<Vec<_>>()
            .join(" ");
        let (r, n) = count_allocs(|| most_frequent(&text));
        assert_eq!(r, Some(("alpha", 2000)));
        assert!(
            n <= 10,
            "アロケーションが {n} 回起きています（単語は 10,000 個）"
        );
    }
}
