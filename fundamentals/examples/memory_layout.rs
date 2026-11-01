//! 実験: スタックとヒープ、型のサイズ
//! cargo run -p fundamentals --example memory_layout

use std::mem::size_of;

fn main() {
    println!("== 型のサイズ (バイト) ==");
    println!("u8            : {}", size_of::<u8>());
    println!("i32           : {}", size_of::<i32>());
    println!("u64           : {}", size_of::<u64>());
    println!("char          : {}", size_of::<char>());
    println!("bool          : {}", size_of::<bool>());
    println!(
        "(u8, u32)     : {}  ← 足すと5なのに8 (アラインメント)",
        size_of::<(u8, u32)>()
    );
    println!("[i32; 4]      : {}", size_of::<[i32; 4]>());
    println!();
    println!("&i32          : {}  ← ポインタ1個", size_of::<&i32>());
    println!("Box<i32>      : {}  ← ポインタ1個", size_of::<Box<i32>>());
    println!("&str          : {}  ← ポインタ + 長さ", size_of::<&str>());
    println!("&[i32]        : {}  ← ポインタ + 長さ", size_of::<&[i32]>());
    println!(
        "String        : {}  ← ポインタ + 長さ + 容量",
        size_of::<String>()
    );
    println!(
        "Vec<i32>      : {}  ← ポインタ + 長さ + 容量",
        size_of::<Vec<i32>>()
    );
    println!(
        "Vec<[u8;1000]>: {}  ← 要素がどれだけ大きくても同じ",
        size_of::<Vec<[u8; 1000]>>()
    );
    println!();
    println!(
        "Option<&i32>  : {}  ← None をヌルポインタで表すので増えない",
        size_of::<Option<&i32>>()
    );
    println!(
        "Option<u32>   : {}  ← u32 (4バイト) には空き値がないので、None の印のぶん増える",
        size_of::<Option<u32>>()
    );

    println!("\n== String の中身を覗く ==");
    let mut s = String::with_capacity(4);
    s.push_str("hi");
    println!("s = {s:?}");
    println!("  スタック上の String 本体のアドレス: {:p}", &s);
    println!("  ヒープ上の文字データのアドレス   : {:p}", s.as_ptr());
    println!("  len = {}, capacity = {}", s.len(), s.capacity());

    println!("\n== ムーブで何がコピーされるのか ==");
    let heap_ptr_before = s.as_ptr();
    let t = s; // ムーブ
    println!("  ムーブ前のヒープアドレス: {heap_ptr_before:p}");
    println!(
        "  ムーブ後のヒープアドレス: {:p}  ← 同じ。中身はコピーされていない",
        t.as_ptr()
    );
    println!("  コピーされたのはスタック上の24バイト (ポインタ・長さ・容量) だけ");

    println!("\n== clone はヒープごとコピーする ==");
    let u = t.clone();
    println!("  t のヒープアドレス: {:p}", t.as_ptr());
    println!(
        "  u のヒープアドレス: {:p}  ← 違う。新しく確保された",
        u.as_ptr()
    );
}
