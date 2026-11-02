//! ## 課題 01: アロケーションを減らす
//!
//! ヒープの確保（アロケーション）は、アロケータが空き領域を探す必要があるので、
//! スタック上の計算に比べてずっと遅い処理です（CS基礎 03）。
//! 性能の改善で一番効果が出やすいのは、多くの場合「アロケーションの回数を減らすこと」です。
//!
//! この課題では、遅い実装と同じ結果を返しつつ、アロケーションの回数を
//! テストで決められた回数以下に抑えます。

use std::borrow::Cow;

// ---------------------------------------------------------------------------
// (1) 文字列の連結
// ---------------------------------------------------------------------------

/// 遅い実装: 単語ごとに新しい String を作り直している。
pub fn join_slow(words: &[&str], sep: &str) -> String {
    let mut out = String::new();
    for (i, w) in words.iter().enumerate() {
        if i == 0 {
            out = w.to_string();
        } else {
            out = format!("{out}{sep}{w}");
        }
    }
    out
}

/// words を sep で繋ぐ。join_slow と同じ結果を返すこと。
///
/// 条件: アロケーションは **最大 1 回**。
/// ヒント: 最終的な長さは、連結する前に計算できる。`String::with_capacity` と `push_str`。
/// （標準ライブラリの `words.join(sep)` も 1 回で済むが、ここでは自分で書いてみること）
pub fn join(words: &[&str], sep: &str) -> String {
    todo!()
}

// ---------------------------------------------------------------------------
// (2) 必要なときだけ新しい文字列を作る: Cow
// ---------------------------------------------------------------------------

/// 遅い実装: 必ず新しい String を作る。
pub fn normalize_spaces_slow(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// 空白文字（`char::is_whitespace`）の並びを 1 つのスペース ' ' にまとめ、前後の空白を取り除く。
/// normalize_spaces_slow と同じ内容の文字列を返すこと。
///
/// 条件: 入力が**すでに正規化されている**（前後に空白がなく、空白はスペース 1 つずつ）なら、
/// アロケーションは **0 回**（`Cow::Borrowed` で入力をそのまま返す）。
/// 正規化が必要な場合は、アロケーションは **最大 1 回**。
///
/// 現実のデータの大半がすでに綺麗なら、この最適化はとても効く。
pub fn normalize_spaces(s: &str) -> Cow<'_, str> {
    todo!()
}

// ---------------------------------------------------------------------------
// (3) バッファの使い回し
// ---------------------------------------------------------------------------

/// 遅い実装: 呼ぶたびに Vec<String> を作り、フィールドごとに String を確保する。
pub fn fields_slow(line: &str) -> Vec<String> {
    line.split(',').map(|f| f.trim().to_string()).collect()
}

/// line をカンマで区切り、前後の空白を除いたフィールドを out に入れる。
/// out の元の中身は消す。
///
/// 条件:
/// - フィールドは `&str` のまま（line を借りたまま）入れる。String を作らない
/// - out の容量が足りていれば、アロケーションは **0 回**
///
/// 大量の行を処理するとき、呼び出し側が 1 つの Vec を使い回せば、
/// 最初の数行の後はアロケーションが一切起きなくなる。
pub fn fields_into<'a>(line: &'a str, out: &mut Vec<&'a str>) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alloc_counter::count_allocs;

    #[test]
    fn test_join_same_result() {
        for (words, sep) in [
            (vec![], ", "),
            (vec!["a"], ", "),
            (vec!["a", "b", "c"], ", "),
            (vec!["Rust", "は", "速い"], ""),
            (vec!["", "", ""], "-"),
        ] {
            assert_eq!(join(&words, sep), join_slow(&words, sep));
        }
    }

    #[test]
    fn test_join_allocs() {
        let words: Vec<&str> = (0..1000).map(|_| "word").collect();
        let (s, n) = count_allocs(|| join(&words, " "));
        assert_eq!(s.len(), 1000 * 4 + 999);
        assert!(
            n <= 1,
            "アロケーションが {n} 回起きています（1 回以下にしてください）"
        );
        let (_, slow) = count_allocs(|| join_slow(&words, " "));
        assert!(slow > 100, "比較用: join_slow は {slow} 回");
    }

    #[test]
    fn test_normalize_same_result() {
        for s in [
            "",
            "   ",
            "hello",
            "hello world",
            "  hello   world  ",
            "a\tb\nc",
            "日本語　の 全角スペース",
        ] {
            assert_eq!(
                normalize_spaces(s),
                normalize_spaces_slow(s),
                "input: {s:?}"
            );
        }
    }

    #[test]
    fn test_normalize_allocs() {
        let (r, n) = count_allocs(|| normalize_spaces("already clean text"));
        assert!(
            matches!(r, Cow::Borrowed(_)),
            "綺麗な入力は Borrowed で返す"
        );
        assert_eq!(n, 0);

        let (r, n) = count_allocs(|| normalize_spaces("  needs   fixing  "));
        assert_eq!(r, "needs fixing");
        assert!(n <= 1, "アロケーションが {n} 回起きています");
    }

    #[test]
    fn test_fields_same_result() {
        let mut out = Vec::new();
        for line in ["a,b,c", " a , b ,c ", "", "x", "1,,3", "名前, 年齢"] {
            fields_into(line, &mut out);
            assert_eq!(out, fields_slow(line), "input: {line:?}");
        }
    }

    #[test]
    fn test_fields_reuse_buffer() {
        let lines: Vec<String> = (0..1000)
            .map(|i| format!("{i}, name{i}, {}", i * 2))
            .collect();
        let mut out = Vec::with_capacity(8);
        let (_, n) = count_allocs(|| {
            for line in &lines {
                fields_into(line, &mut out);
                assert_eq!(out.len(), 3);
            }
        });
        assert_eq!(
            n, 0,
            "バッファを使い回せていません（{n} 回のアロケーション）"
        );
    }
}
