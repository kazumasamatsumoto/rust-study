//! 実験: 静的ディスパッチと動的ディスパッチ
//! cargo run -p fundamentals --example dispatch --release

use std::f64::consts::PI;
use std::hint::black_box;
use std::mem::size_of;
use std::time::Instant;

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    r: f64,
}
struct Square {
    side: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.r * self.r
    }
}
impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// 静的ディスパッチ: 型ごとに別の関数がコンパイル時に作られる（単相化）
fn total_static<T: Shape>(shapes: &[T]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// 動的ディスパッチ: 実行時に vtable を引いて呼ぶ関数を決める
fn total_dyn(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// 単相化で別々の関数ができていることを、関数ポインタのアドレスで確かめる
fn describe<T>(_: T) -> usize {
    size_of::<T>()
}

fn main() {
    println!("== ポインタのサイズ (バイト) ==");
    println!(
        "&Circle          : {}  ← アドレスだけ",
        size_of::<&Circle>()
    );
    println!(
        "&dyn Shape       : {}  ← アドレス + vtable へのポインタ",
        size_of::<&dyn Shape>()
    );
    println!("Box<Circle>      : {}", size_of::<Box<Circle>>());
    println!("Box<dyn Shape>   : {}", size_of::<Box<dyn Shape>>());

    println!("\n== 単相化: ジェネリック関数は型ごとに別の関数になる ==");
    let f_u8 = describe::<u8> as fn(u8) -> usize;
    let f_u64 = describe::<u64> as fn(u64) -> usize;
    let f_string = describe::<String> as fn(String) -> usize;
    println!("describe::<u8>     のアドレス: {:p}", f_u8 as *const ());
    println!("describe::<u64>    のアドレス: {:p}", f_u64 as *const ());
    println!("describe::<String> のアドレス: {:p}", f_string as *const ());

    println!("\n== dyn Shape の中身: データのアドレスと vtable のアドレス ==");
    let c = Circle { r: 1.0 };
    let s = Square { side: 2.0 };
    let c2 = Circle { r: 3.0 };
    for (name, obj) in [
        ("circle(r=1)", &c as &dyn Shape),
        ("square", &s as &dyn Shape),
        ("circle(r=3)", &c2 as &dyn Shape),
    ] {
        // &dyn Shape を (データポインタ, vtable ポインタ) の組として覗く。
        // この内部表現は言語として保証されたものではないので、実験以外では使わないこと。
        let (data, vtable): (*const (), *const ()) = unsafe { std::mem::transmute(obj) };
        println!(
            "{name:<12} data={data:p} vtable={vtable:p} area={}",
            obj.area()
        );
    }
    println!("→ 同じ型 (Circle) は同じ vtable を共有している");

    println!("\n== 速度の比較 (release で実行すること) ==");
    let n = 1_000_000;
    let circles: Vec<Circle> = (0..n).map(|i| Circle { r: i as f64 }).collect();
    let boxed: Vec<Box<dyn Shape>> = (0..n)
        .map(|i| Box::new(Circle { r: i as f64 }) as Box<dyn Shape>)
        .collect();

    for _ in 0..2 {
        let t = Instant::now();
        let a = total_static(black_box(&circles));
        let static_time = t.elapsed();

        let t = Instant::now();
        let b = total_dyn(black_box(&boxed));
        let dyn_time = t.elapsed();

        println!("static (Vec<Circle>)        : {static_time:?}  (sum={a:.3e})");
        println!("dyn    (Vec<Box<dyn Shape>>): {dyn_time:?}  (sum={b:.3e})");
    }
}
