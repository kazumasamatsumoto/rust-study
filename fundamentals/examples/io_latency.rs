//! 実験: I/O はどれくらい遅いのか、スレッドはどれくらい重いのか
//! cargo run -p fundamentals --example io_latency --release

use std::hint::black_box;
use std::io::{Read, Write};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    println!("== 1回あたりの時間を比べる ==");

    // メモリ上の配列を読む
    let data: Vec<u64> = (0..1_000_000).collect();
    let start = Instant::now();
    let mut sum = 0u64;
    for x in &data {
        sum = sum.wrapping_add(*x);
    }
    black_box(sum);
    // Duration の整数割り算だと 1ns 未満が切り捨てられるので、f64 で計算する
    let per_ns = start.elapsed().as_secs_f64() * 1e9 / data.len() as f64;
    println!("メモリから u64 を1個読む      : {per_ns:.3}ns (連続した読み込み)");

    // ファイルに書いて、読む (OS のキャッシュに載るので、実際のディスクより速い)
    let path = std::env::temp_dir().join("rust_study_io_latency.txt");
    let n = 1000;
    let start = Instant::now();
    for i in 0..n {
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, "hello {i}").unwrap();
    }
    println!("ファイルを開いて書く (1回)    : {:?}", start.elapsed() / n);

    let start = Instant::now();
    let mut buf = String::new();
    for _ in 0..n {
        buf.clear();
        std::fs::File::open(&path)
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();
    }
    println!("ファイルを開いて読む (1回)    : {:?}", start.elapsed() / n);
    std::fs::remove_file(&path).unwrap();

    // ネットワーク越しの通信の代わりに sleep で「待ち」を作る
    let start = Instant::now();
    thread::sleep(Duration::from_millis(10));
    println!("10ms 待つ (ネットワークの代役): {:?}", start.elapsed());

    println!("\n== スレッドを作るコスト ==");
    for count in [10, 100, 1000] {
        let start = Instant::now();
        let handles: Vec<_> = (0..count).map(|i| thread::spawn(move || i * 2)).collect();
        let total: usize = handles.into_iter().map(|h| h.join().unwrap()).sum();
        black_box(total);
        let elapsed = start.elapsed();
        println!(
            "{count:>5} 個のスレッドを作って待つ: {elapsed:?} (1個あたり {:?})",
            elapsed / count as u32
        );
    }

    println!("\n== 「待つ」処理を1000個、スレッドで同時にやる ==");
    // 各スレッドは 50ms 寝るだけ。CPU はほとんど使わない。
    let start = Instant::now();
    let handles: Vec<_> = (0..1000)
        .map(|_| thread::spawn(|| thread::sleep(Duration::from_millis(50))))
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    println!("1000スレッド × 50ms 待ち: {:?}", start.elapsed());
    println!("(1つずつ順番に待つと 50ms × 1000 = 50秒かかる)");
}
