//! ## 課題 02: 制御フロー
//!
//! - `if` は式であり、値を返せる
//! - `loop` / `while` / `for`
//! - `match` の基本
//!
//! ヒント: `format!` マクロや `n.to_string()` で数値を文字列にできます。

/// FizzBuzz。3の倍数なら "Fizz"、5の倍数なら "Buzz"、両方なら "FizzBuzz"、
/// それ以外は数字をそのまま文字列にして返す。
pub fn fizzbuzz(n: u32) -> String {
    todo!()
}

/// コラッツ数列が 1 に到達するまでのステップ数を返す。
/// - n が偶数なら n / 2
/// - n が奇数なら 3n + 1
///
/// n == 1 なら 0 を返す。n は 1 以上と仮定してよい。
pub fn collatz_steps(n: u64) -> u32 {
    todo!()
}

/// n が素数かどうかを返す。
pub fn is_prime(n: u32) -> bool {
    todo!()
}

/// 点数から成績を返す。
/// 90-100: 'A', 80-89: 'B', 70-79: 'C', 60-69: 'D', それ以下: 'F'
/// 100 を超える値は 'A' 扱いではなく '?' を返す。
///
/// ヒント: `match` で範囲パターン `90..=100` が使えます。
pub fn grade(score: u32) -> char {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(10), "Buzz");
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(98), "98");
    }

    #[test]
    fn test_collatz_steps() {
        assert_eq!(collatz_steps(1), 0);
        assert_eq!(collatz_steps(2), 1);
        assert_eq!(collatz_steps(6), 8);
        assert_eq!(collatz_steps(27), 111);
    }

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(97));
        assert!(!is_prime(91));
        assert!(is_prime(7919));
    }

    #[test]
    fn test_grade() {
        assert_eq!(grade(100), 'A');
        assert_eq!(grade(90), 'A');
        assert_eq!(grade(85), 'B');
        assert_eq!(grade(70), 'C');
        assert_eq!(grade(65), 'D');
        assert_eq!(grade(0), 'F');
        assert_eq!(grade(101), '?');
    }
}
