//! ## 課題 01: 構造体とメソッド
//!
//! - `impl` ブロックでメソッドを定義する
//! - `&self` / `&mut self` / `self` の違い
//! - `Self` を返す関連関数（コンストラクタ）
//! - `#[derive(...)]` で便利なトレイトを自動実装

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    /// コンストラクタ
    pub fn new(width: u32, height: u32) -> Self {
        todo!()
    }

    /// 正方形を作る関連関数
    pub fn square(size: u32) -> Self {
        todo!()
    }

    pub fn area(&self) -> u32 {
        todo!()
    }

    /// other が自分の中に（回転せずに）収まるか
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        todo!()
    }

    /// 自分自身を factor 倍に拡大する
    pub fn scale(&mut self, factor: u32) {
        todo!()
    }
}

/// 銀行口座。残高はマイナスにならない。
#[derive(Debug)]
pub struct Account {
    // フィールドは非公開。外からはメソッド経由でのみ触れる（カプセル化）。
    owner: String,
    balance: u64,
}

impl Account {
    pub fn new(owner: &str) -> Self {
        todo!()
    }

    pub fn owner(&self) -> &str {
        todo!()
    }

    pub fn balance(&self) -> u64 {
        todo!()
    }

    pub fn deposit(&mut self, amount: u64) {
        todo!()
    }

    /// 残高が足りれば引き出して true、足りなければ何もせず false。
    pub fn withdraw(&mut self, amount: u64) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle() {
        let r = Rectangle::new(4, 5);
        assert_eq!(r.area(), 20);
        assert_eq!(Rectangle::square(3), Rectangle::new(3, 3));
        assert!(r.can_hold(&Rectangle::new(3, 5)));
        assert!(!r.can_hold(&Rectangle::new(5, 3)));
    }

    #[test]
    fn test_rectangle_scale() {
        let mut r = Rectangle::new(2, 3);
        r.scale(3);
        assert_eq!(r, Rectangle::new(6, 9));
    }

    #[test]
    fn test_account() {
        let mut a = Account::new("kazu");
        assert_eq!(a.owner(), "kazu");
        assert_eq!(a.balance(), 0);
        a.deposit(1000);
        assert!(a.withdraw(300));
        assert_eq!(a.balance(), 700);
        assert!(!a.withdraw(701));
        assert_eq!(a.balance(), 700);
    }
}
