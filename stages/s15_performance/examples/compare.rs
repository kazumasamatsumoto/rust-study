//! 実験: 遅い実装と、あなたの実装を比べる
//!
//! cargo run -p s15_performance --example compare --release
//!
//! 課題を全部解いてから実行してください（まだの関数があると todo!() で止まります）。

use s15_performance::alloc_counter::{CountingAlloc, count_allocs};
use s15_performance::{ex01_allocations as e1, ex02_complexity as e2, ex03_hashing as e3};
use std::collections::HashMap;
use std::hint::black_box;
use std::time::{Duration, Instant};

// この実験プログラムでも、アロケーションを数えるアロケータを使う
#[global_allocator]
static ALLOC: CountingAlloc = CountingAlloc;

/// 3 回実行して、一番速い時間と、そのときのアロケーション回数を返す
fn measure<R>(mut f: impl FnMut() -> R) -> (Duration, usize) {
    let mut best = Duration::MAX;
    let mut allocs = 0;
    for _ in 0..3 {
        let start = Instant::now();
        let (r, n) = count_allocs(&mut f);
        let t = start.elapsed();
        black_box(r);
        if t < best {
            best = t;
            allocs = n;
        }
    }
    (best, allocs)
}

fn row(name: &str, slow: (Duration, usize), fast: (Duration, usize)) {
    println!(
        "{name:<20} 遅い版 {:>11.3?} ({:>7} allocs)   あなたの版 {:>11.3?} ({:>5} allocs)   {:>8.1} 倍",
        slow.0,
        slow.1,
        fast.0,
        fast.1,
        slow.0.as_secs_f64() / fast.0.as_secs_f64()
    );
}

fn main() {
    if cfg!(debug_assertions) {
        println!("※ debug ビルドです。--release を付けて実行してください。\n");
    }

    let words: Vec<&str> = (0..5_000).map(|_| "word").collect();
    row(
        "join",
        measure(|| e1::join_slow(&words, " ")),
        measure(|| e1::join(&words, " ")),
    );

    let clean = "the quick brown fox jumps over the lazy dog ".repeat(1000);
    let clean = clean.trim_end();
    row(
        "normalize (綺麗)",
        measure(|| e1::normalize_spaces_slow(clean)),
        measure(|| e1::normalize_spaces(clean)),
    );

    let lines: Vec<String> = (0..100_000)
        .map(|i| format!("{i}, name{i}, {}", i * 2))
        .collect();
    row(
        "fields (10万行)",
        measure(|| {
            let mut total = 0;
            for l in &lines {
                total += e1::fields_slow(l).len();
            }
            total
        }),
        measure(|| {
            let mut out = Vec::new();
            let mut total = 0;
            for l in &lines {
                e1::fields_into(l, &mut out);
                total += out.len();
            }
            total
        }),
    );

    let v: Vec<u64> = (0..20_000).collect();
    row(
        "has_duplicates (2万)",
        measure(|| e2::has_duplicates_slow(&v)),
        measure(|| e2::has_duplicates(&v)),
    );

    let a: Vec<u64> = (0..20_000).collect();
    let b: Vec<u64> = (10_000..30_000).collect();
    row(
        "common (2万×2万)",
        measure(|| e2::common_slow(&a, &b)),
        measure(|| e2::common(&a, &b)),
    );

    let ws = ["alpha", "beta", "gamma", "delta", "epsilon"];
    let text = (0..200_000)
        .map(|i| ws[i % ws.len()])
        .collect::<Vec<_>>()
        .join(" ");
    row(
        "most_frequent",
        measure(|| e2::most_frequent_slow(&text)),
        measure(|| e2::most_frequent(&text)),
    );

    let series: Vec<f64> = (0..100_000).map(|x| (x % 100) as f64).collect();
    row(
        "moving_average (w=1000)",
        measure(|| e3::moving_average_slow(&series, 1000)),
        measure(|| e3::moving_average(&series, 1000)),
    );

    let keys: Vec<u64> = (0..1_000_000u64)
        .map(|i| i.wrapping_mul(2_654_435_761) % 100_000)
        .collect();
    row(
        "count (標準 vs FNV)",
        measure(|| {
            let mut m: HashMap<u64, usize> = HashMap::new();
            for k in &keys {
                *m.entry(*k).or_insert(0) += 1;
            }
            m.len()
        }),
        measure(|| e3::count_with_fnv(&keys).len()),
    );
}
