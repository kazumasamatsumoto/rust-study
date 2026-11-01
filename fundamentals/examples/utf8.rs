//! 実験: 文字コードと UTF-8
//! cargo run -p fundamentals --example utf8

fn show(s: &str) {
    let bytes: Vec<String> = s.bytes().map(|b| format!("{b:02X}")).collect();
    let chars: Vec<String> = s.chars().map(|c| format!("U+{:04X}", c as u32)).collect();
    println!("{s:?}");
    println!("  len() (バイト数) = {}", s.len());
    println!("  chars().count()  = {}", s.chars().count());
    println!("  バイト列         = {}", bytes.join(" "));
    println!("  コードポイント   = {}", chars.join(" "));
}

fn main() {
    show("a");
    show("é");
    show("あ");
    show("🦀");
    show("Rustは楽しい");
    show("👨‍👩‍👧");

    println!("\n== 見た目は同じでも中身が違う ==");
    let nfc = "\u{00E9}"; // é を1文字で表す
    let nfd = "e\u{0301}"; // e + 合成用アクセント記号
    println!("{nfc:?} と {nfd:?}: 等しい? → {}", nfc == nfd);
    show(nfd);

    println!("\n== バイト位置でスライスする ==");
    let s = "あいう";
    println!("&s[0..3] = {:?}", &s[0..3]);
    println!("s.get(0..2) = {:?}  ← 文字の途中で切ると None", s.get(0..2));
    // &s[0..2] と書くと実行時に panic する
}
