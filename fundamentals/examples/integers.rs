//! 実験: 整数の表現とオーバーフロー
//! cargo run -p fundamentals --example integers
//! cargo run -p fundamentals --example integers --release   ← debug との違いを見る

use std::hint::black_box;

fn main() {
    println!("== 2進数で見る ==");
    for n in [0u8, 1, 2, 5, 127, 128, 255] {
        println!("{n:>3} = {n:08b}");
    }

    println!("\n== 型ごとの範囲 ==");
    println!("u8 : {} ..= {}", u8::MIN, u8::MAX);
    println!("i8 : {} ..= {}", i8::MIN, i8::MAX);
    println!("i32: {} ..= {}", i32::MIN, i32::MAX);
    println!("u64: {} ..= {}", u64::MIN, u64::MAX);

    println!("\n== 2の補数: 負の数のビット列 ==");
    for n in [1i8, -1, -2, 127, -128] {
        // as u8 で「同じビット列を符号なしとして見る」
        println!("{n:>4} = {:08b}", n as u8);
    }

    println!("\n== オーバーフローの扱いを明示するメソッド ==");
    let x: u8 = 250;
    println!("250u8.checked_add(10)     = {:?}", x.checked_add(10));
    println!("250u8.wrapping_add(10)    = {}", x.wrapping_add(10));
    println!("250u8.saturating_add(10)  = {}", x.saturating_add(10));
    println!("250u8.overflowing_add(10) = {:?}", x.overflowing_add(10));
    println!("127i8.wrapping_add(1)     = {}", 127i8.wrapping_add(1));

    println!("\n== as による変換は黙って切り捨てる ==");
    println!("300i32 as u8  = {}", 300i32 as u8);
    println!("-1i32 as u32  = {}", -1i32 as u32);
    println!("3.99f64 as i32 = {}", 3.99f64 as i32);
    println!(
        "1e20f64 as i32 = {}  (浮動小数点→整数は飽和する)",
        1e20f64 as i32
    );
    println!("u8::try_from(300i32) = {:?}", u8::try_from(300i32));

    println!("\n== 普通の + でオーバーフローさせる ==");
    // black_box はコンパイラに値を推測させないためのもの。
    // これがないと、コンパイル時にオーバーフローが検出されてエラーになる。
    let a: u8 = black_box(255);
    let result = std::panic::catch_unwind(|| a + 1);
    match result {
        Ok(v) => println!("255u8 + 1 = {v}  ← release ビルド: ラップアラウンドした"),
        Err(_) => println!("255u8 + 1 → panic  ← debug ビルド: オーバーフローを検出した"),
    }
}
