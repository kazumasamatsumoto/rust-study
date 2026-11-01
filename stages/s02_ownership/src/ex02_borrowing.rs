//! ## 課題 02: 借用（参照）
//!
//! 所有権を渡さずに値を使いたいときは参照を渡します。
//! - 読むだけ → `&T`
//! - 書き換える → `&mut T`

/// 文字列の末尾に "!" を追加する（戻り値なし、引数を直接書き換える）。
pub fn exclaim(s: &mut String) {
    todo!()
}

/// スライスの各要素を、その位置までの累積和で置き換える。
/// 例: [1, 2, 3, 4] -> [1, 3, 6, 10]
pub fn prefix_sum_in_place(v: &mut [i32]) {
    todo!()
}

/// 最初の単語（最初の空白より前）を返す。空白がなければ全体を返す。
///
/// 戻り値が `&str` であることに注目。新しい文字列を作らず、
/// 引数の一部を「借りて」返しています。
pub fn first_word(s: &str) -> &str {
    todo!()
}

/// 2つの Vec のうち合計値が大きい方への参照を返す（同じなら a）。
///
/// 戻り値の `'a` は「ライフタイム」です。今は「戻り値は a か b と同じだけ
/// 生きている」という意味だと思っておけば OK。Stage 07 で詳しくやります。
pub fn larger_sum<'a>(a: &'a [i32], b: &'a [i32]) -> &'a [i32] {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exclaim() {
        let mut s = String::from("hello");
        exclaim(&mut s);
        exclaim(&mut s);
        assert_eq!(s, "hello!!");
    }

    #[test]
    fn test_prefix_sum_in_place() {
        let mut v = vec![1, 2, 3, 4];
        prefix_sum_in_place(&mut v);
        assert_eq!(v, vec![1, 3, 6, 10]);

        let mut empty: Vec<i32> = vec![];
        prefix_sum_in_place(&mut empty);
        assert!(empty.is_empty());
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("rust"), "rust");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn test_larger_sum() {
        let a = vec![1, 2, 3];
        let b = vec![10];
        assert_eq!(larger_sum(&a, &b), &[10]);
        assert_eq!(larger_sum(&[5], &[2, 3]), &[5]);
    }
}
