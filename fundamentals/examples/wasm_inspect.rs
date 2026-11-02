//! 実験: .wasm ファイルの中身を、自分で読んでみる（小さな wasm-objdump）
//!
//! cargo build -p s17_wasm --target wasm32-unknown-unknown --release
//! cargo run -p fundamentals --example wasm_inspect
//! cargo run -p fundamentals --example wasm_inspect -- <別の .wasm ファイル>
//!
//! .wasm は「ヘッダ + セクションの並び」というとても単純な形式です。
//! 各セクションは [セクション番号 1バイト][中身のバイト数 (LEB128)][中身] になっています。

use std::env;
use std::fs;
use std::process;

/// LEB128 という可変長の整数形式を読む。
/// 1バイトのうち下位7ビットが値、最上位ビットが「まだ続くか」の印。
/// 小さい数は1バイトで済むので、ファイルが小さくなる（UTF-8 と似た発想。CS基礎 05）。
fn read_leb_u32(bytes: &[u8], pos: &mut usize) -> u32 {
    let mut result = 0u32;
    let mut shift = 0;
    loop {
        let b = bytes[*pos];
        *pos += 1;
        result |= ((b & 0x7f) as u32) << shift;
        if b & 0x80 == 0 {
            return result;
        }
        shift += 7;
    }
}

fn read_name(bytes: &[u8], pos: &mut usize) -> String {
    let len = read_leb_u32(bytes, pos) as usize;
    let s = String::from_utf8_lossy(&bytes[*pos..*pos + len]).into_owned();
    *pos += len;
    s
}

fn section_name(id: u8) -> &'static str {
    match id {
        0 => "custom",
        1 => "type",
        2 => "import",
        3 => "function",
        4 => "table",
        5 => "memory",
        6 => "global",
        7 => "export",
        8 => "start",
        9 => "element",
        10 => "code",
        11 => "data",
        12 => "datacount",
        _ => "?",
    }
}

fn kind_name(kind: u8) -> &'static str {
    match kind {
        0 => "関数",
        1 => "テーブル",
        2 => "メモリ",
        3 => "グローバル変数",
        _ => "?",
    }
}

fn main() {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "target/wasm32-unknown-unknown/release/s17_wasm.wasm".to_string());
    let bytes = match fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{path} を読めませんでした: {e}");
            eprintln!("先にビルドしてください:");
            eprintln!("  cargo build -p s17_wasm --target wasm32-unknown-unknown --release");
            process::exit(1);
        }
    };

    println!("{path}: {} バイト", bytes.len());
    println!("\n== ヘッダ (8 バイト) ==");
    println!(
        "マジックナンバー: {:02x?} = {:?}",
        &bytes[0..4],
        String::from_utf8_lossy(&bytes[0..4])
    );
    println!(
        "バージョン      : {:02x?} = {}",
        &bytes[4..8],
        u32::from_le_bytes(bytes[4..8].try_into().unwrap())
    );
    if &bytes[0..4] != b"\0asm" {
        eprintln!("WASM ファイルではありません");
        process::exit(1);
    }

    // 後で「大きい関数」を表示するために集めておく
    let mut imported_funcs = 0usize;
    let mut body_sizes: Vec<usize> = Vec::new();
    let mut func_names: Vec<(usize, String)> = Vec::new();

    println!("\n== セクション ==");
    println!("{:>4} {:<10} {:>8}  中身", "番号", "名前", "バイト数");
    let mut pos = 8;
    while pos < bytes.len() {
        let id = bytes[pos];
        pos += 1;
        let size = read_leb_u32(&bytes, &mut pos) as usize;
        let body_start = pos;
        let mut p = pos;

        let detail = match id {
            0 => {
                let name = read_name(&bytes, &mut p);
                if name == "name" {
                    // name セクションの中の「関数名」の部分 (サブセクション 1) を読む
                    while p < body_start + size {
                        let sub_id = bytes[p];
                        p += 1;
                        let sub_size = read_leb_u32(&bytes, &mut p) as usize;
                        let sub_end = p + sub_size;
                        if sub_id == 1 {
                            let count = read_leb_u32(&bytes, &mut p);
                            for _ in 0..count {
                                let index = read_leb_u32(&bytes, &mut p) as usize;
                                func_names.push((index, read_name(&bytes, &mut p)));
                            }
                        }
                        p = sub_end;
                    }
                }
                format!("名前: {name:?}")
            }
            2 => {
                let count = read_leb_u32(&bytes, &mut p);
                let mut items = Vec::new();
                for _ in 0..count {
                    let module = read_name(&bytes, &mut p);
                    let field = read_name(&bytes, &mut p);
                    let kind = bytes[p];
                    p += 1;
                    // 関数の import なら、続く型の番号を読み飛ばす（他の種類はここでは扱わない）
                    if kind == 0 {
                        read_leb_u32(&bytes, &mut p);
                        imported_funcs += 1;
                    } else {
                        items.push(format!(
                            "{module}.{field} ({}, 以降は省略)",
                            kind_name(kind)
                        ));
                        break;
                    }
                    items.push(format!("{module}.{field} ({})", kind_name(kind)));
                }
                items.join(", ")
            }
            7 => {
                let count = read_leb_u32(&bytes, &mut p);
                let mut items = Vec::new();
                for _ in 0..count {
                    let name = read_name(&bytes, &mut p);
                    let kind = bytes[p];
                    p += 1;
                    read_leb_u32(&bytes, &mut p);
                    items.push(format!("{name} ({})", kind_name(kind)));
                }
                items.join(", ")
            }
            10 => {
                let count = read_leb_u32(&bytes, &mut p);
                for _ in 0..count {
                    let body_size = read_leb_u32(&bytes, &mut p) as usize;
                    body_sizes.push(body_size);
                    p += body_size;
                }
                format!("{count} 個")
            }
            1 | 3 => format!("{} 個", read_leb_u32(&bytes, &mut p)),
            _ => String::new(),
        };

        println!("{id:>4} {:<10} {size:>8}  {detail}", section_name(id));
        pos = body_start + size;
    }

    // 関数の番号は「import した関数」が先で、その後に「モジュールの中で定義した関数」が続く
    if func_names.is_empty() {
        println!("\n(name セクションがないので、関数名は分かりません。strip されている？)");
        return;
    }
    let mut sized: Vec<(usize, &str)> = func_names
        .iter()
        .filter(|(index, _)| *index >= imported_funcs)
        .filter_map(|(index, name)| Some((*body_sizes.get(index - imported_funcs)?, name.as_str())))
        .collect();
    sized.sort_by_key(|&(size, _)| std::cmp::Reverse(size));
    println!("\n== 大きい関数 上位 10 個 (全 {} 個) ==", body_sizes.len());
    for (size, name) in sized.iter().take(10) {
        let short: String = name.chars().take(90).collect();
        println!("{size:>7} バイト  {short}");
    }
}
