//! ## 課題 01: ムーブとクローン
//!
//! `Vec` や `String` を関数に値で渡すと、所有権が関数に「ムーブ」します。
//! 呼び出し側ではもう使えません。返したいなら戻り値で返す必要があります。

/// v を受け取り、末尾に 42 を追加して返す。
/// （所有権を受け取って、また返す）
///
/// ヒント: 引数を変更するには `mut v: Vec<i32>` と書けます。
pub fn push_42(v: Vec<i32>) -> Vec<i32> {
    todo!()
}

/// s の所有権を受け取り、その文字列を2回繰り返した新しい String を返す。
/// 例: "ab" -> "abab"
pub fn repeat_twice(s: String) -> String {
    todo!()
}

/// 元の Vec は残したまま、要素を2倍にした「新しい」Vec を返す。
///
/// 考えてみよう: なぜこの関数は `Vec<i32>` ではなく `&Vec<i32>`
/// （より良くは `&[i32]`）を受け取るのか？
pub fn doubled(v: &[i32]) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_42() {
        let v = vec![1, 2];
        let v = push_42(v);
        assert_eq!(v, vec![1, 2, 42]);
    }

    #[test]
    fn test_repeat_twice() {
        assert_eq!(repeat_twice(String::from("ab")), "abab");
        assert_eq!(repeat_twice(String::new()), "");
    }

    #[test]
    fn test_doubled_keeps_original() {
        let original = vec![1, 2, 3];
        let d = doubled(&original);
        assert_eq!(d, vec![2, 4, 6]);
        // original はムーブされていないので、まだ使える
        assert_eq!(original, vec![1, 2, 3]);
    }
}
