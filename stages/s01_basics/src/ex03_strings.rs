//! ## 課題 03: 文字列
//!
//! Rust には主に2つの文字列型があります。
//! - `&str`   : 借用した文字列スライス（読むだけ）
//! - `String` : 所有している、伸び縮みできる文字列
//!
//! ヒント: `.chars()`, `.split_whitespace()`, `.to_uppercase()`, `.collect()`
//! を調べてみましょう。

/// 母音 (a, e, i, o, u。大文字も含む) の数を数える。
pub fn count_vowels(s: &str) -> usize {
    todo!()
}

/// 単語の順番を逆にする。単語間はスペース1つで繋ぐ。
/// 例: "hello  rust world" -> "world rust hello"
pub fn reverse_words(s: &str) -> String {
    todo!()
}

/// 各単語の先頭を大文字にする。
/// 例: "hello rust world" -> "Hello Rust World"
pub fn capitalize_words(s: &str) -> String {
    todo!()
}

/// 回文かどうか（大文字小文字・空白を無視）。
/// 例: "A man a plan a canal Panama" -> true
pub fn is_palindrome(s: &str) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_vowels() {
        assert_eq!(count_vowels(""), 0);
        assert_eq!(count_vowels("rhythm"), 0);
        assert_eq!(count_vowels("Hello World"), 3);
        assert_eq!(count_vowels("AEIOU aeiou"), 10);
    }

    #[test]
    fn test_reverse_words() {
        assert_eq!(reverse_words("hello"), "hello");
        assert_eq!(reverse_words("hello  rust world"), "world rust hello");
        assert_eq!(reverse_words(""), "");
    }

    #[test]
    fn test_capitalize_words() {
        assert_eq!(capitalize_words("hello rust world"), "Hello Rust World");
        assert_eq!(capitalize_words("a"), "A");
        assert_eq!(capitalize_words(""), "");
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("A man a plan a canal Panama"));
        assert!(!is_palindrome("rust"));
        assert!(is_palindrome(""));
    }
}
