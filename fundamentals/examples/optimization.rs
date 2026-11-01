//! 実験: debug ビルドと release ビルドの違い
//! cargo run -p fundamentals --example optimization
//! cargo run -p fundamentals --example optimization --release

use std::hint::black_box;
use std::time::Instant;

/// 素朴な再帰のフィボナッチ（関数呼び出しが大量に起きる）
fn fib(n: u32) -> u64 {
    if n < 2 {
        n as u64
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

/// イテレータを何段も重ねた処理（抽象化のコストを見る）
fn iterator_chain(n: u64) -> u64 {
    (0..n)
        .map(|x| x * 3)
        .filter(|x| x % 2 == 0)
        .map(|x| x / 2)
        .sum()
}

/// 同じ処理を for ループで手書きしたもの
fn hand_loop(n: u64) -> u64 {
    let mut sum = 0;
    let mut x = 0;
    while x < n {
        let y = x * 3;
        if y % 2 == 0 {
            sum += y / 2;
        }
        x += 1;
    }
    sum
}

fn measure<T: std::fmt::Debug>(label: &str, f: impl FnOnce() -> T) {
    let start = Instant::now();
    let result = f();
    println!("{label:<28}: {:>12?}  (結果 {result:?})", start.elapsed());
}

fn main() {
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    println!("== {profile} ビルド ==");

    measure("fib(32)", || fib(black_box(32)));
    measure("iterator_chain(100_000_000)", || {
        iterator_chain(black_box(100_000_000))
    });
    measure("hand_loop(100_000_000)", || {
        hand_loop(black_box(100_000_000))
    });
}
