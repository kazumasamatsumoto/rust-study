//! ## 課題 02: 参照を持つ構造体
//!
//! 構造体のフィールドに参照を持たせると、構造体にもライフタイム引数が必要になります。
//! `Tokenizer<'a>` は「`'a` の間だけ生きている入力を借りている」という意味で、
//! **Tokenizer は入力の文字列より長く生きられません。**
//!
//! 入力をコピーしない（ゼロコピー）ので、大きなテキストを解析するときに速くなります。

/// 空白で区切られた単語を、前から1つずつ取り出す。
pub struct Tokenizer<'a> {
    input: &'a str,
    /// 次に読み始めるバイト位置
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        todo!()
    }

    /// 次の単語を返す。もう単語がなければ None。
    ///
    /// ⭐ 戻り値が `Option<&'a str>` であることが重要。
    /// `&mut self` ではなく**入力**から借りているので、返した単語を持ったまま
    /// 次の `next_word` を呼べる（`test_words_outlive_borrow` を参照）。
    /// README の「`&self` に結びつけてしまう失敗」も読むこと。
    ///
    /// ヒント: `self.input[self.pos..]` で残りを取り出し、
    /// `trim_start` や `find(char::is_whitespace)` で単語の範囲を求める。
    pub fn next_word(&mut self) -> Option<&'a str> {
        todo!()
    }

    /// まだ読んでいない残りの部分（先頭の空白は除く）。
    pub fn rest(&self) -> &'a str {
        todo!()
    }
}

/// 文章の中で一番長い単語を、文章から借りて保持する。
#[derive(Debug, PartialEq)]
pub struct Highlight<'a> {
    pub word: &'a str,
    /// 何番目の単語か（0始まり）
    pub index: usize,
}

/// `Tokenizer` を使って一番長い単語を探す（同じ長さなら先に出たもの）。単語がなければ None。
pub fn longest_word(text: &str) -> Option<Highlight<'_>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_word() {
        let mut t = Tokenizer::new("  let x  = 42 ");
        assert_eq!(t.next_word(), Some("let"));
        assert_eq!(t.next_word(), Some("x"));
        assert_eq!(t.rest(), "= 42 ");
        assert_eq!(t.next_word(), Some("="));
        assert_eq!(t.next_word(), Some("42"));
        assert_eq!(t.next_word(), None);
        assert_eq!(t.next_word(), None);
        assert_eq!(t.rest(), "");
    }

    #[test]
    fn test_words_outlive_borrow() {
        let source = String::from("fn main ( )");
        let (first, second);
        {
            let mut t = Tokenizer::new(&source);
            // 1つ目の単語を持ったまま2つ目を取り出せる
            first = t.next_word().unwrap();
            second = t.next_word().unwrap();
        } // Tokenizer はここで消えるが、単語は source から借りているので使える
        assert_eq!((first, second), ("fn", "main"));
    }

    #[test]
    fn test_longest_word() {
        assert_eq!(
            longest_word("the quick brown fox"),
            Some(Highlight {
                word: "quick",
                index: 1
            })
        );
        assert_eq!(longest_word("   "), None);
    }
}
