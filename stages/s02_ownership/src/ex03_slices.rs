//! ## 課題 03: スライスと Option
//!
//! `&[T]` は配列や Vec の一部を指すスライスです。
//! 値が「ないかもしれない」ときは `Option<T>` を返します。

/// 偶数と奇数に分けて (偶数のVec, 奇数のVec) を返す。順序は保つこと。
pub fn split_even_odd(v: &[i32]) -> (Vec<i32>, Vec<i32>) {
    todo!()
}

/// 真ん中の要素への参照を返す。要素数が偶数なら右寄りの方。空なら None。
/// 例: [1,2,3] -> Some(&2), [1,2,3,4] -> Some(&3)
pub fn middle(v: &[i32]) -> Option<&i32> {
    todo!()
}

/// 最大値を持つ要素のインデックスを返す。同じ最大値が複数あれば最初のもの。
pub fn index_of_max(v: &[i32]) -> Option<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_even_odd() {
        assert_eq!(
            split_even_odd(&[1, 2, 3, 4, 5, 6]),
            (vec![2, 4, 6], vec![1, 3, 5])
        );
        assert_eq!(split_even_odd(&[]), (vec![], vec![]));
        assert_eq!(split_even_odd(&[-2, -1]), (vec![-2], vec![-1]));
    }

    #[test]
    fn test_middle() {
        assert_eq!(middle(&[]), None);
        assert_eq!(middle(&[7]), Some(&7));
        assert_eq!(middle(&[1, 2, 3]), Some(&2));
        assert_eq!(middle(&[1, 2, 3, 4]), Some(&3));
    }

    #[test]
    fn test_index_of_max() {
        assert_eq!(index_of_max(&[]), None);
        assert_eq!(index_of_max(&[3, 9, 2, 9]), Some(1));
        assert_eq!(index_of_max(&[-5, -1, -3]), Some(1));
    }
}
