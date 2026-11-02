//! 実験: 速さを正しく測る
//!
//! cargo run -p s15_performance --example measure --release
//! cargo run -p s15_performance --example measure            ← debug ビルドでも実行して比べる
//!
//! この実験は課題を解かなくても動きます。

use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};
use std::hint::black_box;
use std::time::{Duration, Instant};

fn time<R>(f: impl FnOnce() -> R) -> (R, Duration) {
    let start = Instant::now();
    let r = f();
    (r, start.elapsed())
}

/// 何回か実行して一番速い時間を返す
fn best_of<R>(runs: usize, mut f: impl FnMut() -> R) -> Duration {
    (0..runs)
        .map(|_| {
            let start = Instant::now();
            black_box(f());
            start.elapsed()
        })
        .min()
        .unwrap()
}

fn pitfall_optimized_away() {
    println!("== 1. 結果を使わないと、計算そのものが消える ==");
    let v: Vec<u64> = (0..10_000_000).collect();

    let (_, t) = time(|| {
        // 結果を捨てている。コンパイラは「誰も使わない計算」を丸ごと消してよい
        let _ = v.iter().map(|x| x * 3).sum::<u64>();
    });
    println!("結果を捨てる            : {t:>12.3?}");

    let (_, t) = time(|| {
        // black_box は「この値は誰かが使うかもしれない」とコンパイラに思わせる
        black_box(v.iter().map(|x| x * 3).sum::<u64>());
    });
    println!("結果を black_box に渡す : {t:>12.3?}");
    println!(
        "→ 前者は何も計算していない。「速くなった！」と喜ぶ前に、計算が消えていないか疑うこと"
    );
}

fn pitfall_first_run() {
    println!("\n== 2. 1 回目だけ遅い（ウォームアップ） ==");
    // vec![0; n] は、OS が「0 で埋まったページ」を後から渡す仕組みを使うので、
    // 確保した時点ではまだ物理メモリが割り当てられていない。最初に触ったときに割り当てられる。
    let v = vec![0u64; 32 * 1024 * 1024]; // 256 MB
    for i in 1..=5 {
        let (s, t) = time(|| black_box(&v).iter().sum::<u64>());
        black_box(s);
        println!("{i} 回目: {t:>10.3?}");
    }
    println!(
        "→ 1 回目には「ページを初めて触る」コストが含まれている。測るときは何回か回して、最初の結果は捨てるか、最小値や中央値を使う"
    );
}

fn variance() {
    println!("\n== 3. 同じ処理でも毎回ばらつく ==");
    let v: Vec<u64> = (0..1_000_000).collect();
    let mut times: Vec<Duration> = (0..20)
        .map(|_| {
            let (s, t) = time(|| v.iter().map(|x| x * x).sum::<u64>());
            black_box(s);
            t
        })
        .collect();
    times.sort();
    println!(
        "20 回実行: 最小 {:?} / 中央値 {:?} / 最大 {:?}",
        times[0], times[10], times[19]
    );
    println!(
        "→ 1 回だけ測って「速くなった」と判断してはいけない。criterion などのクレートは、これを統計的に処理してくれる"
    );
}

fn iterators_are_not_slow() {
    println!("\n== 4. イテレータは遅くない（ゼロコスト抽象化） ==");
    // 2 乗の合計が u64 に収まるように、値は 0〜999 にしている
    let v: Vec<u64> = (0..10_000_000).map(|x| x % 1000).collect();

    // C 言語風に、インデックスで書いたループ（clippy は「イテレータを使え」と言うが、比較のためにわざと書いている）
    #[allow(clippy::needless_range_loop)]
    let t_index = best_of(5, || {
        let mut s = 0u64;
        for i in 0..v.len() {
            if v[i].is_multiple_of(2) {
                s += v[i] * v[i];
            }
        }
        s
    });
    let t_iter = best_of(5, || {
        v.iter()
            .filter(|&&x| x.is_multiple_of(2))
            .map(|&x| x * x)
            .sum::<u64>()
    });
    println!("インデックスのループ        : {t_index:>10.3?}");
    println!("イテレータ (filter → map → sum): {t_iter:>10.3?}");
}

/// FNV-1a（課題 03 で自分で書くものと同じ）。実験のためにここにも書いておく。
#[derive(Default)]
struct Fnv(u64);
impl Hasher for Fnv {
    fn write(&mut self, bytes: &[u8]) {
        if self.0 == 0 {
            self.0 = 0xcbf2_9ce4_8422_2325;
        }
        for b in bytes {
            self.0 ^= *b as u64;
            self.0 = self.0.wrapping_mul(0x100_0000_01b3);
        }
    }
    fn finish(&self) -> u64 {
        self.0
    }
}

fn hash_functions() {
    println!("\n== 5. HashMap のハッシュ関数 ==");
    const N: u64 = 1_000_000;
    let t_sip = best_of(3, || {
        let mut m: HashMap<u64, u64> = HashMap::new();
        for i in 0..N {
            *m.entry(i.wrapping_mul(2_654_435_761)).or_insert(0) += 1;
        }
        m.len()
    });
    let t_fnv = best_of(3, || {
        let mut m: HashMap<u64, u64, BuildHasherDefault<Fnv>> = HashMap::default();
        for i in 0..N {
            *m.entry(i.wrapping_mul(2_654_435_761)).or_insert(0) += 1;
        }
        m.len()
    });
    println!("標準 (SipHash-1-3) で 100 万回 entry: {t_sip:>10.3?}");
    println!("FNV-1a で 100 万回 entry            : {t_fnv:>10.3?}");
}

fn with_capacity() {
    println!("\n== 6. with_capacity: 小さな Vec をたくさん作る ==");
    // 長さ 100 の Vec を 10 万個作る。Vec::new から push すると、容量が 0→4→8→...→128 と
    // 伸びるたびに確保し直す（CS基礎 06）。with_capacity なら 1 個につき 1 回で済む。
    const COUNT: usize = 100_000;
    const LEN: usize = 100;
    let t_new = best_of(5, || {
        let mut total = 0;
        for _ in 0..COUNT {
            let mut v = Vec::new();
            for i in 0..LEN {
                v.push(black_box(i));
            }
            total += v.len();
        }
        total
    });
    let t_cap = best_of(5, || {
        let mut total = 0;
        for _ in 0..COUNT {
            let mut v = Vec::with_capacity(LEN);
            for i in 0..LEN {
                v.push(black_box(i));
            }
            total += v.len();
        }
        total
    });
    println!("Vec::new から push      : {t_new:>10.3?}  (1 個あたり 6 回確保する)");
    println!("with_capacity から push : {t_cap:>10.3?}  (1 個あたり 1 回)");
}

fn main() {
    if cfg!(debug_assertions) {
        println!("※ debug ビルドで実行しています。性能を測るときは --release を付けてください。\n");
    }
    pitfall_optimized_away();
    pitfall_first_run();
    variance();
    iterators_are_not_slow();
    hash_functions();
    with_capacity();
}
