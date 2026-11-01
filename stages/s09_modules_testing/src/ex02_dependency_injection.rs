//! ## 課題 02: テストしやすい設計（依存の注入）
//!
//! 「現在時刻」「ファイル」「データベース」「ネットワーク」に直接触る関数は、テストが難しくなります。
//! 実行するたびに結果が変わったり、準備に手間がかかったりするからです。
//!
//! そこで、外の世界に触る部分を**トレイトで抽象化して、引数で受け取る**ようにします（依存の注入 / dependency injection）。
//! - 本番では、本物の実装（`SystemClock`）を渡す
//! - テストでは、決まった値を返す偽物（フェイク）を渡す

use std::collections::HashMap;

// ---------------------------------------------------------------
// Part 1: 時計
// ---------------------------------------------------------------

/// 現在の「時」(0〜23) を返すもの
pub trait Clock {
    fn hour(&self) -> u32;
}

/// 本物の時計（UTC）。実装済み。
pub struct SystemClock;

impl Clock for SystemClock {
    fn hour(&self) -> u32 {
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        ((secs / 3600) % 24) as u32
    }
}

/// 時間帯に応じた挨拶を返す。
/// - 5時〜11時: "おはよう"
/// - 12時〜17時: "こんにちは"
/// - それ以外: "こんばんは"
///
/// 悪い例: この関数の中で `SystemClock.hour()` を直接呼ぶと、
/// 「朝のときの挙動」を夜にテストすることができなくなる。
pub fn greeting(clock: &impl Clock) -> &'static str {
    todo!()
}

// ---------------------------------------------------------------
// Part 2: 保存先
// ---------------------------------------------------------------

/// キーと値を保存する場所。本番ならファイルや DB、テストならメモリ。
pub trait Store {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: &str, value: &str);
}

/// メモリ上の Store
#[derive(Debug, Default)]
pub struct MemoryStore {
    data: HashMap<String, String>,
}

impl Store for MemoryStore {
    fn get(&self, key: &str) -> Option<String> {
        todo!()
    }

    fn set(&mut self, key: &str, value: &str) {
        todo!()
    }
}

/// ユーザーの訪問回数を1増やし、増やした後の回数を返す。
/// 回数はキー `"visits:<user>"` に文字列で保存する。
/// まだ保存されていない、または数値として読めない場合は 0 回とみなす。
///
/// この関数は `Store` トレイトにしか依存していないので、
/// 保存先がファイルでも DB でも、同じコードのまま使える。
pub fn record_visit(store: &mut impl Store, user: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// テスト用の偽物の時計: 常に決まった時刻を返す
    struct FixedClock(u32);

    impl Clock for FixedClock {
        fn hour(&self) -> u32 {
            self.0
        }
    }

    #[test]
    fn test_greeting() {
        assert_eq!(greeting(&FixedClock(4)), "こんばんは");
        assert_eq!(greeting(&FixedClock(5)), "おはよう");
        assert_eq!(greeting(&FixedClock(11)), "おはよう");
        assert_eq!(greeting(&FixedClock(12)), "こんにちは");
        assert_eq!(greeting(&FixedClock(17)), "こんにちは");
        assert_eq!(greeting(&FixedClock(18)), "こんばんは");
        assert_eq!(greeting(&FixedClock(0)), "こんばんは");
    }

    #[test]
    fn test_greeting_with_system_clock_does_not_panic() {
        // 本物の時計でも動くこと「だけ」を確認する（結果は時刻次第なので比べない）
        let g = greeting(&SystemClock);
        assert!(["おはよう", "こんにちは", "こんばんは"].contains(&g));
    }

    #[test]
    fn test_memory_store() {
        let mut s = MemoryStore::default();
        assert_eq!(s.get("a"), None);
        s.set("a", "1");
        s.set("a", "2");
        assert_eq!(s.get("a"), Some("2".to_string()));
    }

    #[test]
    fn test_record_visit() {
        let mut s = MemoryStore::default();
        assert_eq!(record_visit(&mut s, "kazu"), 1);
        assert_eq!(record_visit(&mut s, "kazu"), 2);
        assert_eq!(record_visit(&mut s, "alice"), 1);
        assert_eq!(s.get("visits:kazu"), Some("2".to_string()));
    }

    #[test]
    fn test_record_visit_broken_value() {
        let mut s = MemoryStore::default();
        s.set("visits:bob", "not a number");
        assert_eq!(record_visit(&mut s, "bob"), 1);
    }
}
