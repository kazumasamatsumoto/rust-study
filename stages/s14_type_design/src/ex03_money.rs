//! ## 課題 03: 通貨を型で区別するお金の型 — これまでの総まとめ
//!
//! - newtype: 金額を「最小単位の整数」で持つ（CS基礎 02: お金を f64 で持たない）
//! - PhantomData: 通貨を型パラメータで表す。`Money<Jpy>` と `Money<Usd>` は別の型
//! - sealed trait: `Currency` を、このクレートの外から実装できないようにする
//! - `#[non_exhaustive]`: エラーの種類を将来増やしても、利用者のコードを壊さない
//!
//! 円とドルは足せません。
//!
//! ```compile_fail
//! use s14_type_design::ex03_money::{Jpy, Money, Usd};
//! let total = Money::<Jpy>::from_minor(100) + Money::<Usd>::from_minor(100);
//! ```
//!
//! クレートの外で、勝手に新しい通貨を作ることはできません（sealed trait）。
//!
//! ```compile_fail
//! use s14_type_design::ex03_money::Currency;
//! struct Bitcoin;
//! impl Currency for Bitcoin {
//!     const CODE: &'static str = "BTC";
//!     const DECIMALS: u32 = 8;
//! }
//! ```
//!
//! `#[non_exhaustive]` が付いた enum を外のクレートで match するときは、
//! `_` の腕が必須になります。
//!
//! ```compile_fail
//! use s14_type_design::ex03_money::ParseMoneyError;
//! fn describe(e: ParseMoneyError) -> &'static str {
//!     match e {
//!         ParseMoneyError::Empty => "empty",
//!         ParseMoneyError::InvalidDigit => "digit",
//!         ParseMoneyError::TooManyDecimals { .. } => "decimals",
//!         ParseMoneyError::Overflow => "overflow",
//!         // `_ => ...` がないのでコンパイルエラー
//!     }
//! }
//! ```

use std::fmt;
use std::marker::PhantomData;
use std::ops::Add;

// ---------------------------------------------------------------------------
// sealed trait
// ---------------------------------------------------------------------------

mod sealed {
    /// 非公開モジュールの中にある、公開トレイト。
    /// 外のクレートからは名前を書けないので、実装することもできない。
    pub trait Sealed {}
}

/// 通貨。このクレートの中で定義した型だけが実装できる。
pub trait Currency: sealed::Sealed {
    /// ISO 4217 の通貨コード
    const CODE: &'static str;
    /// 小数点以下の桁数（円は 0、ドルは 2）
    const DECIMALS: u32;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Jpy;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Usd;

impl sealed::Sealed for Jpy {}
impl sealed::Sealed for Usd {}

impl Currency for Jpy {
    const CODE: &'static str = "JPY";
    const DECIMALS: u32 = 0;
}

impl Currency for Usd {
    const CODE: &'static str = "USD";
    const DECIMALS: u32 = 2;
}

// ---------------------------------------------------------------------------
// Money
// ---------------------------------------------------------------------------

/// 通貨 C の金額。最小単位（円なら 1 円、ドルなら 1 セント）の整数で持つ。
///
/// `PartialEq` などを derive ではなく手で実装しているのは、
/// derive だと「C も PartialEq を実装していること」が条件に付いてしまうため。
/// （Jpy と Usd は実装しているので、ここでは derive でも動くが、一般的な注意点として。）
pub struct Money<C: Currency> {
    minor: i64,
    _currency: PhantomData<C>,
}

impl<C: Currency> Clone for Money<C> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<C: Currency> Copy for Money<C> {}

impl<C: Currency> PartialEq for Money<C> {
    fn eq(&self, other: &Self) -> bool {
        self.minor == other.minor
    }
}

impl<C: Currency> fmt::Debug for Money<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Money<{}>({})", C::CODE, self.minor)
    }
}

/// 文字列から Money を作るときのエラー。
///
/// `#[non_exhaustive]` が付いているので、将来バリアントを追加しても、
/// 利用者のコードはコンパイルエラーにならない（利用者は必ず `_` の腕を書いているため）。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseMoneyError {
    /// 空文字列（前後の空白を除いた後）
    Empty,
    /// 数字・先頭の '-'・小数点 1 つ 以外の文字がある
    InvalidDigit,
    /// 小数点以下の桁数が、その通貨の DECIMALS を超えている
    TooManyDecimals { max: u32 },
    /// i64 に収まらない
    Overflow,
}

impl<C: Currency> Money<C> {
    /// 最小単位の値から作る。
    pub fn from_minor(minor: i64) -> Self {
        todo!()
    }

    pub fn minor(&self) -> i64 {
        todo!()
    }

    /// 文字列から作る。前後の空白は無視する。
    ///
    /// - "1000"（JPY）→ 1000 円
    /// - "12.34"（USD）→ 1234 セント
    /// - "12.3"（USD）→ 1230 セント（足りない桁は 0 で埋める）
    /// - "12"（USD）→ 1200 セント
    /// - "-0.5"（USD）→ -50 セント
    /// - "12.345"（USD）→ Err(TooManyDecimals { max: 2 })
    /// - "1.5"（JPY）→ Err(TooManyDecimals { max: 0 })
    /// - "12a" / "1.2.3" / "." / "-" → Err(InvalidDigit)
    ///
    /// ヒント: 浮動小数点数を経由しないこと（"0.1" を f64 にした時点で誤差が出る）。
    /// 整数部と小数部を分けて、整数として組み立てる。
    /// オーバーフローの検出には `checked_mul` / `checked_add` が使える。
    pub fn parse(s: &str) -> Result<Self, ParseMoneyError> {
        todo!()
    }

    /// 通貨 To に換算する。
    ///
    /// rate は「1 最小単位の C が、To の最小単位でいくつになるか」を分数 numer / denom で表す。
    /// 例: 1 円 = 0.65 セント なら Money::<Jpy> → Money::<Usd> の rate は (65, 100)。
    ///
    /// 計算は `self.minor * numer / denom`。割り切れない分は 0 の方向に切り捨てる（i64 の / と同じ）。
    /// 掛け算が i64 に収まらないときは None。denom が 0 のときも None。
    pub fn convert<To: Currency>(self, numer: i64, denom: i64) -> Option<Money<To>> {
        todo!()
    }
}

/// 同じ通貨同士なら足せる（溢れたら panic してよい。i64 の + と同じ扱い）。
impl<C: Currency> Add for Money<C> {
    type Output = Money<C>;
    fn add(self, rhs: Self) -> Self {
        todo!()
    }
}

/// 表示形式: "<金額> <通貨コード>"。小数点以下は DECIMALS 桁で表示する。
/// - Money::<Jpy>::from_minor(1500)  → "1500 JPY"
/// - Money::<Usd>::from_minor(1234)  → "12.34 USD"
/// - Money::<Usd>::from_minor(5)     → "0.05 USD"
/// - Money::<Usd>::from_minor(-1234) → "-12.34 USD"
/// - Money::<Usd>::from_minor(-5)    → "-0.05 USD"
impl<C: Currency> fmt::Display for Money<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// 合計。同じ通貨の金額のスライスを足す。空なら 0。
pub fn total<C: Currency>(items: &[Money<C>]) -> Money<C> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_total() {
        let a = Money::<Jpy>::from_minor(1000);
        let b = Money::<Jpy>::from_minor(250);
        assert_eq!((a + b).minor(), 1250);
        assert_eq!(total(&[a, b, b]).minor(), 1500);
        assert_eq!(total::<Usd>(&[]).minor(), 0);
    }

    #[test]
    fn test_display() {
        assert_eq!(Money::<Jpy>::from_minor(1500).to_string(), "1500 JPY");
        assert_eq!(Money::<Usd>::from_minor(1234).to_string(), "12.34 USD");
        assert_eq!(Money::<Usd>::from_minor(5).to_string(), "0.05 USD");
        assert_eq!(Money::<Usd>::from_minor(-1234).to_string(), "-12.34 USD");
        assert_eq!(Money::<Usd>::from_minor(-5).to_string(), "-0.05 USD");
        assert_eq!(Money::<Jpy>::from_minor(-3).to_string(), "-3 JPY");
    }

    #[test]
    fn test_parse_ok() {
        assert_eq!(Money::<Jpy>::parse("1000").unwrap().minor(), 1000);
        assert_eq!(Money::<Usd>::parse(" 12.34 ").unwrap().minor(), 1234);
        assert_eq!(Money::<Usd>::parse("12.3").unwrap().minor(), 1230);
        assert_eq!(Money::<Usd>::parse("12").unwrap().minor(), 1200);
        assert_eq!(Money::<Usd>::parse("-0.5").unwrap().minor(), -50);
        assert_eq!(Money::<Usd>::parse("0.10").unwrap().minor(), 10);
    }

    #[test]
    fn test_parse_err() {
        assert_eq!(Money::<Usd>::parse("  "), Err(ParseMoneyError::Empty));
        assert_eq!(
            Money::<Usd>::parse("12.345"),
            Err(ParseMoneyError::TooManyDecimals { max: 2 })
        );
        assert_eq!(
            Money::<Jpy>::parse("1.5"),
            Err(ParseMoneyError::TooManyDecimals { max: 0 })
        );
        for bad in ["12a", "1.2.3", ".", "-", "+5", "1,000"] {
            assert_eq!(
                Money::<Usd>::parse(bad),
                Err(ParseMoneyError::InvalidDigit),
                "input: {bad:?}"
            );
        }
        assert_eq!(
            Money::<Jpy>::parse("99999999999999999999"),
            Err(ParseMoneyError::Overflow)
        );
        assert_eq!(
            Money::<Usd>::parse("92233720368547758.08"),
            Err(ParseMoneyError::Overflow)
        );
    }

    #[test]
    fn test_parse_display_roundtrip() {
        for s in ["0.01", "-7.50", "123456.78"] {
            assert_eq!(
                Money::<Usd>::parse(s).unwrap().to_string(),
                format!("{s} USD")
            );
        }
    }

    #[test]
    fn test_convert() {
        let yen = Money::<Jpy>::from_minor(1000);
        let usd: Money<Usd> = yen.convert(65, 100).unwrap();
        assert_eq!(usd.to_string(), "6.50 USD");
        // 切り捨て
        let usd: Money<Usd> = Money::<Jpy>::from_minor(1).convert(65, 100).unwrap();
        assert_eq!(usd.minor(), 0);
        // オーバーフローと 0 除算
        assert!(
            Money::<Jpy>::from_minor(i64::MAX)
                .convert::<Usd>(2, 1)
                .is_none()
        );
        assert!(yen.convert::<Usd>(1, 0).is_none());
    }
}
