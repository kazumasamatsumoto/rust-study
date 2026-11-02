//! # Stage 17: WebAssembly を素手で触る
//!
//! wasm-bindgen のような便利なツールを使わずに、Rust の関数を WASM として書き出し、
//! JavaScript（Node.js）から呼び出します。境界で何が起きているのかを、自分の手で確かめるのが目的です。
//!
//! - ex01: 数値だけをやり取りする関数（WASM が直接扱えるのは数値だけ）
//! - ex02: 線形メモリを通して、文字列や配列をやり取りする
//! - ex03: ホスト（JavaScript）の関数を import して呼ぶ
//!
//! 先に `README.md` と `docs/fundamentals/15_webassembly.md` を読んでください。
//!
//! ## 2つの動かし方
//!
//! ```text
//! # 1. ネイティブでテストする（いつもどおり）
//! cargo test -p s17_wasm
//!
//! # 2. WASM にビルドして、Node.js から呼ぶ
//! cargo build -p s17_wasm --target wasm32-unknown-unknown --release
//! node stages/s17_wasm/host/run.mjs
//! ```
//!
//! ## なぜ `todo!()` ではなく仮の値が書いてあるのか
//!
//! 他の Stage と違い、`extern "C"` の関数には `todo!()` ではなく、`0` や `null` などの仮の値を書いてあります。
//! **`extern "C"` の関数の中で panic すると、テストのプロセス全体が強制終了（abort）する**からです。
//! C の ABI には「panic を呼び出し元に伝える」仕組みがないので、Rust は panic が境界を越えようとした時点でプロセスを止めます。
//! `todo!()` のままだと、1つ目のテストでプロセスが止まり、どのテストが通ったのか分からなくなってしまいます。
//! （Stage 13 の `compare_i32` と同じ理由です。）
//!
//! 同じ理由で、**あなたの実装も `extern "C"` の関数の中で panic しないように**書いてください。
//! 例えば、`a + b` でオーバーフローしたり、`unwrap()` が失敗したりすると、テストがまとめて止まります。
//!
//! **この Stage のルール**: Stage 13 と同じく、`unsafe` ブロックの直前には
//! `// SAFETY: ...` で「なぜ安全なのか」を書くこと。

// 未実装の状態で出る警告を抑えています。ステージを解き終えたらこの行を消して、警告が出ないか確認しましょう。
#![allow(unused_variables, unused_imports, dead_code)]
#![warn(clippy::undocumented_unsafe_blocks)]

pub mod ex01_numbers;
pub mod ex02_memory;
pub mod ex03_imports;
