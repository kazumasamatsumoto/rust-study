//! ## 課題 01: Vec / HashMap / HashSet / BTreeMap
//!
//! ヒント:
//! - `HashMap::entry(key).or_insert(0)` で「なければ 0 を入れて、その値への可変参照を得る」
//! - `HashSet::insert` は、新しく追加できたら true を返す
//! - `HashMap` の反復順序は決まっていない。順序が必要なら並べ替えるか `BTreeMap` を使う

use std::collections::{BTreeMap, HashMap, HashSet};

/// 単語ごとの出現回数を数える。
/// - 英数字以外の文字（空白、句読点など）を区切りとみなす
/// - 大文字と小文字は区別しない（小文字にそろえる）
///
/// 例: "Hello, hello world!" -> {"hello": 2, "world": 1}
pub fn word_count(text: &str) -> HashMap<String, usize> {
    todo!()
}

/// 出現回数の多い順に上位 n 個を返す。
/// 回数が同じなら、単語の辞書順（昇順）で並べる。
///
/// 考えてみよう: なぜ「回数が同じなら辞書順」という決まりが必要なのか？
pub fn top_n(counts: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    todo!()
}

/// 単語を文字数（`chars().count()`）ごとにまとめる。
/// 同じ文字数のグループの中では、入力の順序を保つ。
///
/// 戻り値が `BTreeMap` なのは、キー（文字数）の順に取り出せるようにするため。
pub fn group_by_length(words: &[&str]) -> BTreeMap<usize, Vec<String>> {
    todo!()
}

/// 重複を取り除く。最初に出てきた順序を保つ。
/// 例: [3, 1, 3, 2, 1] -> [3, 1, 2]
pub fn dedup_keep_order(v: &[i32]) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count() {
        let c = word_count("Hello, hello world! Rust's world.");
        assert_eq!(c.get("hello"), Some(&2));
        assert_eq!(c.get("world"), Some(&2));
        assert_eq!(c.get("rust"), Some(&1));
        assert_eq!(c.get("s"), Some(&1));
        assert_eq!(c.len(), 4);
        assert!(word_count("  ...  ").is_empty());
    }

    #[test]
    fn test_top_n() {
        let c = word_count("b a c b a b d");
        assert_eq!(
            top_n(&c, 3),
            vec![
                ("b".to_string(), 3),
                ("a".to_string(), 2),
                ("c".to_string(), 1)
            ]
        );
        assert_eq!(top_n(&c, 10).len(), 4);
        assert!(top_n(&c, 0).is_empty());
    }

    #[test]
    fn test_group_by_length() {
        let g = group_by_length(&["rust", "go", "java", "c", "あい", "zig"]);
        let expected: BTreeMap<usize, Vec<String>> = [
            (1, vec!["c".to_string()]),
            (2, vec!["go".to_string(), "あい".to_string()]),
            (3, vec!["zig".to_string()]),
            (4, vec!["rust".to_string(), "java".to_string()]),
        ]
        .into_iter()
        .collect();
        assert_eq!(g, expected);
    }

    #[test]
    fn test_dedup_keep_order() {
        assert_eq!(dedup_keep_order(&[3, 1, 3, 2, 1]), vec![3, 1, 2]);
        assert_eq!(dedup_keep_order(&[]), Vec::<i32>::new());
        assert_eq!(dedup_keep_order(&[5, 5, 5]), vec![5]);
    }
}
