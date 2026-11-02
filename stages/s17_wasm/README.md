# Stage 17: WebAssembly を素手で触る — 解説と設計判断

> **前提知識**: [15 WebAssembly の仕組み](../../docs/fundamentals/15_webassembly.md)（**必読**）/ [03 スタックとヒープ](../../docs/fundamentals/03_memory.md) / [04 ポインタとメモリ安全性](../../docs/fundamentals/04_pointers.md) / [Stage 13: unsafe と FFI](../s13_unsafe_ffi/README.md)

Rust で WASM を作るときは、普通は `wasm-bindgen` や `wasm-pack` というツールを使います。
これらを使うと、Rust の `String` をそのまま JavaScript に渡せるように**見えます**。
でも実際には、WASM の関数は**数値しか受け取れません**。ツールが裏で、たくさんの仕事をしているのです。

この Stage では、そのツールを使わずに、Rust の関数を WASM として書き出し、Node.js から直接呼び出します。
ツールが何を隠しているのかを、自分の手で確かめるのが目的です。最後に、ツールが自動で生成しているものを答え合わせします。

---

## 0. 準備と動かし方

```bash
# WASM 向けにビルドするための準備（1回だけ）
rustup target add wasm32-unknown-unknown

# 1. ネイティブでテストする（いつもどおり）
cargo test -p s17_wasm

# 2. WASM にビルドして、Node.js から呼ぶ
cargo build -p s17_wasm --target wasm32-unknown-unknown --release
node stages/s17_wasm/host/run.mjs
```

Node.js が必要です（`node --version` で確認できます）。

| ファイル | 内容 | いつ実行するか |
|---|---|---|
| `host/handmade.mjs` | WASM をバイト列から手で組み立てて動かす | 最初に（CS基礎 15 と一緒に） |
| `host/run.mjs` | ビルドした WASM の関数を、全部呼んでみる | 課題を解く前と、解いた後 |
| `host/bench.mjs` | JavaScript と WASM の速度を比べる | ex01 と ex02 を解いた後 |
| `cargo run -p fundamentals --example wasm_inspect` | .wasm の中身（セクション、関数の大きさ）を表示する | ビルドした後いつでも |

`target` に `wasm32-unknown-unknown` を指定すると、「OS のない（unknown）環境で動く、32ビットの WASM」向けにコンパイルされます。
CI ではネイティブのテストと lint だけを実行しています。WASM へのビルドは手元で試してください。

---

## 1. 実験 1: 未実装のまま呼んでみる

課題を解く前に、一度ビルドして `run.mjs` を実行してみてください。

```
読み込んだ: target/wasm32-unknown-unknown/release/s17_wasm.wasm (17766 バイト)
export されているもの: memory, add, alloc, celsius_to_fahrenheit, count_chars, dealloc, fib, fizzbuzz_log, greet, is_prime, panic_demo, roll_dice, sum_i32, to_upper_in_place, __data_end, __heap_base
線形メモリの大きさ: 1114112 バイト (17 ページ)

== WASM の中で panic すると ==
panic_demo() → RuntimeError: unreachable

== ex01: 数値だけをやり取りする ==
add(2, 3) = 0 (number)
...
== ex02: 線形メモリを通して文字列を渡す ==
count_chars("Rustは楽しい🦀") → Error: alloc が 0 (null) を返しました。alloc は実装しましたか？
```

未実装の関数は、仮の値（`0` や null）を返すようにしてあります（理由は下の「panic と `extern "C"`」で説明します）。この出力から、3つのことが分かります。

- **`#[unsafe(no_mangle)]` を付けた関数が、名前のとおりに export されている**。付けていない関数（`host::log` など）は見えません
- `memory` も export されている。これが**線形メモリ**です。JavaScript から中身を読み書きできます
- **Rust の panic は、JavaScript からは `RuntimeError: unreachable` に見える**（`panic_demo` は、わざと panic する関数です）

最後の点を詳しく見ます。`wasm32-unknown-unknown` では、panic は「スタックを巻き戻して後片付けする」のではなく、その場で実行を止めます（abort）。
WASM には `unreachable` という「ここに来たら即座に止まる」命令があり、panic はそれに変換されます。これを**トラップ**と呼びます。
トラップは JavaScript の例外になるので、JavaScript 側は `try/catch` で受け止められます。ただし、**panic のメッセージ（"WASM の中で panic しました"）は伝わりません**。ホストに伝える手段（ex03 の `host_log`）を用意していないからです。

### panic と `extern "C"`: なぜ `todo!()` を使っていないのか

他の Stage では、未実装の関数に `todo!()` を書いていました。この Stage では、`extern "C"` の関数に `todo!()` を書いていません。
**`extern "C"` の関数の中で panic すると、ネイティブのテストでも、テストのプロセス全体が強制終了（abort）する**からです。

```
thread caused non-unwinding panic. aborting.
error: test failed, to rerun pass `-p s17_wasm --lib`

Caused by:
  process didn't exit successfully: `.../s17_wasm-6703bf19720e430c` (signal: 6, SIGABRT: process abort signal)
```

（最初に `todo!()` で作ったときに、実際に起きたことです。1つ目のテストでプロセスが止まり、どのテストが通ったのか分からなくなりました。）

C の ABI には「panic（Rust の例外のようなもの）を、呼び出し元に伝える」仕組みがありません。C の関数は、自分が呼んだ関数から panic が飛んでくるとは想定していないからです。
そこで Rust は、panic が `extern "C"` の境界を越えようとした時点で、プロセスを止めます。
（境界を越えて panic を伝えたい場合のために `extern "C-unwind"` という ABI もありますが、呼び出し元がそれに対応している必要があります。）

**だから、あなたの実装も `extern "C"` の関数の中で panic しないように書いてください。**
`a + b` のオーバーフロー（debug ビルドでは panic します。CS基礎 01）や、`unwrap()` の失敗も panic です。`add` で `wrapping_add` を求めているのはこのためです。

---

## 2. なぜ `extern "C"` と `#[unsafe(no_mangle)]` が必要なのか

### 名前の書き換え（マングリング）

Rust のコンパイラは、関数名を**そのままの名前では出力しません**。
同じ名前の関数が別のモジュールにあっても区別できるように、モジュールの名前やハッシュ値を混ぜた名前にします。これを**マングリング**（name mangling）と呼びます。

この Stage のコードをネイティブでビルドし、`nm` コマンド（バイナリの中の関数名を一覧するコマンド）で見ると、こうなっていました。

```
T _fib                                                     ← #[unsafe(no_mangle)] あり
T __ZN8s17_wasm12ex03_imports4host3log17hb2faa9bdda1fee08E ← なし（host::log）
```

`s17_wasm::ex03_imports::host::log` という名前が、長さの数字と一緒に埋め込まれています。末尾の `17hb2fa...` はハッシュ値です。
（macOS では、関数名の先頭に `_` が1つ付きます。）

JavaScript から `wasm.fib(10)` と呼ぶには、名前が `fib` のままでなければ困ります。`#[unsafe(no_mangle)]` は「この関数の名前を書き換えないで」という指示です。

`unsafe(...)` で囲む書き方は Rust 2024 からです。理由は、**同じ名前の関数が2つあると、リンクの段階で区別できなくなる**からです。
例えば C の標準ライブラリの `abs` と同じ名前で `#[unsafe(no_mangle)]` を付けると、どちらが呼ばれるか分からなくなり、未定義動作になりえます。名前の衝突がないことは、書いた人が保証しなければなりません。

### 受け渡しの約束（ABI）

**ABI**（Application Binary Interface）とは、「関数を呼ぶときに、引数をどこに置き、戻り値をどこで受け取るか」という機械語レベルの約束です。
（API がソースコードの約束なのに対して、ABI はコンパイルされた後の約束です。）

- Rust 同士の呼び出しの ABI は、**安定していません**。コンパイラのバージョンが変わると、変わる可能性があります。そのほうが最適化しやすいからです
- `extern "C"` を付けると、**C 言語の ABI** を使います。C の ABI は何十年も変わっていないので、他の言語との共通語として使われています

WASM の場合、C の ABI では「`i32` は `i32` のまま、`u64` は `i64` に、ポインタは `i32` に」という形で、WASM の4つの型に割り当てられます。

---

## 3. 境界を越えると、型はどう変わるのか

`run.mjs` を、課題を解いた後に実行した結果です。

```
add(2, 3) = 5 (number)
add(2147483647, 1) = -2147483648 (number)
add(2.9, 0)         = 2 (number)
fib(90) = 2880067194370816120n (BigInt)
is_prime(7919) = 1 (number)
celsius_to_fahrenheit(36.5) = 97.7 (number)
```

| Rust の型 | WASM の型 | JavaScript で見える値 | 注意 |
|---|---|---|---|
| `i32` | `i32` | `number` | `2.9` を渡すと `2` に切り捨てられる |
| `u32` | `i32` | `number` | **2³¹ 以上の値は、負の数に見える**（CS基礎 01 の2の補数） |
| `bool` | `i32` | `number`（`1` / `0`） | `true` ではなく `1` が返ってくる |
| `u64` / `i64` | `i64` | **`BigInt`** | `number`（f64）では 2⁵³ を超える整数を正確に表せないため（CS基礎 02） |
| `f64` | `f64` | `number` | そのまま |
| `*const u8`, `usize` | `i32` | `number` | wasm32 では、ポインタも `usize` も 32 ビット |

WASM の `i32` には「符号付き」「符号なし」の区別がありません。区別は命令の側にあります（CS基礎 15）。
JavaScript の API は、`i32` を**符号付きとして**解釈して `number` にします。だから Rust で `u32` のつもりで返した値でも、大きいと負の数に見えます。
`u32::MAX` を返す関数を作って確かめた結果です。

```
big() = -1  (u32::MAX)
big() >>> 0 = 4294967295
```

ポインタを JavaScript 側で扱うときに `ptr >>> 0`（符号なしとして解釈し直すイディオム）と書かれていることがあるのは、このためです。

---

## 4. 文字列はどう渡すのか — 線形メモリ

WASM の関数は数値しか受け取れないので、文字列は次のように渡します（ex02）。

```
JavaScript                                 WASM の線形メモリ（ただのバイト配列）
                                           0                                    1179648
1. ptr = wasm.alloc(5)              ──→   [ スタック | データ | ヒープ  ? ? ? ? ?  ...]
   （Rust が場所を確保して、                                         ↑ptr
     その位置の数値を返す）
2. new Uint8Array(memory.buffer)    ──→   [ スタック | データ | ヒープ  h e l l o  ...]
     .set(bytes, ptr)
3. wasm.count_chars(ptr, 5)         ──→   Rust が ptr から 5 バイトを読んで、5 を返す
4. wasm.dealloc(ptr, 5)             ──→   領域を返す
```

**ポインタとは、このバイト配列の何番目かを表す数値**です。CS基礎 04 で「ポインタはアドレスを持つ変数」と説明しましたが、WASM ではそのアドレスが、線形メモリの先頭からのオフセットになります。

### 落とし穴: メモリが伸びると、古い buffer は使えなくなる

`run.mjs` の `putString` には、こういうコメントがあります。

```js
const ptr = wasm.alloc(encoded.length);
// 注意: alloc の中でメモリが拡張されると、古い memory.buffer は使えなくなる。
// だから、Uint8Array は alloc を呼んだ「後」に作り直す。
new Uint8Array(memory.buffer).set(encoded, ptr);
```

本当にそうなるか、確かめた結果です。

```
alloc 前: 1114112 バイト
alloc(5) = 1114120
alloc 後: 1179648 バイト
古い buffer の byteLength: 0  ← 0 になった (detached)
```

最初の `alloc` で、Rust のアロケータがメモリを 1 ページ（65536 バイト）伸ばしました。
すると、伸ばす前に作った `memory.buffer` は**切り離され（detached）**、長さが 0 になります。古い buffer に書き込んでも、WASM 側には届きません。

これは、Stage 02 で学んだ「**Vec に push すると中身が移動するので、借用中は push できない**」と同じ構造の問題です（CS基礎 06）。
Rust では借用チェッカーがこれを防いでくれましたが、JavaScript との境界では、**誰も防いでくれません**。

---

## 5. ⭐ 境界を越えると、所有権が消える

ex02 で一番考えてほしいのがここです。

Rust の中では、「このメモリを解放するのは誰か」をコンパイラが追跡してくれます（Stage 02）。
ところが、`alloc` がポインタを返した瞬間、そのメモリは**Rust の所有権の仕組みの外**に出ます。JavaScript にとって、ポインタはただの数値です。

| 起こりうるミス | 何が起きるか |
|---|---|
| JavaScript が `dealloc` を呼び忘れる | メモリリーク。ページを読み込み直すまで、メモリが増え続ける |
| 同じポインタを 2 回 `dealloc` する | 二重解放。アロケータの管理情報が壊れる |
| `dealloc` した後に、そのポインタを使う | 解放後の使用。別のデータを読み書きしてしまう |
| `alloc(5)` で確保したのに `dealloc(ptr, 6)` と呼ぶ | 確保したときと違う大きさで解放する。未定義動作 |

これは CS基礎 04 の表にあった、C 言語のバグそのものです。**境界を越えた瞬間に、Rust は C と同じ世界に戻る**のです。

だから、ポインタを受け取る関数は全て `unsafe fn` にして、`# Safety` に「呼ぶ側が守るべき約束」を書いています。
clippy にも `not_unsafe_ptr_arg_deref` という lint があり、「普通の関数が、引数の生ポインタの中身を読んでいる」とエラーにします。ポインタの正しさは関数の中では確かめられないので、呼ぶ側に約束を求める必要があるからです。

### WASM の中の未定義動作は、どこまで壊すのか

WASM のサンドボックスは、**ホスト（ブラウザや Node.js）を守ります**。WASM の中で二重解放が起きても、ブラウザのメモリが壊れることはありません。
線形メモリの範囲外を読もうとすると、トラップになります。

ただし、**WASM の線形メモリの中は、普通に壊れます**。二重解放でアロケータが壊れれば、その後の `alloc` が同じ場所を2回返して、データが混ざるかもしれません。
「サンドボックスがあるから安全」は、「ホストが安全」という意味であって、「あなたのプログラムが正しく動く」という意味ではありません。

### 2つの値を返したいとき

`greet` は、新しく作った文字列の**ポインタと長さ**の2つを返す必要があります。
しかし `extern "C"` の関数が返せる値は1つだけです（WASM 自体は複数の値を返せますが、C の ABI はそうなっていません）。

そこで、呼ぶ側に「長さを書き込む場所」を用意してもらい、そのポインタ（`out_len`）を引数で受け取る形にしています。
C 言語の API でよく見かける、**出力引数**（out parameter）というパターンです。

```js
const outLenPtr = wasm.alloc(4);  // usize は wasm32 では 4 バイト
const ptr = wasm.greet(namePtr, nameLen, outLenPtr);
const outLen = new DataView(memory.buffer).getUint32(outLenPtr, true);  // true = リトルエンディアン
```

---

## 6. ホストの関数を呼ぶ — WASM は何も持っていない

ex03 では、ホストから `host_log` と `host_random` を受け取ります。

```rust
unsafe extern "C" {
    fn host_log(ptr: *const u8, len: usize);
    fn host_random() -> f64;
}
```

```js
const imports = {
  env: {
    host_log(ptr, len) { /* メモリから文字列を読んで console.log */ },
    host_random() { return Math.random(); },
  },
};
const { instance } = await WebAssembly.instantiate(bytes, imports);
```

`wasm32-unknown-unknown` では、宣言しただけの外部関数は `env` という名前のモジュールから import されます。
`wasm_inspect` で見ると、解答した後のモジュールは、確かにこの2つだけを import しています。

```
   2 import           34  env.host_log (関数), env.host_random (関数)
```

**WASM のモジュールが外の世界に触れる手段は、この import だけ**です。
ファイルも、ネットワークも、時刻も、乱数の種も、ホストが渡さなければ手に入りません。`println!` も、そのままでは何も表示されません。
これは CS基礎 13 で学んだ「普通のプログラムは、システムコールで OS に何でも頼める」のとは正反対の設計です（CS基礎 15）。

### テストのために、ホストを差し替える

`host_log` は WASM として動くときにしか存在しないので、ネイティブで `cargo test` するときには使えません。
そこで `#[cfg(target_arch = "wasm32")]` で、ビルドする対象ごとに実装を切り替えています。

- wasm32 のとき: 本物の `host_log` を呼ぶ
- ネイティブのとき: ログを `Vec` に記録するモック。乱数は、テストで決めた値を順番に返す

これは Stage 09 の「依存の注入」と同じ考え方です。外の世界との境界を1か所（`host` モジュール）に集めておくと、テストで差し替えられます。
乱数を決まった値に差し替えられるので、`roll_dice` のテストも毎回同じ結果になります（Stage 10 で、テストが非決定的になる問題を扱いました）。

---

## 7. 速さについて — 実験 3

ex01 と ex02 を解いたら、`bench.mjs` を実行してみてください。この PC（Apple M4、Node.js v24）での結果です。

```
== 1. add を 1000 万回呼ぶ（計算はほぼゼロ。境界を越えるコストが目立つ）==
JavaScript の add                        3.2 ms
WASM の add (毎回 境界を越える)                 15.5 ms

== 2. is_prime を 1〜300 万で呼ぶ（1回あたりの計算が多い）==
JavaScript の is_prime                 305.4 ms
WASM の is_prime                       192.6 ms

== 3. 約 10MB の文字列の文字数を数える（データのコピーが必要）==
JavaScript だけで数える                      24.3 ms
WASM で数える (UTF-8 変換+コピー込み)             15.7 ms
  └ そのうち UTF-8 への変換とコピー: 9.7 ms
```

- **1**: 中身が空に近い関数を何度も呼ぶと、WASM のほうが**約5倍遅い**。JavaScript の JIT は、JavaScript の関数をループの中に埋め込める（インライン化。CS基礎 10）けれど、WASM の関数の呼び出しは境界を越えるからです
- **2**: 1回の呼び出しの中で計算が多いと、WASM のほうが**約1.6倍速い**
- **3**: 速くはなったけれど、**時間の6割以上は、文字列を UTF-8 に変換してコピーするのにかかっている**

つまり、WASM が効くのは「**境界を越える回数が少なく、1回の中でたくさん計算する**」場合です。詳しくは CS基礎 15 で扱います。

---

## 8. 答え合わせ: wasm-bindgen は何をしてくれているのか

ここまで手で書いてきたことを、`wasm-bindgen` は自動で生成します。

```rust
// wasm-bindgen を使うと、こう書ける（この Stage では依存を追加していないので、動かせません）
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}
```

この1つの関数のために、`wasm-bindgen` は次のようなものを生成します。

| この Stage で手で書いたもの | wasm-bindgen が生成するもの |
|---|---|
| `alloc` / `dealloc` | 文字列を渡すための、確保と解放の関数を export する |
| `run.mjs` の `putString`（TextEncoder で UTF-8 にしてコピー） | JavaScript の「グルーコード」（接着剤のコード）の中で、同じことをする |
| `alloc` の後に `memory.buffer` を作り直す | グルーコードが、メモリの view を必要に応じて作り直す |
| `greet` の出力引数 `out_len` | 戻り値の (ポインタ, 長さ) を受け取る仕組みを、自動で作る |
| `readString` してから `dealloc` | JavaScript の文字列に変換してから、WASM 側のメモリを解放する |
| `#[unsafe(no_mangle)] extern "C"` | `#[wasm_bindgen]` マクロ（Stage 12）が、その形のラッパー関数を作る |
| `host_log` の宣言と `imports` の組み立て | `#[wasm_bindgen] extern "C" { fn alert(s: &str); }` と書くだけで、両側を生成する |

仕組みはこうです。`#[wasm_bindgen]` マクロは、`extern "C"` のラッパー関数を作り、さらに「この関数はどんな型を受け取るか」という情報を .wasm の中に埋め込みます。
それを `wasm-bindgen` というコマンドが読み取って、JavaScript のグルーコードを書き出します。`wasm-pack` は、そのビルドからパッケージにまとめるまでを一度にやってくれるツールです。

**便利なツールは、境界を消してくれるわけではありません。境界を越える手間を、見えないところで代わりにやってくれているだけです。**
だから、ツールを使っていても、次のことは変わりません。

- 文字列を渡すたびに、コピーが起きる（実験 3 のコスト）
- 細かい関数を何度も呼ぶと遅い（実験 1 のコスト）
- 渡した Rust のオブジェクトを JavaScript 側で解放し忘れると、リークする（wasm-bindgen では `free()` を呼ぶ必要がある場面があります）

---

## 9. ⭐ 選球眼: WASM を使うべき場面、使うべきでない場面

### 使うと効果が大きい場面

| 場面 | 理由 | 例 |
|---|---|---|
| ブラウザの中で重い計算をする | 計算が多く、境界を越える回数が少ない | 画像・動画・音声の処理、圧縮、暗号、物理演算、CAD |
| 既存の Rust / C / C++ の資産をブラウザで使う | 書き直さずに済む | Figma（C++ のエンジンを WASM で動かしている） |
| 信頼できないコードを安全に動かす（プラグイン） | import したもの以外に触れないサンドボックス | Envoy のプラグイン（proxy-wasm）、Shopify Functions |
| エッジでの実行 | 起動が速く、メモリが小さい | Cloudflare Workers、Fastly Compute |

### 使わないほうがよい場面

| 場面 | 理由 |
|---|---|
| DOM の操作が中心の UI | DOM は JavaScript からしか触れない。毎回境界を越えるので、遅くなりやすい |
| 計算がそれほど重くない | JavaScript の JIT は十分速い（実験 1・2）。コピーのコストで逆に遅くなることもある |
| チームが Rust を書けない | ビルドの仕組み、デバッグ、境界の設計と、覚えることが増える |
| デバッグのしやすさが重要 | ソースマップやデバッガの対応は、JavaScript ほど成熟していない |

### よく出てくる名前の位置づけ

| 名前 | 何をするものか |
|---|---|
| `wasm-bindgen` | Rust と JavaScript の境界のコード（グルーコード）を生成する |
| `wasm-pack` | ビルドから npm パッケージにまとめるまでを行うツール |
| `wasmtime` | ブラウザの外で WASM を動かすランタイム（Bytecode Alliance が開発） |
| WASI | ブラウザの外の WASM が、ファイルや時刻などを使うための標準的なインターフェース |
| コンポーネントモデル | 言語の違う WASM のモジュール同士を、文字列や構造体などの型でつなぐ仕組み（WIT という言語でインターフェースを書く） |

Rust のターゲットにも、用途ごとに `wasm32-unknown-unknown`（この Stage。ホストが全部用意する）、`wasm32-wasip1` / `wasm32-wasip2`（WASI を使う）などがあります。

> **判断の軸**: 「**境界を越える回数**」と「**1回の中での計算量**」の比です。
> そして、それを推測ではなく計測すること（選球眼 05）。

---

## 設計の問い

`answers/s17.md` に答えてください。

1. `alloc` で確保したメモリを、JavaScript 側が `dealloc` し忘れないようにするには、どんな工夫ができますか？
   JavaScript の `FinalizationRegistry`（オブジェクトが GC で回収されたときに処理を呼ぶ仕組み）を調べて、それに頼ることの長所と危うさを考えてください。
2. `greet` は出力引数で長さを返しました。他にも、「ポインタと長さを、メモリ上に並べて書き、その場所を返す」「長さを返す関数を別に用意する」などの設計が考えられます。それぞれの長所と短所を比べてください。
3. 10 万個の点の座標を JavaScript から WASM に渡し、WASM 側でソートして返したいとします。
   `sort_point(x, y)` を 10 万回呼ぶ設計と、全部を一度にメモリに書いて1回だけ呼ぶ設計があります。速さとメモリの両面から比べてください。
   （ヒント: 実験 1 と 3）
4. 「Web アプリを速くしたいので、全部 Rust + WASM で書き直そう」と同僚が提案しました。どんな質問をしますか？ この Stage の実験結果と、選球眼 05 の考え方を使って答えてください。
