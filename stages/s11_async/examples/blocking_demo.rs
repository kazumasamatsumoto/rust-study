//! 実験: async の中でブロッキング処理をすると何が起きるか
//! cargo run -p s11_async --example blocking_demo
//!
//! シングルスレッドのランタイムで、「100ms ごとに時刻を表示するタスク」を動かしながら、
//! 別のタスクで 500ms の処理をする。処理のやり方を3通り比べる。

use std::time::{Duration, Instant};

async fn ticker(start: Instant, label: &'static str) {
    for _ in 0..6 {
        println!("  [{label}] tick at {:>4}ms", start.elapsed().as_millis());
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

fn main() {
    // あえてスレッド1本だけのランタイムを作る
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    println!("== A: tokio::time::sleep (正しい待ち方) ==");
    rt.block_on(async {
        let start = Instant::now();
        let t = tokio::spawn(ticker(start, "A"));
        tokio::time::sleep(Duration::from_millis(500)).await;
        t.await.unwrap();
    });

    println!("\n== B: std::thread::sleep (async の中でやってはいけない) ==");
    rt.block_on(async {
        let start = Instant::now();
        let t = tokio::spawn(ticker(start, "B"));
        tokio::task::yield_now().await; // ticker を一度動かす
        std::thread::sleep(Duration::from_millis(500)); // スレッドごと止まる
        t.await.unwrap();
    });

    println!("\n== C: spawn_blocking に逃がす ==");
    rt.block_on(async {
        let start = Instant::now();
        let t = tokio::spawn(ticker(start, "C"));
        tokio::task::spawn_blocking(|| std::thread::sleep(Duration::from_millis(500)))
            .await
            .unwrap();
        t.await.unwrap();
    });
}
