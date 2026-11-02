//! ## 課題 01: 生ポインタと unsafe
//!
//! - 参照 `&T` / `&mut T` から生ポインタ `*const T` / `*mut T` を**作る**のは安全
//! - 生ポインタの**参照外し**（`*p` で中身を読み書きする）は unsafe
//! - `unsafe fn` を**呼ぶ**のも unsafe
//!
//! `std::ptr` モジュールと、ポインタのメソッド（`add`, `read`, `write`）のドキュメントを読みながら進めよう。

use std::ptr;

/// 2 つの値を、生ポインタを経由して入れ替える。
///
/// （普段は `std::mem::swap(a, b)` を使えば済みます。生ポインタの練習です。）
///
/// ヒント:
/// - `let pa: *mut i32 = a;` のように、`&mut i32` は `*mut i32` に変換できる
/// - `unsafe { *pa }` で読み、`unsafe { *pa = 1 }` で書ける
/// - `std::ptr::swap(pa, pb)` という関数もある（これも unsafe）。
///   一時変数を使って手で入れ替えると、clippy に「`std::mem::swap` を使え」と言われるので、両方試してみよう
/// - 直前に `// SAFETY: ...` を書くのを忘れずに
pub fn swap_via_raw(a: &mut i32, b: &mut i32) {
    todo!()
}

/// スライスの `i` 番目の要素を、**範囲チェックなしで**返す。
///
/// # Safety
///
/// 呼び出す側は `i < s.len()` を保証しなければならない。
/// 守られない場合、スライスの外のメモリを読むことになり、未定義動作になる。
///
/// ヒント: `s.as_ptr()` で先頭の生ポインタを取り、`.add(i)` で i 個先に進める。
/// Rust 2024 では、`unsafe fn` の中でも、unsafe な操作は `unsafe { }` ブロックで囲む（囲まないと警告が出る）。
pub unsafe fn get_unchecked_copy(s: &[i32], i: usize) -> i32 {
    todo!()
}

/// 標準ライブラリの `split_at_mut` を自作する。
///
/// `s` を `[0, mid)` と `[mid, len)` の 2 つの **可変** スライスに分ける。
/// `mid > s.len()` なら panic する（メッセージは問わない）。
///
/// なぜこれが unsafe なしでは書けないのかは README の「借用チェッカーの限界」を参照。
///
/// ヒント: `s.as_mut_ptr()` と `std::slice::from_raw_parts_mut(ptr, len)`
pub fn my_split_at_mut<T>(s: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    todo!()
}

/// バイト列の先頭 4 バイトを、リトルエンディアンの `u32` として読む。4 バイト未満なら `None`。
///
/// **`*(bytes.as_ptr() as *const u32)` と書いてはいけません。** `bytes` の先頭が 4 の倍数の
/// アドレスにあるとは限らないからです（README の「アラインメント」を参照）。
/// `ptr::read_unaligned` を使ってください。
///
/// （普段は `u32::from_le_bytes` を使えば unsafe なしで書けます。答え合わせに使おう。）
pub fn read_u32_le(bytes: &[u8]) -> Option<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_via_raw() {
        let mut a = 1;
        let mut b = 2;
        swap_via_raw(&mut a, &mut b);
        assert_eq!((a, b), (2, 1));
    }

    #[test]
    fn test_get_unchecked_copy() {
        let v = [10, 20, 30];
        // SAFETY: 0 と 2 はどちらも v.len() == 3 より小さい
        let (first, last) = unsafe { (get_unchecked_copy(&v, 0), get_unchecked_copy(&v, 2)) };
        assert_eq!((first, last), (10, 30));
    }

    #[test]
    fn test_my_split_at_mut() {
        let mut v = [1, 2, 3, 4, 5];
        let (left, right) = my_split_at_mut(&mut v, 2);
        assert_eq!(left, &[1, 2]);
        assert_eq!(right, &[3, 4, 5]);
        // 2 つの可変スライスを同時に使える
        left[0] = 100;
        right[0] = 300;
        assert_eq!(v, [100, 2, 300, 4, 5]);
    }

    #[test]
    fn test_my_split_at_mut_edges() {
        let mut v = vec![String::from("a"), String::from("b")];
        let (l, r) = my_split_at_mut(&mut v, 0);
        assert!(l.is_empty());
        assert_eq!(r.len(), 2);
        let (l, r) = my_split_at_mut(&mut v, 2);
        assert_eq!(l.len(), 2);
        assert!(r.is_empty());
    }

    #[test]
    #[should_panic]
    fn test_my_split_at_mut_out_of_range() {
        let mut v = [1, 2, 3];
        my_split_at_mut(&mut v, 4);
    }

    #[test]
    fn test_read_u32_le() {
        assert_eq!(read_u32_le(&[1, 0, 0, 0]), Some(1));
        assert_eq!(
            read_u32_le(&[0x78, 0x56, 0x34, 0x12, 0xFF]),
            Some(0x1234_5678)
        );
        assert_eq!(read_u32_le(&[1, 2, 3]), None);
    }

    #[test]
    fn test_read_u32_le_unaligned() {
        // 1 バイトずらした位置から読む。アドレスが 4 の倍数でなくても正しく読めること
        let buf = [0u8, 0xEF, 0xBE, 0xAD, 0xDE];
        for offset in 0..2 {
            let expected = u32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap());
            assert_eq!(read_u32_le(&buf[offset..]), Some(expected));
        }
    }
}
