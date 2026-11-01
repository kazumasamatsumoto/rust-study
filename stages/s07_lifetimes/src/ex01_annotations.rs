//! ## 課題 01: ライフタイム注釈を読む
//!
//! シグネチャのライフタイム注釈は「戻り値の参照が、どの引数から借りたものか」を表します。
//! 実装する前に、各関数のシグネチャを声に出して読んでみましょう。
//! 例: `longest<'a>(a: &'a str, b: &'a str) -> &'a str`
//!   →「戻り値は a か b のどちらかから借りたもので、両方が生きている間だけ有効」

/// 長いほうの文字列を返す（同じ長さなら a）。長さはバイト数でよい。
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    todo!()
}

/// 最初の行を返す（改行を含まない）。改行がなければ全体。
///
/// 注釈が書かれていないことに注目。引数の参照が1つだけなので、
/// 「戻り値は s から借りている」とコンパイラが推論できる（省略規則）。
pub fn first_line(s: &str) -> &str {
    todo!()
}

/// s が prefix で始まっていれば、それを取り除いた残りを返す。
/// 始まっていなければ s をそのまま返す。
///
/// 戻り値は `'a`、つまり s だけに結びついている。prefix には別のライフタイム
/// （省略されているが、`'a` とは無関係）が付いている。
/// そのおかげで、prefix が先に消えても戻り値は使い続けられる（テストを参照）。
pub fn strip_or_self<'a>(s: &'a str, prefix: &str) -> &'a str {
    todo!()
}

/// 単語の一覧から、needle を含む最初の単語を返す。
///
/// `words: &[&'a str]` は「`'a` の文字列への参照を並べたスライス」。
/// 戻り値は**スライス本体ではなく、中の文字列**から借りているので `'a`。
pub fn find_containing<'a>(words: &[&'a str], needle: &str) -> Option<&'a str> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("hello", "hi"), "hello");
        assert_eq!(longest("ab", "cd"), "ab");
        assert_eq!(longest("", "x"), "x");
    }

    #[test]
    fn test_first_line() {
        assert_eq!(first_line("one\ntwo\nthree"), "one");
        assert_eq!(first_line("single"), "single");
        assert_eq!(first_line(""), "");
    }

    #[test]
    fn test_strip_or_self() {
        assert_eq!(strip_or_self("v1.2.3", "v"), "1.2.3");
        assert_eq!(strip_or_self("1.2.3", "v"), "1.2.3");

        let text = String::from("https://example.com");
        let rest;
        {
            let prefix = String::from("https://");
            rest = strip_or_self(&text, &prefix);
        } // prefix はここで消える
        // 戻り値は text にだけ結びついているので、まだ使える
        assert_eq!(rest, "example.com");
    }

    #[test]
    fn test_find_containing() {
        let text = String::from("apple banana cherry");
        let found;
        {
            // words（スライス本体）はこのブロックで消える
            let words: Vec<&str> = text.split(' ').collect();
            found = find_containing(&words, "nan");
        }
        // 戻り値は text から借りているので、words が消えても使える
        assert_eq!(found, Some("banana"));

        let words = ["a", "b"];
        assert_eq!(find_containing(&words, "z"), None);
    }
}
