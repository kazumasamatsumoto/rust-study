//! ## 課題 03: FFI (Foreign Function Interface)
//!
//! C の標準ライブラリ (libc) の関数を Rust から呼びます。
//! Rust の標準ライブラリはもともと libc にリンクしているので、追加の設定なしで呼べます。
//!
//! FFI の境界では、Rust の保証が全て消えます。C の関数は、
//! - 引数のポインタが有効かどうかを確かめない
//! - 文字列の終わりを NUL バイト (`\0`) で判断する（Rust の `&str` は長さを持っていて、NUL で終わらない）
//! - 一部の入力で未定義動作になる
//!
//! この課題の目的は、**C の関数を、絶対に未定義動作を起こさない安全な Rust の関数で包む**ことです。

use std::ffi::{CStr, CString, c_char, c_int, c_void};

// C の関数の宣言。Rust 2024 では `unsafe extern` と書く必要がある。
// 「この宣言（引数や戻り値の型）が本物と一致している」ことを、書いた人が保証するという意味。
unsafe extern "C" {
    /// `size_t strlen(const char *s);` — NUL バイトまでのバイト数を返す
    fn strlen(s: *const c_char) -> usize;

    /// `int abs(int x);` — 絶対値。**`abs(INT_MIN)` は C では未定義動作**
    fn abs(x: c_int) -> c_int;

    /// `void qsort(void *base, size_t nmemb, size_t size, int (*compar)(const void *, const void *));`
    fn qsort(
        base: *mut c_void,
        nmemb: usize,
        size: usize,
        compar: extern "C" fn(*const c_void, *const c_void) -> c_int,
    );
}

/// C の `abs` を安全に呼ぶ。C で未定義動作になる入力 (`i32::MIN`) では、呼ばずに `None` を返す。
///
/// なぜ `i32::MIN` の絶対値が `i32` で表せないのかは、CS基礎 01 の「2の補数」を参照。
pub fn c_abs(x: i32) -> Option<i32> {
    todo!()
}

/// Rust の文字列を C の文字列に変換して、C の `strlen` で長さを測る。
/// 文字列の途中に NUL バイトが含まれていて C の文字列にできない場合は `None`。
///
/// ヒント: `CString::new(s)` は、NUL を含んでいると `Err` を返す。
/// `CString` は末尾に NUL を付けた、C 向けの文字列。`.as_ptr()` で `*const c_char` が得られる。
pub fn c_strlen(s: &str) -> Option<usize> {
    todo!()
}

/// C の関数が書き込んだ固定長バッファ（NUL で終わる文字列のあとにゴミが続く）から、
/// Rust の `String` を取り出す。
///
/// - NUL が見つからない → `None`（C 側のバグ。バッファの外まで読んではいけない）
/// - UTF-8 として不正 → `None`
///
/// ヒント: `CStr::from_bytes_until_nul` は、**unsafe なしで**使える安全な API。
/// 生ポインタから `CStr::from_ptr` で作る方法もあるが、長さが分からないので危険。
pub fn string_from_c_buffer(buf: &[u8]) -> Option<String> {
    todo!()
}

/// C の `qsort` に渡す比較関数。`a` と `b` は `i32` を指すポインタとして渡される。
/// `*a < *b` なら負、等しければ 0、`*a > *b` なら正の値を返す。
///
/// `extern "C"` が付いているので、C から呼べる関数になっている。
///
/// **注意**: `extern "C"` の関数の中で panic すると、プロセス全体が即座に異常終了します
/// （C には Rust の panic を受け止める仕組みがないため）。そのため、この関数だけは
/// `todo!()` ではなく、仮に `0` を返すスタブにしてあります。実装でも panic しうる処理
/// （`unwrap` など）を書かないこと。
///
/// ヒント: `a.cast::<i32>()` で `*const i32` にできる。`i32::cmp` の結果を `as c_int` で変換できる。
/// `(x - y)` を返す書き方はオーバーフローするので不可（なぜか考えよう）。
pub extern "C" fn compare_i32(a: *const c_void, b: *const c_void) -> c_int {
    // TODO: 仮の実装（常に「等しい」と答える）。書き直すこと。
    0
}

/// C の `qsort` を使って、`i32` のスライスを昇順に並べ替える。
///
/// ヒント: `v.as_mut_ptr().cast::<c_void>()`、要素数、`std::mem::size_of::<i32>()`、`compare_i32`
pub fn c_sort(v: &mut [i32]) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_abs() {
        assert_eq!(c_abs(-5), Some(5));
        assert_eq!(c_abs(7), Some(7));
        assert_eq!(c_abs(0), Some(0));
        assert_eq!(c_abs(i32::MAX), Some(i32::MAX));
        assert_eq!(c_abs(i32::MIN + 1), Some(i32::MAX));
        assert_eq!(c_abs(i32::MIN), None);
    }

    #[test]
    fn test_c_strlen() {
        assert_eq!(c_strlen(""), Some(0));
        assert_eq!(c_strlen("hello"), Some(5));
        // strlen が数えるのは「文字数」ではなく「バイト数」（CS基礎 05）
        assert_eq!(c_strlen("こんにちは"), Some(15));
        assert_eq!(c_strlen("ab\0cd"), None);
    }

    #[test]
    fn test_string_from_c_buffer() {
        let mut buf = [0xFFu8; 16]; // ゴミで埋まったバッファ
        buf[..5].copy_from_slice(b"rust\0");
        assert_eq!(string_from_c_buffer(&buf), Some("rust".to_string()));

        assert_eq!(string_from_c_buffer(b"\0garbage"), Some(String::new()));
        assert_eq!(string_from_c_buffer(b"no terminator"), None);
        assert_eq!(string_from_c_buffer(&[0xFF, 0xFE, 0x00]), None); // 不正な UTF-8
    }

    #[test]
    fn test_compare_i32() {
        let (a, b, c) = (1i32, 2i32, i32::MIN);
        let p = |x: &i32| (x as *const i32).cast::<c_void>();
        assert!(compare_i32(p(&a), p(&b)) < 0);
        assert!(compare_i32(p(&b), p(&a)) > 0);
        assert_eq!(compare_i32(p(&a), p(&a)), 0);
        // 引き算で書くとここでオーバーフローする
        assert!(compare_i32(p(&b), p(&c)) > 0);
    }

    #[test]
    fn test_c_sort() {
        let mut v = vec![5, -3, 9, 0, i32::MIN, i32::MAX, 2];
        c_sort(&mut v);
        assert_eq!(v, vec![i32::MIN, -3, 0, 2, 5, 9, i32::MAX]);

        let mut empty: Vec<i32> = vec![];
        c_sort(&mut empty);
        assert!(empty.is_empty());
    }
}
