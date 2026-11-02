//! ## 課題 01: 数値だけをやり取りする
//!
//! WASM の関数がやり取りできる値の型は、`i32` / `i64` / `f32` / `f64` の4つだけです。
//! Rust の `u32` や `bool` は、境界を越えるときにこの4つのどれかに変換されます。
//!
//! 関数を外（JavaScript）から呼べるようにするには、2つの指定が必要です。
//!
//! - `#[unsafe(no_mangle)]`: 関数名を、コンパイラが勝手に書き換えないようにする
//!   （Rust は普段、関数名に型やモジュールの情報を混ぜた名前を付けている）
//! - `extern "C"`: 引数と戻り値の受け渡し方（ABI）を、C 言語の決まりに合わせる
//!
//! 中身はふつうの Rust です。Stage 01 の復習だと思って書いてください。

/// a + b を返す。オーバーフローしたら、ラップアラウンドした値を返す（panic させない）。
///
/// 考えてみよう: WASM の中で panic すると、JavaScript からはどう見えるか？（README の §1 実験 1）
#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    // TODO: 仮の値です。自分の実装に置き換えてください（todo!() を使わない理由は lib.rs を参照）
    0
}

/// n 番目のフィボナッチ数を返す（fib(0) = 0, fib(1) = 1）。n は 93 以下と仮定してよい。
///
/// 戻り値が `u64` であることに注目。JavaScript の数値（f64）では 2^53 より大きい整数を
/// 正確に表せないので、JavaScript 側では `BigInt` として受け取ることになる（CS基礎 02）。
#[unsafe(no_mangle)]
pub extern "C" fn fib(n: u32) -> u64 {
    // TODO: 仮の値です。自分の実装に置き換えてください（todo!() を使わない理由は lib.rs を参照）
    0
}

/// n が素数かどうか。
///
/// WASM には bool 型がない。この `bool` は、JavaScript からは何に見えるか？
#[unsafe(no_mangle)]
pub extern "C" fn is_prime(n: u32) -> bool {
    // TODO: 仮の値です。自分の実装に置き換えてください（todo!() を使わない理由は lib.rs を参照）
    false
}

/// 摂氏を華氏に変換する。
#[unsafe(no_mangle)]
pub extern "C" fn celsius_to_fahrenheit(c: f64) -> f64 {
    // TODO: 仮の値です。自分の実装に置き換えてください（todo!() を使わない理由は lib.rs を参照）
    0.0
}

/// **わざと panic する関数**（実験用。書き換える必要はありません）。
///
/// `host/run.mjs` から呼ぶと、JavaScript からは `RuntimeError: unreachable` に見えます。
/// WASM の中での panic は、その場で実行を止める「トラップ」になるからです（README §1）。
/// ネイティブのテストからは呼びません（lib.rs の注意を参照）。
#[unsafe(no_mangle)]
pub extern "C" fn panic_demo() -> i32 {
    panic!("WASM の中で panic しました");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-10, 4), -6);
        assert_eq!(add(i32::MAX, 1), i32::MIN);
    }

    #[test]
    fn test_fib() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(10), 55);
        assert_eq!(fib(90), 2_880_067_194_370_816_120);
    }

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(7919));
        assert!(!is_prime(7917));
    }

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
    }
}
