//! ## 課題 03: 共有状態（Arc / Mutex / アトミック）
//!
//! - `Arc<T>`: 複数のスレッドで所有を共有するためのポインタ（Atomically Reference Counted）
//! - `Mutex<T>`: 中のデータを「鍵」で守る。`lock()` で鍵を取り、ガードが drop されると返す
//! - `AtomicU64` など: 単純な数値なら、鍵なしで安全に更新できる
//!
//! この課題の型は `&self` のメソッドしか持たないのに、中身を書き換えます（内部可変性）。
//! 複数のスレッドから `&` で共有されたまま使えるようにするためです。

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

// ---------------------------------------------------------------
// Part 1: アクセスカウンタ
// ---------------------------------------------------------------

/// ページごとのアクセス数と、全体のアクセス数を数える。
#[derive(Debug, Default)]
pub struct HitCounter {
    pages: Mutex<HashMap<String, u64>>,
    total: AtomicU64,
}

impl HitCounter {
    pub fn new() -> Self {
        Self::default()
    }

    /// ページのアクセス数と全体のアクセス数を、それぞれ1増やす
    pub fn hit(&self, page: &str) {
        todo!()
    }

    /// ページのアクセス数（アクセスがなければ 0）
    pub fn get(&self, page: &str) -> u64 {
        todo!()
    }

    pub fn total(&self) -> u64 {
        todo!()
    }
}

// ---------------------------------------------------------------
// Part 2: 銀行（デッドロックを避ける）
// ---------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum TransferError {
    /// 送金元と送金先が同じ
    SameAccount,
    /// 口座番号が存在しない
    NoSuchAccount,
    /// 残高不足
    InsufficientFunds,
}

/// 口座ごとに Mutex を持つ銀行。
///
/// 口座 A→B と B→A の送金が同時に起きると、素朴に書くと**デッドロック**する。
/// （`docs/fundamentals/08_concurrency.md` の「デッドロック」を参照）
///
/// ヒント: 2つの鍵を取るときは、**常に口座番号の小さい方から**取る。
#[derive(Debug)]
pub struct Bank {
    accounts: Vec<Mutex<u64>>,
}

impl Bank {
    /// 各口座の初期残高を受け取って銀行を作る
    pub fn new(balances: Vec<u64>) -> Self {
        todo!()
    }

    pub fn balance(&self, id: usize) -> Option<u64> {
        todo!()
    }

    /// 全口座の残高の合計
    pub fn total(&self) -> u64 {
        todo!()
    }

    /// `from` から `to` へ `amount` を送金する。
    /// エラーのときは、どの口座の残高も変えないこと。
    pub fn transfer(&self, from: usize, to: usize, amount: u64) -> Result<(), TransferError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    /// T がスレッド間で送れて (Send)、共有できる (Sync) ことをコンパイル時に確かめる。
    /// 実行時には何もしない。条件を満たさなければ、コンパイルエラーになる。
    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn types_are_thread_safe() {
        assert_send_sync::<HitCounter>();
        assert_send_sync::<Bank>();
    }

    #[test]
    fn test_hit_counter_single_thread() {
        let c = HitCounter::new();
        c.hit("/");
        c.hit("/about");
        c.hit("/");
        assert_eq!(c.get("/"), 2);
        assert_eq!(c.get("/about"), 1);
        assert_eq!(c.get("/none"), 0);
        assert_eq!(c.total(), 3);
    }

    #[test]
    fn test_hit_counter_many_threads() {
        let c = Arc::new(HitCounter::new());
        let handles: Vec<_> = (0..8)
            .map(|i| {
                let c = Arc::clone(&c);
                thread::spawn(move || {
                    for _ in 0..1000 {
                        c.hit(if i % 2 == 0 { "/even" } else { "/odd" });
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(c.get("/even"), 4000);
        assert_eq!(c.get("/odd"), 4000);
        assert_eq!(c.total(), 8000);
    }

    #[test]
    fn test_transfer_basic() {
        let bank = Bank::new(vec![100, 50]);
        assert_eq!(bank.transfer(0, 1, 30), Ok(()));
        assert_eq!(bank.balance(0), Some(70));
        assert_eq!(bank.balance(1), Some(80));
        assert_eq!(
            bank.transfer(1, 0, 81),
            Err(TransferError::InsufficientFunds)
        );
        assert_eq!(bank.transfer(0, 0, 1), Err(TransferError::SameAccount));
        assert_eq!(bank.transfer(0, 5, 1), Err(TransferError::NoSuchAccount));
        assert_eq!(bank.balance(5), None);
        assert_eq!(bank.total(), 150);
    }

    #[test]
    fn test_transfer_no_deadlock_and_money_is_conserved() {
        // 口座 0 と 1 の間で、逆方向の送金を同時に大量に行う。
        // デッドロックすると、このテストは終わらなくなる。
        let bank = Bank::new(vec![1_000, 1_000, 1_000]);
        thread::scope(|s| {
            for t in 0..6 {
                let bank = &bank;
                s.spawn(move || {
                    for i in 0..2_000 {
                        let (from, to) = if t % 2 == 0 { (0, 1) } else { (1, 0) };
                        let to = if i % 3 == 0 { 2 } else { to };
                        // 残高不足のエラーは起きてよい。お金が消えたり増えたりしないことが大事。
                        let _ = bank.transfer(from, to, 7);
                        let _ = bank.transfer(2, from, 3);
                    }
                });
            }
        });
        assert_eq!(bank.total(), 3_000);
    }
}
