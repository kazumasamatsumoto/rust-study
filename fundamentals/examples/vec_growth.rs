//! 実験: Vec が伸びるときに起きること
//! cargo run -p fundamentals --example vec_growth --release

use std::time::Instant;

fn main() {
    println!("== push するたびに capacity はどう変わるか ==");
    let mut v: Vec<u64> = Vec::new();
    let mut last_cap = v.capacity();
    let mut last_ptr = v.as_ptr();
    println!("len={:>4} cap={:>4}", v.len(), v.capacity());
    for i in 0..1000u64 {
        v.push(i);
        if v.capacity() != last_cap {
            let moved = if v.as_ptr() != last_ptr {
                "移動した"
            } else {
                ""
            };
            println!("len={:>4} cap={:>4} {moved}", v.len(), v.capacity());
            last_cap = v.capacity();
            last_ptr = v.as_ptr();
        }
    }

    println!("\n== 先頭に挿入 (O(n)) と末尾に追加 (償却 O(1)) ==");
    let n = 50_000;

    let start = Instant::now();
    let mut back = Vec::new();
    for i in 0..n {
        back.push(i);
    }
    println!("push      x{n}: {:?}", start.elapsed());

    let start = Instant::now();
    let mut front = Vec::new();
    for i in 0..n {
        front.insert(0, i);
    }
    println!("insert(0) x{n}: {:?}", start.elapsed());
}
