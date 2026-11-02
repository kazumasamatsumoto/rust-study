//! ## 課題 01: newtype — 同じ「数」でも意味が違えば別の型にする
//!
//! `f64` や `u64` は、それが「メートル」なのか「フィート」なのか、
//! 「ユーザー ID」なのか「注文 ID」なのかを知りません。
//! 1 つのフィールドだけを持つ構造体（newtype）で包むと、コンパイラが区別してくれます。
//!
//! 単位を取り違えるとコンパイルエラーになることを、doc テストで確かめています。
//!
//! ```compile_fail
//! use s14_type_design::ex01_newtypes::{Feet, Meters};
//! // フィートとメートルは足せない
//! let total = Meters(1.0) + Feet(3.0);
//! ```
//!
//! ```
//! use s14_type_design::ex01_newtypes::{Feet, Meters};
//! // 変換してからなら足せる
//! let total = Meters(1.0) + Meters::from(Feet(3.0));
//! assert!((total.0 - 1.9144).abs() < 1e-9);
//! ```
//!
//! ID の取り違えもコンパイルエラーになります。
//!
//! ```compile_fail
//! use s14_type_design::ex01_newtypes::{orders_of, Order, OrderId, UserId};
//! let orders: Vec<Order> = vec![];
//! // UserId を渡すべきところに OrderId を渡している
//! let found = orders_of(&orders, OrderId(1));
//! ```
//!
//! `NonEmptyVec` の中身を、空にすることはできません。
//!
//! ```compile_fail
//! use s14_type_design::ex01_newtypes::NonEmptyVec;
//! // フィールドは非公開なので、new を通さずに作れない
//! let v: NonEmptyVec<i32> = NonEmptyVec { items: vec![] };
//! ```

use std::fmt;
use std::ops::Add;

// ---------------------------------------------------------------------------
// (1) 単位
// ---------------------------------------------------------------------------

/// 長さ（メートル）
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Meters(pub f64);

/// 長さ（フィート）
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Feet(pub f64);

/// 1 フィート = 0.3048 メートル（国際フィート。定義なので正確な値）
pub const METERS_PER_FOOT: f64 = 0.3048;

impl From<Feet> for Meters {
    fn from(feet: Feet) -> Self {
        todo!()
    }
}

impl From<Meters> for Feet {
    fn from(m: Meters) -> Self {
        todo!()
    }
}

/// メートル同士なら足せる。
impl Add for Meters {
    type Output = Meters;
    fn add(self, rhs: Meters) -> Meters {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// (2) ID
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderId(pub u64);

#[derive(Debug, Clone)]
pub struct Order {
    pub id: OrderId,
    pub user: UserId,
    pub total_yen: u64,
}

/// user の注文の ID を、元の順番のまま返す。
pub fn orders_of(orders: &[Order], user: UserId) -> Vec<OrderId> {
    todo!()
}

// ---------------------------------------------------------------------------
// (3) 検証済みの値: "Parse, don't validate"
// ---------------------------------------------------------------------------

/// 検証済みのメールアドレス。
///
/// `Email` 型の値が存在するなら、それは必ず検証を通っている。
/// そのため、`Email` を受け取る関数は、もう一度チェックする必要がない。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmailError {
    /// '@' がちょうど 1 つではない
    AtSign,
    /// '@' の前が空
    EmptyLocal,
    /// '@' の後ろが空、または '.' を含まない、または '.' で始まる・終わる
    InvalidDomain,
    /// 空白文字を含む
    Whitespace,
}

impl Email {
    /// 次の順番でチェックし、最初に見つかった問題を返す。
    /// 1. 空白文字（`char::is_whitespace`）を含む → Whitespace
    /// 2. '@' がちょうど 1 つではない → AtSign
    /// 3. '@' の前が空 → EmptyLocal
    /// 4. '@' の後ろが空 / '.' を含まない / '.' で始まるか終わる → InvalidDomain
    ///
    /// 通ったら、**ドメイン部分（'@' の後ろ）だけを小文字にして**保存する。
    /// （ドメインは大文字小文字を区別しないが、'@' の前は区別しうるため）
    pub fn parse(s: &str) -> Result<Email, EmailError> {
        todo!()
    }

    pub fn as_str(&self) -> &str {
        todo!()
    }

    /// '@' の後ろの部分。
    /// parse で検証済みなので、ここでは Option を返す必要がない。
    pub fn domain(&self) -> &str {
        todo!()
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// (4) 空でない Vec
// ---------------------------------------------------------------------------

/// 必ず 1 つ以上の要素を持つ Vec。
///
/// `Vec::first()` は `Option<&T>` を返すが、`NonEmptyVec::first()` は `&T` を返せる。
/// 「空かもしれない」というケースを、型の段階で消しているから。
#[derive(Debug, Clone, PartialEq)]
pub struct NonEmptyVec<T> {
    // 非公開にすることで、空の NonEmptyVec を外から作れないようにしている
    items: Vec<T>,
}

impl<T> NonEmptyVec<T> {
    /// 先頭の要素と、残りの要素から作る。この形なら絶対に空にならない。
    pub fn new(first: T, rest: Vec<T>) -> Self {
        todo!()
    }

    /// Vec から作る。空なら None。
    pub fn from_vec(v: Vec<T>) -> Option<Self> {
        todo!()
    }

    pub fn first(&self) -> &T {
        todo!()
    }

    pub fn last(&self) -> &T {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    /// 空になることはないので常に false。
    /// （clippy の `len_without_is_empty` に従って用意している）
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn push(&mut self, item: T) {
        todo!()
    }

    pub fn into_vec(self) -> Vec<T> {
        todo!()
    }
}

impl<T: Ord> NonEmptyVec<T> {
    /// 最大の要素。同じ最大値が複数あれば、どれを返してもよい。
    /// `Iterator::max` と違って Option にならない。
    pub fn max(&self) -> &T {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn test_units() {
        assert!(approx(Meters::from(Feet(10.0)).0, 3.048));
        assert!(approx(Feet::from(Meters(3.048)).0, 10.0));
        assert!(approx((Meters(1.5) + Meters(2.0)).0, 3.5));
    }

    #[test]
    fn test_orders_of() {
        let orders = vec![
            Order {
                id: OrderId(1),
                user: UserId(10),
                total_yen: 500,
            },
            Order {
                id: OrderId(2),
                user: UserId(20),
                total_yen: 800,
            },
            Order {
                id: OrderId(3),
                user: UserId(10),
                total_yen: 300,
            },
        ];
        assert_eq!(orders_of(&orders, UserId(10)), vec![OrderId(1), OrderId(3)]);
        assert!(orders_of(&orders, UserId(99)).is_empty());
    }

    #[test]
    fn test_email_ok() {
        let e = Email::parse("Kazu@Example.COM").unwrap();
        assert_eq!(e.as_str(), "Kazu@example.com");
        assert_eq!(e.domain(), "example.com");
        assert_eq!(e.to_string(), "Kazu@example.com");
    }

    #[test]
    fn test_email_errors() {
        assert_eq!(Email::parse("a b@example.com"), Err(EmailError::Whitespace));
        assert_eq!(Email::parse("example.com"), Err(EmailError::AtSign));
        assert_eq!(Email::parse("a@b@example.com"), Err(EmailError::AtSign));
        assert_eq!(Email::parse("@example.com"), Err(EmailError::EmptyLocal));
        assert_eq!(Email::parse("a@"), Err(EmailError::InvalidDomain));
        assert_eq!(Email::parse("a@localhost"), Err(EmailError::InvalidDomain));
        assert_eq!(Email::parse("a@.com"), Err(EmailError::InvalidDomain));
        assert_eq!(Email::parse("a@example."), Err(EmailError::InvalidDomain));
    }

    #[test]
    fn test_non_empty_vec() {
        let mut v = NonEmptyVec::new(3, vec![9, 1]);
        assert_eq!(*v.first(), 3);
        assert_eq!(*v.last(), 1);
        assert_eq!(*v.max(), 9);
        assert_eq!(v.len(), 3);
        assert!(!v.is_empty());
        v.push(20);
        assert_eq!(*v.last(), 20);
        assert_eq!(*v.max(), 20);
        assert_eq!(v.into_vec(), vec![3, 9, 1, 20]);
    }

    #[test]
    fn test_non_empty_from_vec() {
        assert!(NonEmptyVec::<i32>::from_vec(vec![]).is_none());
        let v = NonEmptyVec::from_vec(vec!["a"]).unwrap();
        assert_eq!(*v.first(), "a");
        assert_eq!(*v.last(), "a");
    }
}
