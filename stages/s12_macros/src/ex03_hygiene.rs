//! ## 課題 03: 衛生性・複数のパターン・使い分け
//!
//! - **衛生性 (hygiene)**: マクロの中で作った変数は、呼び出し側の変数と衝突しない
//! - パターンを複数書いて、引数の数によって振る舞いを変える
//! - `stringify!` で「書かれた式そのもの」をエラーメッセージに含める

/// 式を評価して、その値を返す。ついでに、かかった時間を標準エラーに出力する。
///
/// ```text
/// let r = time_it!("重い計算", heavy(10));
/// // 標準エラーに `[重い計算] 1.2ms` のように出力され、r には heavy(10) の結果が入る
/// ```
///
/// 実装の中では `let start = std::time::Instant::now();` のように
/// `start` という名前の変数を使ってかまいません。呼び出し側に同じ名前の変数があっても衝突しないことを、
/// テストで確認しています（これが衛生性です）。
#[macro_export]
macro_rules! time_it {
    // ↓ 仮の定義。消して書き直す
    ($($t:tt)*) => {
        $crate::__todo()
    };
}

/// 2 つの f64 がほぼ等しいことを確認する。等しくなければ panic する。
///
/// - `assert_approx!(a, b)`      … 許容誤差 1e-9
/// - `assert_approx!(a, b, eps)` … 許容誤差を指定
///
/// panic のメッセージは次の形式にする（テストで確認しています）:
///
/// ```text
/// assert_approx failed: `0.1 + 0.2` = 0.30000000000000004, `0.4` = 0.4 (eps = 0.000000001)
/// ```
///
/// - バッククォートの中は、**呼び出し側に書かれた式そのもの**（`stringify!` を使う）
/// - eps は `{:.9}` で表示する
///
/// ヒント: 2 つ目のパターンから、3 つ目のパターンを呼び出すと重複が減ります。
#[macro_export]
macro_rules! assert_approx {
    // ↓ 仮の定義。消して書き直す
    ($($t:tt)*) => {
        $crate::__todo::<()>()
    };
}

/// 渡されたトークン木の数を数える。
///
/// 例: `count!()` → `0`、`count!(a b c)` → `3`、`count!(x (y z) w)` → `3`
/// （`(y z)` はかっこで囲まれているので 1 つのトークン木）
///
/// 戻り値は `usize`。
///
/// ヒント: `($head:tt $($tail:tt)*) => { 1usize + count!($($tail)*) }` という再帰で書けます。
/// ただし再帰には上限（デフォルト 128 段）があることに注意（README の設計の問いを参照）。
#[macro_export]
macro_rules! count {
    // ↓ 仮の定義。消して書き直す
    ($($t:tt)*) => {
        $crate::__todo::<usize>()
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_time_it_returns_value() {
        let r: i32 = time_it!("sum", (1..=10).sum::<i32>());
        assert_eq!(r, 55);
    }

    #[test]
    fn test_time_it_is_hygienic() {
        // 呼び出し側にも start という変数がある
        let start = 100;
        let r: i32 = time_it!("add", start + 1);
        assert_eq!(r, 101);
        assert_eq!(start, 100);
    }

    #[test]
    fn test_assert_approx_ok() {
        assert_approx!(0.1 + 0.2, 0.3);
        assert_approx!(1.0, 1.05, 0.1);
    }

    #[test]
    #[should_panic(
        expected = "assert_approx failed: `0.1 + 0.2` = 0.30000000000000004, `0.4` = 0.4 (eps = 0.000000001)"
    )]
    fn test_assert_approx_fails_with_message() {
        assert_approx!(0.1 + 0.2, 0.4);
    }

    #[test]
    #[should_panic(expected = "(eps = 0.010000000)")]
    fn test_assert_approx_custom_eps() {
        assert_approx!(1.0, 1.05, 0.01);
    }

    #[test]
    fn test_count() {
        assert_eq!(count!(), 0);
        assert_eq!(count!(a), 1);
        assert_eq!(count!(a b c), 3);
        assert_eq!(count!(x (y z) w), 3);
    }
}
