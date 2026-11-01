# CS基礎トラック — Rust を理解するための土台

Rust は、コンピュータの中で実際に何が起きているかを隠さない言語です。
そのため、他の言語では意識しなくて済んだ知識（ビット、メモリ、ポインタ、文字コード）が、そのまま言語の設計に現れます。
ここでは、Rust を理解するために必要な理論を、**手元で動かせる実験**と一緒に学びます。

## 進め方

1. ドキュメントを読む
2. 実験を実行して、自分の目で確かめる（`cargo run -p fundamentals --example <名前>`）
3. 実験のコードを書き換えて、予想どおりになるか試す
4. 確認問題に `answers/fundamentals.md` で答える

## 目次

| # | テーマ | 実験 | 関係するステージ |
|---|---|---|---|
| [01](01_integers.md) | 整数の表現とオーバーフロー | `integers` | s01 |
| [02](02_floats.md) | 浮動小数点数 | `floats` | s01, s05 |
| [03](03_memory.md) | メモリ: スタックとヒープ | `memory_layout` | s02 |
| [04](04_pointers.md) | ポインタと、メモリ安全性のバグ | `memory_layout` | s02, s07, s08 |
| [05](05_text_encoding.md) | 文字コードと UTF-8 | `utf8` | s01, s02 |
| [06](06_complexity.md) | 計算量とデータ構造 | `vec_growth` | s02, s05 |
| [07](07_dispatch.md) | 静的ディスパッチと動的ディスパッチ | `dispatch` | s06 |
| [08](08_concurrency.md) | プロセス・スレッド・データ競合 | `threads` | s10 |
| [09](09_io_async.md) | I/O モデルと非同期処理 | `io_latency` | s11 |
| [10](10_compilation.md) | コンパイルとリンク、最適化 | `optimization` | s09 |

速度を比べる実験（`dispatch`, `threads`, `optimization`, `io_latency`）は `--release` を付けて実行してください。
