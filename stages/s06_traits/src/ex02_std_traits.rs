//! ## 課題 02: 標準ライブラリのトレイトを実装する
//!
//! 標準のトレイトを実装すると、その型が言語の機能や標準ライブラリと「つながり」ます。
//! - `Display` → `println!("{}")`, `.to_string()` が使える
//! - `From`    → `.into()` が使え、`?` でのエラー変換にも使われる
//! - `Add`     → `+` 演算子が使える
//! - `Sum`     → `.sum()` が使える
//! - `Iterator` → `next` を1つ書くだけで、`map` / `filter` / `take` などが全て使える

use std::fmt;
use std::iter::Sum;
use std::ops::Add;

/// 日本円の金額。CS基礎 02 の教訓に従い、f64 ではなく整数で持つ。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Money(i64);

impl Money {
    pub fn yen(&self) -> i64 {
        self.0
    }
}

impl fmt::Display for Money {
    /// 3桁ごとにカンマを入れて表示する。
    /// 例: 1234567 -> "¥1,234,567", 0 -> "¥0", -500 -> "-¥500", -1234 -> "-¥1,234"
    ///
    /// 注意: i64::MIN の絶対値は i64 に収まらない（CS基礎 01）。
    /// `unsigned_abs()` を調べてみよう。
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl From<i64> for Money {
    fn from(yen: i64) -> Self {
        todo!()
    }
}

impl Add for Money {
    type Output = Money;

    fn add(self, rhs: Money) -> Money {
        todo!()
    }
}

impl Sum for Money {
    fn sum<I: Iterator<Item = Money>>(iter: I) -> Money {
        todo!()
    }
}

/// from から 1 まで数え下げるイテレータ。
/// 例: Countdown::new(3) -> 3, 2, 1
pub struct Countdown {
    current: u32,
}

impl Countdown {
    pub fn new(from: u32) -> Self {
        todo!()
    }
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        todo!()
    }
}

/// フィボナッチ数列 0, 1, 1, 2, 3, 5, 8, ... を**無限に**返すイテレータ。
pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        todo!()
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    /// 無限に続くので、常に Some を返す。
    /// （u64 で表せる範囲を超えると溢れるが、この課題では気にしなくてよい）
    fn next(&mut self) -> Option<u64> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_display() {
        assert_eq!(Money::from(1_234_567).to_string(), "¥1,234,567");
        assert_eq!(Money::from(999).to_string(), "¥999");
        assert_eq!(Money::from(1000).to_string(), "¥1,000");
        assert_eq!(Money::from(0).to_string(), "¥0");
        assert_eq!(Money::from(-500).to_string(), "-¥500");
        assert_eq!(Money::from(-1234).to_string(), "-¥1,234");
        assert_eq!(
            Money::from(i64::MIN).to_string(),
            "-¥9,223,372,036,854,775,808"
        );
    }

    #[test]
    fn test_money_ops() {
        let a: Money = 300.into();
        let b = Money::from(200);
        assert_eq!((a + b).yen(), 500);
        let total: Money = [100, 250, -50].into_iter().map(Money::from).sum();
        assert_eq!(total, Money::from(300));
        assert!(Money::from(10) < Money::from(20));
    }

    #[test]
    fn test_countdown() {
        assert_eq!(Countdown::new(3).collect::<Vec<_>>(), vec![3, 2, 1]);
        assert_eq!(Countdown::new(0).count(), 0);
        // next を書くだけで、標準のアダプタが全て使える
        assert_eq!(Countdown::new(5).map(|x| x * 10).sum::<u32>(), 150);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(
            Fibonacci::new().take(10).collect::<Vec<_>>(),
            vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
        );
        // 100 未満の偶数のフィボナッチ数の合計
        let s: u64 = Fibonacci::new()
            .take_while(|&x| x < 100)
            .filter(|x| x % 2 == 0)
            .sum();
        assert_eq!(s, 44);
    }
}
