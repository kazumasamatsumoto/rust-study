//! ## 課題 03: 状態機械
//!
//! enum で「状態」を表し、match で遷移を書くのは Rust の定番パターンです。
//! ありえない状態を型で表現できなくする（make illegal states unrepresentable）
//! という考え方を体感しましょう。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Light {
    Green,
    Yellow,
    Red,
}

impl Light {
    /// 次の状態: Green -> Yellow -> Red -> Green
    pub fn next(self) -> Light {
        todo!()
    }

    /// 各状態の秒数: Green 30, Yellow 5, Red 25
    pub fn duration_secs(self) -> u32 {
        todo!()
    }
}

/// 自動販売機
#[derive(Debug, PartialEq)]
pub enum Vending {
    /// 投入金額を待っている
    Idle,
    /// お金が入っている
    HasMoney(u32),
    /// 商品を出した（お釣り）
    Dispensed { change: u32 },
}

pub const PRICE: u32 = 150;

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Insert(u32),
    PushButton,
    Cancel,
}

impl Vending {
    /// イベントを受けて次の状態を返す。
    /// - Idle + Insert(n)          -> HasMoney(n)
    /// - HasMoney(m) + Insert(n)   -> HasMoney(m + n)
    /// - HasMoney(m) + PushButton  -> m >= PRICE なら Dispensed { change: m - PRICE }、
    ///   足りなければ HasMoney(m) のまま
    /// - HasMoney(_) + Cancel      -> Idle
    /// - Dispensed + 何か          -> Idle として扱ってからイベントを処理する
    /// - それ以外                  -> 状態はそのまま
    pub fn handle(self, event: Event) -> Vending {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_cycle() {
        let mut l = Light::Green;
        let mut total = 0;
        for _ in 0..3 {
            total += l.duration_secs();
            l = l.next();
        }
        assert_eq!(l, Light::Green);
        assert_eq!(total, 60);
    }

    #[test]
    fn test_vending_buy() {
        let s = Vending::Idle
            .handle(Event::Insert(100))
            .handle(Event::PushButton)
            .handle(Event::Insert(100));
        assert_eq!(s, Vending::HasMoney(200));
        assert_eq!(
            s.handle(Event::PushButton),
            Vending::Dispensed { change: 50 }
        );
    }

    #[test]
    fn test_vending_cancel_and_restart() {
        assert_eq!(Vending::HasMoney(10).handle(Event::Cancel), Vending::Idle);
        assert_eq!(Vending::Idle.handle(Event::PushButton), Vending::Idle);
        assert_eq!(
            Vending::Dispensed { change: 0 }.handle(Event::Insert(150)),
            Vending::HasMoney(150)
        );
    }
}
