//! ## 課題 03: ホストの関数を import して呼ぶ
//!
//! WASM のモジュールは、**ホスト（JavaScript など、WASM を動かしている側）が渡してくれた関数しか呼べません**。
//! 画面への出力も、乱数も、時刻も、WASM の中だけでは手に入りません。
//! これが WASM のサンドボックスの正体です（CS基礎 15）。
//!
//! この課題では、ホストから2つの関数を受け取ります。
//!
//! - `host_log(ptr, len)`: メモリ上の文字列を、ホスト側で表示してもらう
//! - `host_random() -> f64`: 0 以上 1 未満の乱数をもらう
//!
//! ## ネイティブでテストするための工夫
//!
//! `host_log` は、WASM として動くときにしか存在しません（JavaScript が渡すため）。
//! ネイティブで `cargo test` するときには、代わりの実装（モック）に差し替えます。
//! `#[cfg(target_arch = "wasm32")]` で、ビルドする対象ごとに使うコードを切り替えています。
//!
//! ```text
//!              あなたが書く関数（fizzbuzz_log, roll_dice）
//!                         │ host::log / host::random を呼ぶ
//!          ┌──────────────┴──────────────┐
//!   wasm32 のとき                    ネイティブのとき
//!   host_log を import して呼ぶ       Vec に記録するモック（テストで中身を確かめる）
//!   （JavaScript の console.log）     乱数は決まった順番の値を返す
//! ```
//!
//! 書くもの: wasm32 側の `log` と `random`、`fizzbuzz_log`、`roll_dice`。

/// WASM として動くときの実装。ホストの関数を呼ぶ。
#[cfg(target_arch = "wasm32")]
pub mod host {
    // ホストから受け取る関数の宣言。wasm32-unknown-unknown では、
    // これらは "env" という名前のモジュールから import される（host/run.mjs を見よ）。
    unsafe extern "C" {
        fn host_log(ptr: *const u8, len: usize);
        fn host_random() -> f64;
    }

    /// msg をホストに表示してもらう。
    pub fn log(msg: &str) {
        todo!()
    }

    /// ホストから、0 以上 1 未満の乱数をもらう。
    pub fn random() -> f64 {
        todo!()
    }
}

/// ネイティブでテストするときの実装（モック）。これは書いてあるので、読むだけでよい。
#[cfg(not(target_arch = "wasm32"))]
pub mod host {
    use std::cell::RefCell;

    thread_local! {
        static LOGS: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
        static RANDOMS: RefCell<Vec<f64>> = const { RefCell::new(Vec::new()) };
    }

    /// 表示する代わりに、記録しておく。
    pub fn log(msg: &str) {
        LOGS.with(|l| l.borrow_mut().push(msg.to_string()));
    }

    /// あらかじめ `set_randoms` で決めておいた値を、先頭から順に返す。
    pub fn random() -> f64 {
        RANDOMS.with(|r| {
            let mut r = r.borrow_mut();
            assert!(
                !r.is_empty(),
                "モックの乱数が足りません。set_randoms で用意してください"
            );
            r.remove(0)
        })
    }

    /// テスト用: 記録されたログを取り出す。
    pub fn take_logs() -> Vec<String> {
        LOGS.with(|l| std::mem::take(&mut *l.borrow_mut()))
    }

    /// テスト用: random が返す値を決めておく。
    pub fn set_randoms(values: &[f64]) {
        RANDOMS.with(|r| *r.borrow_mut() = values.to_vec());
    }
}

/// 1 から n までの FizzBuzz を、1行ずつ `host::log` で出力する。
/// （Stage 01 の fizzbuzz を思い出そう。ただし今回は、結果を返すのではなくホストに出力してもらう）
#[unsafe(no_mangle)]
pub extern "C" fn fizzbuzz_log(n: u32) {
    // TODO: 仮の値です。自分の実装に置き換えてください（todo!() を使わない理由は lib.rs を参照）
}

/// サイコロ（1〜6）を count 回振って、出た目の合計を返す。
/// 乱数は `host::random()`（0 以上 1 未満）から作る。
///
/// 考えてみよう: WASM の中で乱数を作れないのはなぜか？ 乱数の「種」はどこから来るのか？
#[unsafe(no_mangle)]
pub extern "C" fn roll_dice(count: u32) -> u32 {
    // TODO: 仮の値です。自分の実装に置き換えてください（todo!() を使わない理由は lib.rs を参照）
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz_log() {
        host::take_logs();
        fizzbuzz_log(15);
        let logs = host::take_logs();
        assert_eq!(logs.len(), 15);
        assert_eq!(logs[0], "1");
        assert_eq!(logs[2], "Fizz");
        assert_eq!(logs[4], "Buzz");
        assert_eq!(logs[14], "FizzBuzz");
    }

    #[test]
    fn test_fizzbuzz_log_zero() {
        host::take_logs();
        fizzbuzz_log(0);
        assert!(host::take_logs().is_empty());
    }

    #[test]
    fn test_roll_dice() {
        // 0.0 → 1, 0.5 → 4, 0.999 → 6 になるはず
        host::set_randoms(&[0.0, 0.5, 0.999]);
        assert_eq!(roll_dice(3), 1 + 4 + 6);
    }

    #[test]
    fn test_roll_dice_range() {
        // 1/6 刻みの境目でも、1〜6 の範囲に収まること
        let values: Vec<f64> = (0..6).map(|i| i as f64 / 6.0).collect();
        host::set_randoms(&values);
        assert_eq!(roll_dice(6), 1 + 2 + 3 + 4 + 5 + 6);
    }
}
