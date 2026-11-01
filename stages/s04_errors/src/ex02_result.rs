//! ## 課題 02: Result と ? 演算子
//!
//! `?` は「Ok なら中身を取り出し、Err なら即座に return する」演算子です。

use std::num::ParseIntError;

/// カンマ区切りの整数を合計する。1つでも解析に失敗したらそのエラーを返す。
/// 各要素の前後の空白は無視する。
/// 例: "1, 2,3" -> Ok(6)
pub fn sum_csv(input: &str) -> Result<i64, ParseIntError> {
    todo!()
}

/// a / b を計算する。b が 0 なら Err("division by zero")。
pub fn checked_div(a: i32, b: i32) -> Result<i32, String> {
    todo!()
}

/// "a/b" 形式の文字列を解析して割り算する。
/// 数値の解析失敗も、ゼロ除算も、形式不正も全部 Err(String) にする。
///
/// ヒント: `map_err` でエラーの型を変換できます。
pub fn eval_division(expr: &str) -> Result<i32, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_csv() {
        assert_eq!(sum_csv("1, 2,3"), Ok(6));
        assert_eq!(sum_csv("-10"), Ok(-10));
        assert!(sum_csv("1,x,3").is_err());
    }

    #[test]
    fn test_checked_div() {
        assert_eq!(checked_div(7, 2), Ok(3));
        assert_eq!(checked_div(1, 0), Err("division by zero".to_string()));
    }

    #[test]
    fn test_eval_division() {
        assert_eq!(eval_division("10/3"), Ok(3));
        assert_eq!(eval_division(" 8 / -2 "), Ok(-4));
        assert!(eval_division("1/0").is_err());
        assert!(eval_division("a/1").is_err());
        assert!(eval_division("10").is_err());
    }
}
