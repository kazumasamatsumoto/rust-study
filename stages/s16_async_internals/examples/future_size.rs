//! 実験: async ブロックが作る Future の大きさ
//! cargo run -p s16_async_internals --example future_size
//!
//! async ブロックはコンパイラによってステートマシン（enum のような型）に変換されます。
//! `.await` をまたいで生きている変数は、その状態の中に保存されるので、Future が大きくなります。

use std::future::Future;
use std::mem::size_of_val;
use std::pin::Pin;
use std::task::{Context, Poll};

/// 一度だけ Pending を返す Future（中断点を作るためのもの）
struct YieldOnce(bool);

impl Future for YieldOnce {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.0 {
            Poll::Ready(())
        } else {
            self.0 = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn yield_once() -> YieldOnce {
    YieldOnce(false)
}

fn main() {
    println!("== async ブロックの Future のサイズ (バイト) ==");

    let empty = async {};
    println!(
        "何もしない                         : {:>5}",
        size_of_val(&empty)
    );

    let no_await = async {
        let buf = [0u8; 1024];
        buf.iter().map(|&b| b as u32).sum::<u32>()
    };
    println!(
        "1KB の配列を使うが await しない     : {:>5}",
        size_of_val(&no_await)
    );

    let small_across = async {
        let x: u64 = 1;
        yield_once().await;
        x + 1
    };
    println!(
        "u64 を1つ持ったまま await           : {:>5}",
        size_of_val(&small_across)
    );

    let big_across = async {
        let buf = [0u8; 1024];
        yield_once().await;
        buf.iter().map(|&b| b as u32).sum::<u32>()
    };
    println!(
        "1KB の配列を持ったまま await        : {:>5}",
        size_of_val(&big_across)
    );

    let dropped_before = async {
        {
            let buf = [0u8; 1024];
            let _sum: u32 = buf.iter().map(|&b| b as u32).sum();
        } // ここで buf は消える
        yield_once().await;
    };
    println!(
        "1KB の配列を await の前に捨てる     : {:>5}",
        size_of_val(&dropped_before)
    );

    let two_points = async {
        let a = [0u8; 512];
        yield_once().await;
        let b = [0u8; 512];
        yield_once().await;
        a[0] as u32 + b[0] as u32
    };
    println!(
        "512B を2つ、別々の await をまたぐ   : {:>5}",
        size_of_val(&two_points)
    );

    let overlap = async {
        // ブロック { } で、a と b が生きている範囲を分ける
        let sa = {
            let a = [1u8; 512];
            yield_once().await; // ここでは a だけが生きている
            a[0] as u32
        };
        let sb = {
            let b = [2u8; 512];
            yield_once().await; // ここでは b だけが生きている
            b[0] as u32
        };
        sa + sb
    };
    println!(
        "512B ずつ、生きている範囲を分ける   : {:>5}",
        size_of_val(&overlap)
    );

    let boxed = async {
        let buf = Box::new([0u8; 1024]);
        yield_once().await;
        buf[0]
    };
    println!(
        "1KB を Box に入れて await           : {:>5}",
        size_of_val(&boxed)
    );

    let nested = async {
        big_across_fn().await;
    };
    println!(
        "1KB を持つ async fn を await する   : {:>5}",
        size_of_val(&nested)
    );

    println!();
    println!(
        "比較: スレッド1本のスタック (Rust の spawn のデフォルト) = 2 MiB = {} バイト",
        2 * 1024 * 1024
    );
}

async fn big_across_fn() -> u32 {
    let buf = [0u8; 1024];
    yield_once().await;
    buf.iter().map(|&b| b as u32).sum::<u32>()
}
