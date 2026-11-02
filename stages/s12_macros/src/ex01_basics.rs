//! ## 課題 01: `macro_rules!` の基本
//!
//! ```text
//! macro_rules! 名前 {
//!     (パターン) => { 展開結果 };
//!     (パターン) => { 展開結果 };   // 上から順に試される
//! }
//! ```
//!
//! よく使うフラグメント指定子:
//! - `$x:expr`  式（`1 + 2`, `foo()`, `v[0]` など）
//! - `$x:ident` 識別子（変数名・関数名・型名）
//! - `$x:ty`    型（`i32`, `Vec<String>` など）
//! - `$x:tt`    トークン木 1 つ（何でも受け取れる万能型）
//! - `$x:literal` リテラル（`1`, `"abc"` など）
//!
//! `#[macro_export]` を付けると、クレートのルートから `crate::名前!` で使えるようになります。
//! 他のクレートに公開したいマクロには必ず付けます。

/// 式を2乗する。
///
/// 例: `square!(3)` → `9`、`square!(1 + 2)` → `9`
///
/// **注意 1**: `$x * $x` と素直に書くと、`square!(1 + 2)` はどうなるでしょう？
/// （C のマクロとの違いを README で確認しよう）
///
/// **注意 2**: 引数の式は **1 回だけ** 評価すること。
/// `square!(next())` で `next()` が 2 回呼ばれてはいけません（テストで確認しています）。
///
/// ヒント: 展開結果をブロック `{ let v = $x; ... }` にすると、式を 1 回だけ評価できます。
#[macro_export]
macro_rules! square {
    // ↓ 仮の定義。消して書き直す
    ($($t:tt)*) => {
        $crate::__todo()
    };
}

/// 1 個以上の式の最大値を返す。
///
/// 例: `max_of!(3)` → `3`、`max_of!(3, 9, 2)` → `9`
///
/// ヒント: パターンを 2 つ書く。
/// - 要素が 1 個のとき: その値
/// - 2 個以上のとき: `$first:expr, $($rest:expr),+` と分けて、残りに対して自分自身を **再帰的に** 呼ぶ
///
/// 比較には `std::cmp::max` を使ってよい。
#[macro_export]
macro_rules! max_of {
    // ↓ 仮の定義。消して書き直す
    ($($t:tt)*) => {
        $crate::__todo()
    };
}

/// `HashMap` をリテラルのように作る。
///
/// ```text
/// let m = hashmap! {
///     "apple" => 3,
///     "banana" => 5,   // 末尾のカンマがあってもなくてもよい
/// };
/// ```
///
/// 空の `hashmap! {}` も書けること。
///
/// ヒント:
/// - 繰り返し: `$( $k:expr => $v:expr ),*`
/// - 末尾のカンマを許す: `$(,)?`
/// - 展開側でも `$( map.insert($k, $v); )*` のように繰り返せる
/// - 標準ライブラリの型は `::std::collections::HashMap` とフルパスで書く（理由は README の「衛生性」）
#[macro_export]
macro_rules! hashmap {
    // ↓ 仮の定義。消して書き直す
    ($($t:tt)*) => {
        $crate::__todo()
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    /// `assert_eq!` と同じだが、2 つの引数が同じ型であることを要求する。
    /// （仮のマクロ定義のままでも、期待値の側から型が決まるようにするため）
    fn check<T: PartialEq + std::fmt::Debug>(actual: T, expected: T) {
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_square() {
        check(square!(3), 9);
        check(square!(-4), 16);
        check(square!(1 + 2), 9);
        check(square!(1.5_f64), 2.25);
    }

    #[test]
    fn test_square_evaluates_once() {
        let mut calls = 0;
        let mut next = || {
            calls += 1;
            calls
        };
        // next() が 1 回だけ呼ばれれば 1 * 1 = 1。2 回呼ばれると 1 * 2 = 2 になってしまう
        check(square!(next()), 1);
        assert_eq!(calls, 1);
    }

    #[test]
    fn test_max_of() {
        check(max_of!(3), 3);
        check(max_of!(3, 9), 9);
        check(max_of!(3, 9, 2, 7), 9);
        check(max_of!(-5, -1, -3), -1);
        check(max_of!("pear", "apple"), "pear");
    }

    #[test]
    fn test_hashmap() {
        let m: HashMap<&str, i32> = hashmap! {
            "apple" => 3,
            "banana" => 5,
        };
        assert_eq!(m.len(), 2);
        assert_eq!(m["apple"], 3);

        let no_trailing: HashMap<i32, char> = hashmap! { 1 => 'a', 2 => 'b' };
        assert_eq!(no_trailing[&2], 'b');

        let empty: HashMap<String, u8> = hashmap! {};
        assert!(empty.is_empty());
    }
}
