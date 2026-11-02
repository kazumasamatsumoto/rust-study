//! 実験: メモリ階層とキャッシュ
//! cargo run -p fundamentals --example cache --release
//!
//! 必ず --release で実行すること。debug ビルドでは命令の実行そのものが遅すぎて、
//! メモリの待ち時間の差が埋もれてしまう。

use std::collections::LinkedList;
use std::hint::black_box;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

/// 同じ処理を何回か実行して、一番速かった時間を返す。
/// 1回目はページの確保などの雑音が入るので、最小値を使う。
fn best_of<F: FnMut() -> u64>(runs: usize, mut f: F) -> Duration {
    let mut best = Duration::MAX;
    for _ in 0..runs {
        let start = Instant::now();
        black_box(f());
        best = best.min(start.elapsed());
    }
    best
}

/// 依存クレートなしで使える、簡単な疑似乱数 (xorshift)。
struct XorShift(u64);

impl XorShift {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

fn shuffled_indices(n: usize, seed: u64) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..n).collect();
    let mut rng = XorShift(seed);
    // Fisher-Yates シャッフル
    for i in (1..n).rev() {
        let j = (rng.next() % (i as u64 + 1)) as usize;
        idx.swap(i, j);
    }
    idx
}

fn row_vs_column() {
    println!("== 1. 2次元配列: 行優先 vs 列優先 ==");
    const N: usize = 4096;
    // N×N の行列を1本の Vec に「行ごとに」並べて持つ (row-major)
    let m: Vec<u32> = (0..N * N).map(|i| i as u32).collect();
    println!(
        "行列のサイズ: {N}x{N} の u32 = {} MB",
        N * N * 4 / 1024 / 1024
    );

    let row = best_of(5, || {
        let mut s = 0u64;
        for r in 0..N {
            for c in 0..N {
                s += m[r * N + c] as u64; // メモリ上で隣の要素へ進む
            }
        }
        s
    });
    let col = best_of(5, || {
        let mut s = 0u64;
        for c in 0..N {
            for r in 0..N {
                s += m[r * N + c] as u64; // 毎回 N*4 バイト先へ飛ぶ
            }
        }
        s
    });
    println!("行優先 (連続アクセス)       : {row:?}");
    println!("列優先 (16KB 飛ばしアクセス): {col:?}");
    println!(
        "列優先は {:.1} 倍遅い",
        col.as_secs_f64() / row.as_secs_f64()
    );
}

fn sequential_vs_random() {
    println!("\n== 2. 連続アクセス vs ランダムアクセス (同じ要素を同じ回数読む) ==");
    for &n in &[1usize << 12, 1 << 16, 1 << 20, 1 << 24] {
        let data: Vec<u64> = (0..n as u64).collect();
        let seq_idx: Vec<usize> = (0..n).collect();
        let rnd_idx = shuffled_indices(n, 42);

        let seq = best_of(5, || seq_idx.iter().map(|&i| data[i]).sum());
        let rnd = best_of(5, || rnd_idx.iter().map(|&i| data[i]).sum());
        let kb = n * 8 / 1024;
        println!(
            "データ {:>7} KB: 連続 {:>6.2} ns/要素, ランダム {:>6.2} ns/要素 ({:.1} 倍)",
            kb,
            seq.as_nanos() as f64 / n as f64,
            rnd.as_nanos() as f64 / n as f64,
            rnd.as_secs_f64() / seq.as_secs_f64()
        );
    }
}

fn latency_ladder() {
    println!("\n== 3. メモリ階層のはしご: データの大きさごとの「1回の読み込み」の待ち時間 ==");
    // next[i] に「次に読む位置」を入れておき、p = next[p] を繰り返す (ポインタ追跡)。
    // 次の位置は今の読み込みが終わるまで分からないので、CPU は先読みもまとめ読みもできない。
    // そのため、1回の読み込みにかかる「待ち時間 (レイテンシ)」がそのまま見える。
    for kb in [16usize, 64, 256, 1024, 4096, 16384, 65536, 262144] {
        let n = kb * 1024 / 8;
        let perm = shuffled_indices(n, 1);
        // 全要素を1周する大きな輪を作る: perm[0] -> perm[1] -> ... -> perm[n-1] -> perm[0]
        let mut next = vec![0usize; n];
        for i in 0..n {
            next[perm[i]] = perm[(i + 1) % n];
        }
        const STEPS: usize = 10_000_000;
        let t = best_of(3, || {
            let mut p = 0usize;
            for _ in 0..STEPS {
                p = next[p];
            }
            p as u64
        });
        println!(
            "データ {:>7} KB: {:>7.2} ns/回",
            kb,
            t.as_nanos() as f64 / STEPS as f64
        );
    }
}

fn vec_vs_linked_list() {
    println!("\n== 4. Vec vs LinkedList の走査 ==");
    const N: usize = 1_000_000;
    let v: Vec<u64> = (0..N as u64).collect();

    // ノードがメモリ上で散らばるように、他の確保を挟みながらリストを作る
    let mut list = LinkedList::new();
    let mut noise: Vec<Vec<u8>> = Vec::new();
    let mut rng = XorShift(7);
    for i in 0..N as u64 {
        list.push_back(i);
        noise.push(vec![0u8; (rng.next() % 64) as usize + 1]);
    }
    drop(noise);

    let tv = best_of(5, || v.iter().sum());
    let tl = best_of(5, || list.iter().sum());
    println!("Vec        : {tv:?}");
    println!(
        "LinkedList : {tl:?}  ({:.1} 倍)",
        tl.as_secs_f64() / tv.as_secs_f64()
    );
}

#[allow(dead_code)] // x 以外のフィールドは「重さ」を作るためだけにある
#[derive(Clone, Copy)]
struct ParticleAoS {
    x: u32,
    y: u32,
    z: u32,
    vx: u32,
    vy: u32,
    vz: u32,
    mass: u32,
    charge: u32,
    id: u64,
    flags: u64,
}

struct ParticlesSoA {
    x: Vec<u32>,
    // 本来は y, z, vx ... も別々の Vec で持つ。この実験では x しか読まないので省略。
}

fn aos_vs_soa() {
    println!("\n== 5. AoS (構造体の配列) vs SoA (配列の構造体) ==");
    const N: usize = 4_000_000;
    let aos: Vec<ParticleAoS> = (0..N)
        .map(|i| ParticleAoS {
            x: i as u32,
            y: 0,
            z: 0,
            vx: 0,
            vy: 0,
            vz: 0,
            mass: 1,
            charge: 0,
            id: i as u64,
            flags: 0,
        })
        .collect();
    let soa = ParticlesSoA {
        x: (0..N).map(|i| i as u32).collect(),
    };
    println!(
        "ParticleAoS 1個のサイズ: {} バイト (そのうち x は 4 バイト)",
        std::mem::size_of::<ParticleAoS>()
    );

    let ta = best_of(5, || black_box(&aos).iter().map(|p| p.x as u64).sum());
    let ts = best_of(5, || black_box(&soa.x).iter().map(|&x| x as u64).sum());
    println!("AoS で x の合計: {ta:?}");
    println!(
        "SoA で x の合計: {ts:?}  (AoS は {:.1} 倍遅い)",
        ta.as_secs_f64() / ts.as_secs_f64()
    );
}

fn branch_prediction() {
    println!("\n== 6. 分岐予測: ソート済み vs 未ソート ==");
    const N: usize = 1 << 20;
    let mut rng = XorShift(99);
    let unsorted: Vec<u8> = (0..N).map(|_| (rng.next() % 256) as u8).collect();
    let mut sorted = unsorted.clone();
    sorted.sort_unstable();

    // 分岐のあるループ。条件が成り立ったときだけ black_box を通すことで、
    // コンパイラが「条件付き選択命令」や SIMD に置き換えて分岐を消すのを防いでいる。
    #[inline(never)]
    fn count_branchy(data: &[u8]) -> u64 {
        let mut sum = 0u64;
        for &x in data {
            if x >= 128 {
                sum = black_box(sum + x as u64);
            }
        }
        sum
    }
    // 同じ計算を、分岐なしで書いたもの (条件を 0/1 の掛け算にする)
    #[inline(never)]
    fn count_branchless(data: &[u8]) -> u64 {
        data.iter().map(|&x| x as u64 * (x >= 128) as u64).sum()
    }

    let bu = best_of(5, || count_branchy(black_box(&unsorted)));
    let bs = best_of(5, || count_branchy(black_box(&sorted)));
    let lu = best_of(5, || count_branchless(black_box(&unsorted)));
    let ls = best_of(5, || count_branchless(black_box(&sorted)));
    println!("分岐あり  / 未ソート: {bu:?}");
    println!(
        "分岐あり  / ソート済: {bs:?}  (未ソートは {:.1} 倍遅い)",
        bu.as_secs_f64() / bs.as_secs_f64()
    );
    println!("分岐なし  / 未ソート: {lu:?}");
    println!("分岐なし  / ソート済: {ls:?}");
}

/// 2つのカウンタが同じキャッシュラインに乗る
struct Adjacent {
    a: AtomicU64,
    b: AtomicU64,
}

/// 128 バイト境界に揃えて、別々のキャッシュラインに置く
#[repr(align(128))]
struct Padded(AtomicU64);

fn false_sharing() {
    println!("\n== 7. フォルスシェアリング ==");
    const ITERS: u64 = 20_000_000;

    let adj = Adjacent {
        a: AtomicU64::new(0),
        b: AtomicU64::new(0),
    };
    let t_adj = {
        let start = Instant::now();
        thread::scope(|s| {
            s.spawn(|| {
                for _ in 0..ITERS {
                    adj.a.fetch_add(1, Ordering::Relaxed);
                }
            });
            s.spawn(|| {
                for _ in 0..ITERS {
                    adj.b.fetch_add(1, Ordering::Relaxed);
                }
            });
        });
        start.elapsed()
    };

    let pa = Padded(AtomicU64::new(0));
    let pb = Padded(AtomicU64::new(0));
    let t_pad = {
        let start = Instant::now();
        thread::scope(|s| {
            s.spawn(|| {
                for _ in 0..ITERS {
                    pa.0.fetch_add(1, Ordering::Relaxed);
                }
            });
            s.spawn(|| {
                for _ in 0..ITERS {
                    pb.0.fetch_add(1, Ordering::Relaxed);
                }
            });
        });
        start.elapsed()
    };

    println!(
        "a と b のアドレスの差: {} バイト (同じキャッシュライン)",
        &adj.b as *const _ as usize - &adj.a as *const _ as usize
    );
    println!("隣り合ったカウンタ    : {t_adj:?}");
    println!(
        "128 バイト離したカウンタ: {t_pad:?}  (隣り合うと {:.1} 倍遅い)",
        t_adj.as_secs_f64() / t_pad.as_secs_f64()
    );
    // 結果が正しいことも確認しておく
    assert_eq!(
        adj.a.load(Ordering::Relaxed) + adj.b.load(Ordering::Relaxed),
        ITERS * 2
    );
}

fn main() {
    row_vs_column();
    sequential_vs_random();
    latency_ladder();
    vec_vs_linked_list();
    aos_vs_soa();
    branch_prediction();
    false_sharing();
}
