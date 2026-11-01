//! 実験: スレッドと共有データ
//! cargo run -p fundamentals --example threads --release

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

const PER_THREAD: usize = 1_000_000;

fn main() {
    println!("== このマシンの論理 CPU 数 ==");
    let cpus = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    println!("available_parallelism = {cpus}");

    println!("\n== スレッドの実行順は毎回変わる ==");
    // 4つのスレッドが、それぞれ自分の番号を共有の Vec に 1000 回ずつ書き込む。
    // 書き込まれた順番を見ると、OS がスレッドをいつ切り替えたかが分かる。
    for round in 1..=3 {
        let log = Arc::new(Mutex::new(Vec::new()));
        let handles: Vec<_> = (0..4)
            .map(|id| {
                let log = Arc::clone(&log);
                thread::spawn(move || {
                    for _ in 0..1000 {
                        log.lock().unwrap().push(id);
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        let log = log.lock().unwrap();
        // 同じ番号が続く部分を「番号x回数」にまとめる
        let mut runs: Vec<(usize, usize)> = Vec::new();
        for &id in log.iter() {
            match runs.last_mut() {
                Some((last, n)) if *last == id => *n += 1,
                _ => runs.push((id, 1)),
            }
        }
        let head: Vec<String> = runs
            .iter()
            .take(6)
            .map(|(id, n)| format!("{id}x{n}"))
            .collect();
        println!(
            "{round}回目: 切り替わり {:>4} 回, 先頭: {} ...",
            runs.len() - 1,
            head.join(" ")
        );
    }

    println!(
        "\n== 4スレッドで合計 {} 回カウントアップする ==",
        4 * PER_THREAD
    );

    // (1) AtomicUsize: CPU の不可分な命令で足す
    let counter = Arc::new(AtomicUsize::new(0));
    let start = Instant::now();
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let c = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..PER_THREAD {
                    c.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    println!(
        "AtomicUsize  : 結果 = {:>7}, 時間 = {:?}",
        counter.load(Ordering::Relaxed),
        start.elapsed()
    );

    // (2) Mutex: ロックを取ってから足す
    let counter = Arc::new(Mutex::new(0usize));
    let start = Instant::now();
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let c = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..PER_THREAD {
                    *c.lock().unwrap() += 1;
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    println!(
        "Mutex        : 結果 = {:>7}, 時間 = {:?}",
        *counter.lock().unwrap(),
        start.elapsed()
    );

    // (3) 共有しない: 各スレッドが自分の変数で数え、最後に合算する
    let start = Instant::now();
    let handles: Vec<_> = (0..4)
        .map(|_| {
            thread::spawn(|| {
                let mut local = 0usize;
                for _ in 0..PER_THREAD {
                    local = std::hint::black_box(local + 1);
                }
                local
            })
        })
        .collect();
    let total: usize = handles.into_iter().map(|h| h.join().unwrap()).sum();
    println!(
        "共有しない   : 結果 = {total:>7}, 時間 = {:?}",
        start.elapsed()
    );

    println!("\n== 「読んでから書く」を2回に分けると、アトミックでも壊れる ==");
    let counter = Arc::new(AtomicUsize::new(0));
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let c = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..PER_THREAD {
                    let v = c.load(Ordering::Relaxed); // 読む
                    c.store(v + 1, Ordering::Relaxed); // 書く（その間に他のスレッドが割り込める）
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    println!(
        "load + store : 結果 = {:>7}  ← 期待値 {} より少ない（競合状態）",
        counter.load(Ordering::Relaxed),
        4 * PER_THREAD
    );

    println!("\n== スレッドを作るコスト ==");
    let n = 10_000;
    let start = Instant::now();
    for _ in 0..n {
        thread::spawn(|| {}).join().unwrap();
    }
    let elapsed = start.elapsed();
    println!(
        "spawn + join x{n}: {elapsed:?}  (1回あたり {:?})",
        elapsed / n
    );
}
