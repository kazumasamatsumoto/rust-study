//! ## 課題 01: 変数と型
//!
//! - `let` と `let mut` の違い
//! - 整数型 (`i32`, `u64` ...) と浮動小数点型 (`f64`)
//! - `as` による型変換
//! - タプル
//!
//! ヒント: Rust では関数の最後の式（セミコロンなし）が戻り値になります。

/// 摂氏を華氏に変換する。 F = C × 9/5 + 32
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    todo!()
}

/// 2つの値を入れ替えたタプルを返す。
pub fn swap(a: i32, b: i32) -> (i32, i32) {
    todo!()
}

/// 1 から n までの合計を返す（ループを使ってみよう）。
/// n が大きくても溢れないよう、戻り値は u64。
pub fn sum_to(n: u32) -> u64 {
    todo!()
}

/// 秒数を (時, 分, 秒) に分解する。
/// 例: 3725 -> (1, 2, 5)
pub fn split_seconds(total: u32) -> (u32, u32, u32) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
    }

    #[test]
    fn test_swap() {
        assert_eq!(swap(1, 2), (2, 1));
        assert_eq!(swap(-5, 5), (5, -5));
    }

    #[test]
    fn test_sum_to() {
        assert_eq!(sum_to(0), 0);
        assert_eq!(sum_to(10), 55);
        assert_eq!(sum_to(100_000), 5_000_050_000);
    }

    #[test]
    fn test_split_seconds() {
        assert_eq!(split_seconds(0), (0, 0, 0));
        assert_eq!(split_seconds(59), (0, 0, 59));
        assert_eq!(split_seconds(3725), (1, 2, 5));
    }
}
