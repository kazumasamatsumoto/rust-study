//! 実験: 浮動小数点数
//! cargo run -p fundamentals --example floats

// NaN の性質を見せるために、わざと 0.0 / 0.0 や nan == nan と書いている。
// 普段のコードでは clippy の指摘どおり f64::NAN や .is_nan() を使うこと。
#![allow(clippy::eq_op, clippy::zero_divided_by_zero)]

fn main() {
    println!("== 0.1 は2進数で正確に表せない ==");
    println!("0.1 + 0.2        = {}", 0.1 + 0.2);
    println!("0.1 + 0.2 == 0.3 → {}", 0.1 + 0.2 == 0.3);
    println!("0.1 を精度20桁で = {:.20}", 0.1);
    println!(
        "差が十分小さいか → {}",
        ((0.1f64 + 0.2) - 0.3).abs() < f64::EPSILON
    );

    println!("\n== 大きな数では整数すら正確に表せない ==");
    let big = 9_007_199_254_740_992.0f64; // 2^53
    println!("2^53     = {big}");
    println!("2^53 + 1 = {}  ← 1 が消えた", big + 1.0);

    println!("\n== 特殊な値 ==");
    println!("1.0 / 0.0  = {}", 1.0f64 / 0.0);
    println!("-1.0 / 0.0 = {}", -1.0f64 / 0.0);
    let nan = 0.0f64 / 0.0;
    println!("0.0 / 0.0  = {nan}");
    println!("NaN == NaN → {}  ← 自分自身と等しくない!", nan == nan);

    println!("\n== だから f64 は Ord を実装していない ==");
    let mut v = vec![3.2, f64::NAN, 1.5, -0.0, 2.7];
    // v.sort(); ← コンパイルエラー: f64 は Ord ではない
    v.sort_by(f64::total_cmp);
    println!("total_cmp で並べた結果: {v:?}");
}
