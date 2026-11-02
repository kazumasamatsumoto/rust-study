# ツアー03: `String` と UTF-8

> **出典**: rustc 1.95.0 の rust-src
> - `library/alloc/src/string.rs`
> - `library/core/src/str/converts.rs`
> - `library/core/src/str/validations.rs`
> - `library/core/src/str/mod.rs`
> - `library/core/src/char/methods.rs`, `library/core/src/char/mod.rs`
>
> **つながるところ**: Stage 01（文字列）、CS基礎 01（2進数）、CS基礎 05（UTF-8）、ツアー02（`Vec`）

CS基礎 05 では、「あ」が `E3 81 82` の3バイトになることを実験で確かめました。
このツアーでは、「`String` は何者なのか」「UTF-8 であることをどうやって保証しているのか」「文字をどうやってバイト列にしているのか」を読みます。

---

## 1. `String` は `Vec<u8>` の newtype

`alloc/src/string.rs` で `pub struct String` を検索してください。

```rust
#[derive(PartialEq, PartialOrd, Eq, Ord)]
#[stable(feature = "rust1", since = "1.0.0")]
#[lang = "String"]
pub struct String {
    vec: Vec<u8>,
}
```

**`String` の中身は `Vec<u8>`（バイトの列）だけです。**
既存の型を1つだけ包んで、新しい意味を持たせた型を **newtype** と呼びます（Stage 06 の `Money(i64)` と同じパターンです）。

`String` が `Vec<u8>` に付け加えているのは、たった1つの約束です。

> **中身のバイト列は、常に正しい UTF-8 である。**

この約束（不変条件）を守るために、フィールド `vec` は非公開になっています。外から勝手にバイトを書き込めてしまうと、UTF-8 として壊れたバイト列が入りかねないからです。
Stage 03 の `Account` で「残高はマイナスにならない」を守るためにフィールドを非公開にしたのと、全く同じ設計です。

### 多くのメソッドは `Vec` に任せているだけ

```rust
pub const fn len(&self) -> usize {
    self.vec.len()
}
```

```rust
pub fn push_str(&mut self, string: &str) {
    self.vec.extend_from_slice(string.as_bytes())
}
```

- **`len()` が文字数ではなくバイト数を返す理由**が、ここにあります。中身の `Vec<u8>` の長さを、そのまま返しているだけだからです（CS基礎 05）
- `push_str` は、`&str` のバイト列を末尾にコピーしているだけです。`&str` はすでに正しい UTF-8 だと分かっているので、検査する必要がありません。正しい UTF-8 同士をつなげたものは、やはり正しい UTF-8 です
- 容量の伸ばし方は、ツアー02で読んだ `Vec` の仕組みそのものです

実験で確かめてみましょう。

```
String            : 24
Option<String>    : 24
```

`String` が 24 バイトなのは、中身が `Vec<u8>`（24 バイト）だからです。`Option<String>` が 24 バイトのままなのも、ツアー02で見た `Cap` のニッチ最適化のおかげです。

---

## 2. ⭐ `String::from_utf8`: 約束を守るための検査

バイト列から `String` を作るときには、それが正しい UTF-8 かどうかを検査する必要があります。

```rust
pub fn from_utf8(vec: Vec<u8>) -> Result<String, FromUtf8Error> {
    match str::from_utf8(&vec) {
        Ok(..) => Ok(String { vec }),
        Err(e) => Err(FromUtf8Error { bytes: vec, error: e }),
    }
}
```

1. `str::from_utf8(&vec)` で検査する
2. 正しければ、**`vec` をそのまま包んで** `String` にする。コピーは発生しません
3. 正しくなければ、エラーを返す

エラーの型 `FromUtf8Error` を見てください。

```rust
pub struct FromUtf8Error {
    bytes: Vec<u8>,
    error: Utf8Error,
}
```

**エラーの中に、渡したバイト列 `bytes` がそのまま入っています。**
`from_utf8` は `Vec<u8>` の所有権を受け取ってしまうので、失敗したときにバイト列を捨ててしまうと、呼び出し側はそれを取り戻せません。そこで、エラーに入れて返しています。`into_bytes()` で取り出せます。

```
from_utf8 の失敗: incomplete utf-8 byte sequence from index 0  / 返ってきたバイト列: [E3, 81]
```

（`E3 81` は「あ」の3バイトのうち、最初の2バイトだけです。途中で切れているので失敗しました。）

**所有権を受け取る関数が失敗したときは、受け取ったものを返す。** これは API 設計の良い習慣です（選球眼 04 で詳しく扱います）。

### `str::from_utf8` と `from_utf8_unchecked`

`core/src/str/converts.rs` を開いてください。

```rust
pub const fn from_utf8(v: &[u8]) -> Result<&str, Utf8Error> {
    // FIXME(const-hack): This should use `?` again, once it's `const`
    match run_utf8_validation(v) {
        Ok(_) => {
            // SAFETY: validation succeeded.
            Ok(unsafe { from_utf8_unchecked(v) })
        }
        Err(err) => Err(err),
    }
}
```

```rust
pub const unsafe fn from_utf8_unchecked(v: &[u8]) -> &str {
    // SAFETY: the caller must guarantee that the bytes `v` are valid UTF-8.
    // Also relies on `&str` and `&[u8]` having the same layout.
    unsafe { mem::transmute(v) }
}
```

`from_utf8_unchecked` は `mem::transmute`（ビット列をそのまま別の型として読み替える関数）を使っているだけです。
コメントに「`&str` と `&[u8]` は同じレイアウトである」とある通り、**`&str` と `&[u8]` は、メモリ上は全く同じもの**（ポインタ + 長さの 16 バイト）です。違いは「中身が UTF-8 だと保証されているかどうか」という、**型が持っている約束だけ**です。

`from_utf8_unchecked` は、検査をせずに「正しい UTF-8 だ」と言い張る関数なので `unsafe` です。正しくないバイト列を渡すと、`String` の約束が破れてしまいます。
`from_utf8` は「検査が成功した」ことを根拠に（`// SAFETY: validation succeeded.`）、それを安全に呼んでいます。

---

## 3. ⭐ UTF-8 の検査: `run_utf8_validation`

`core/src/str/validations.rs` の `run_utf8_validation` を読みます。少し長いので、2つの部分に分けて見ていきます。

### 3-1. 先頭バイトで、文字のバイト数が分かる

CS基礎 05 で学んだ通り、UTF-8 では先頭のバイトを見れば、その文字が何バイトかが分かります。

| 先頭バイト（2進数） | 範囲（16進数） | バイト数 |
|---|---|---|
| `0xxxxxxx` | `00`〜`7F` | 1（ASCII） |
| `110xxxxx` | `C2`〜`DF` | 2 |
| `1110xxxx` | `E0`〜`EF` | 3 |
| `11110xxx` | `F0`〜`F4` | 4 |
| `10xxxxxx` | `80`〜`BF` | **2バイト目以降**（継続バイト） |

```rust
        let first = v[index];
        if first >= 128 {
            let w = utf8_char_width(first);
            // 2-byte encoding is for codepoints  \u{0080} to  \u{07ff}
            //        first  C2 80        last DF BF
            // 3-byte encoding is for codepoints  \u{0800} to  \u{ffff}
            //        first  E0 A0 80     last EF BF BF
            //   excluding surrogates codepoints  \u{d800} to  \u{dfff}
            //               ED A0 80 to       ED BF BF
            // 4-byte encoding is for codepoints \u{10000} to \u{10ffff}
            //        first  F0 90 80 80  last F4 8F BF BF
            //
            // Use the UTF-8 syntax from the RFC
            //
            // https://tools.ietf.org/html/rfc3629
            // ...
            match w {
                2 => {
                    if next!() as i8 >= -64 {
                        err!(Some(1))
                    }
                }
                3 => {
                    match (first, next!()) {
                        (0xE0, 0xA0..=0xBF)
                        | (0xE1..=0xEC, 0x80..=0xBF)
                        | (0xED, 0x80..=0x9F)
                        | (0xEE..=0xEF, 0x80..=0xBF) => {}
                        _ => err!(Some(1)),
                    }
                    if next!() as i8 >= -64 {
                        err!(Some(2))
                    }
                }
                // 4 => { ... 同様 ... }
                _ => err!(Some(1)),
            }
            index += 1;
        }
```

`utf8_char_width` は、256 要素の表（`UTF8_CHAR_WIDTH`）を引いて、先頭バイトから文字のバイト数を返す関数です。

### `as i8 >= -64` は何を調べているのか

`next!() as i8 >= -64` という不思議な式が何度も出てきます。これは「次のバイトが継続バイト（`10xxxxxx`）**ではない**か」を調べています。CS基礎 01 の **2の補数**の知識で読み解けます。

- 継続バイトは `0x80`〜`0xBF`（`10000000`〜`10111111`）
- これを `i8`（符号付き）として読むと、2の補数なので **-128〜-65** になる
- つまり `as i8 >= -64` が成り立つなら、継続バイトではない → エラー

`0x80 <= b && b <= 0xBF` と書く代わりに、比較1回で済ませるための技です。

### なぜ 3 バイト文字で細かく範囲を調べているのか

コメントにある通り、正しい UTF-8 には、先頭バイトの形が合っているだけでは足りない制約があります。

- **最短の形で表すこと**: 例えば `/`（U+002F）は1バイトで表せますが、ビットの形の上では `E0 80 AF` のように3バイトでも表せてしまいます。これを許すと、「`/` を含まないか」という検査をすり抜けられてしまうため、**セキュリティ上の問題**になります（過去に実際の脆弱性の原因になりました）。そのため、`E0` の次は `A0` 以上でなければならない、といった制約があります
- **サロゲート（U+D800〜U+DFFF）を含まないこと**: この範囲は UTF-16 の内部で使う特別な番号で、文字ではありません。`ED` の次が `9F` 以下に制限されているのはこのためです

Rust の `char` は「Unicode スカラー値」（サロゲートを除いたコードポイント）と定義されていて、この検査と合っています。

### 3-2. ASCII の高速道路

英語のテキストのように ASCII が続く部分では、1バイトずつ調べるのは遅すぎます。そこで、こんな工夫がされています。

```rust
        } else {
            // Ascii case, try to skip forward quickly.
            // When the pointer is aligned, read 2 words of data per iteration
            // until we find a word containing a non-ascii byte.
            if align != usize::MAX && align.wrapping_sub(index).is_multiple_of(USIZE_BYTES) {
                let ptr = v.as_ptr();
                while index < blocks_end {
                    // SAFETY: since `align - index` and `ascii_block_size` are
                    // multiples of `USIZE_BYTES`, `block = ptr.add(index)` is
                    // always aligned with a `usize` so it's safe to dereference
                    // both `block` and `block.add(1)`.
                    unsafe {
                        let block = ptr.add(index) as *const usize;
                        // break if there is a nonascii byte
                        let zu = contains_nonascii(*block);
                        let zv = contains_nonascii(*block.add(1));
                        if zu || zv {
                            break;
                        }
                    }
                    index += ascii_block_size;
                }
                // ...
```

そして `contains_nonascii` は、こう定義されています。

```rust
const NONASCII_MASK: usize = usize::repeat_u8(0x80);

/// Returns `true` if any byte in the word `x` is nonascii (>= 128).
#[inline]
const fn contains_nonascii(x: usize) -> bool {
    (x & NONASCII_MASK) != 0
}
```

**8バイトを1つの `usize` としてまとめて読み、1回の計算で8バイト分を調べています。**

- ASCII のバイトは、最上位ビットが必ず 0（`0xxxxxxx`）
- `NONASCII_MASK` は `0x8080808080808080`（各バイトの最上位ビットだけが 1）
- `x & NONASCII_MASK` が 0 なら、8バイトすべてが ASCII

ループ1回で `usize` を2つ（16バイト）調べます。1バイトずつ調べるより、ずっと速くなります。
`usize` として読むには、アドレスが8の倍数でなければなりません（CS基礎 03 のアラインメント）。そのために、`align` の計算と SAFETY コメントがあります。

**ビット演算、2の補数、アラインメントといった CS基礎の知識が、標準ライブラリの性能を支えています。**

---

## 4. ⭐ `String::push`: 文字を UTF-8 のバイト列にする

```rust
pub fn push(&mut self, ch: char) {
    let len = self.len();
    let ch_len = ch.len_utf8();
    self.reserve(ch_len);

    // SAFETY: Just reserved capacity for at least the length needed to encode `ch`.
    unsafe {
        core::char::encode_utf8_raw_unchecked(ch as u32, self.vec.as_mut_ptr().add(self.len()));
        self.vec.set_len(len + ch_len);
    }
}
```

1. その文字が何バイトになるかを調べる（`len_utf8`）
2. その分の容量を確保する（`reserve`）
3. 末尾に UTF-8 のバイト列を書き込む（`encode_utf8_raw_unchecked`）
4. 長さを増やす（`set_len`）

ツアー02の `Vec::push_mut` と同じ形です。

### 何バイトになるか: `len_utf8`

`core/src/char/methods.rs` で `const fn len_utf8(code: u32)` を検索してください。

```rust
const fn len_utf8(code: u32) -> usize {
    match code {
        ..MAX_ONE_B => 1,
        ..MAX_TWO_B => 2,
        ..MAX_THREE_B => 3,
        _ => 4,
    }
}
```

定数は `core/src/char/mod.rs` にあります。

```rust
const TAG_CONT: u8 = 0b1000_0000;
const TAG_TWO_B: u8 = 0b1100_0000;
const TAG_THREE_B: u8 = 0b1110_0000;
const TAG_FOUR_B: u8 = 0b1111_0000;
const MAX_ONE_B: u32 = 0x80;
const MAX_TWO_B: u32 = 0x800;
const MAX_THREE_B: u32 = 0x10000;
```

CS基礎 05 の表（U+0080 未満は1バイト、U+0800 未満は2バイト、…）が、そのまま `match` になっています。`..MAX_ONE_B` は「`MAX_ONE_B` 未満」という範囲パターンです（Stage 01 の `grade` で使った `90..=100` の仲間です）。

### バイト列に変換する: `encode_utf8_raw_unchecked`

```rust
pub const unsafe fn encode_utf8_raw_unchecked(code: u32, dst: *mut u8) {
    let len = len_utf8(code);
    // SAFETY: The caller must guarantee that the buffer pointed to by `dst`
    // is at least `len` bytes long.
    unsafe {
        if len == 1 {
            *dst = code as u8;
            return;
        }

        let last1 = (code >> 0 & 0x3F) as u8 | TAG_CONT;
        let last2 = (code >> 6 & 0x3F) as u8 | TAG_CONT;
        let last3 = (code >> 12 & 0x3F) as u8 | TAG_CONT;
        let last4 = (code >> 18 & 0x3F) as u8 | TAG_FOUR_B;

        if len == 2 {
            *dst = last2 | TAG_TWO_B;
            *dst.add(1) = last1;
            return;
        }

        if len == 3 {
            *dst = last3 | TAG_THREE_B;
            *dst.add(1) = last2;
            *dst.add(2) = last1;
            return;
        }

        *dst = last4;
        *dst.add(1) = last3;
        *dst.add(2) = last2;
        *dst.add(3) = last1;
    }
}
```

使われている演算子を確認しておきます。

| 演算 | 意味 | 例 |
|---|---|---|
| `x >> 6` | 右に6ビットずらす（下位6ビットを捨てる） | `0b1100_0001 >> 6` = `0b11` |
| `x & 0x3F` | 下位6ビットだけを残す（`0x3F` = `0b111111`） | `0b1100_0001 & 0x3F` = `0b000001` |
| `x \| TAG_CONT` | 最上位ビットを 1 にする（`0b1000_0000` を重ねる） | `0b000001 \| 0b1000_0000` = `0b1000_0001` |

UTF-8 の継続バイトは `10xxxxxx` で、`x` の部分に 6 ビットずつデータが入ります。
そのため、コードポイントを **6 ビットずつ切り出して**（`>> 6` と `& 0x3F`）、**先頭に `10` を付けています**（`| TAG_CONT`）。

### 手で計算してみる: 「あ」（U+3042）

```
code = 0x3042 = 0011 0000 0100 0010 (2進数)

len_utf8: 0x800 ≤ 0x3042 < 0x10000 なので 3バイト

last1 = (0x3042 >> 0  & 0x3F) | 0x80
      = (下位6ビット 000010)  | 1000_0000  = 1000_0010 = 0x82

last2 = (0x3042 >> 6  & 0x3F) | 0x80
      = 0x3042 >> 6 は 1100_0001、その下位6ビット 000001
      = 000001 | 1000_0000                 = 1000_0001 = 0x81

last3 = (0x3042 >> 12 & 0x3F) | 0x80
      = 0x3042 >> 12 は 0011、その下位6ビット 000011
      = 000011 | 1000_0000                 = 1000_0011 = 0x83

先頭バイト = last3 | TAG_THREE_B
          = 1000_0011 | 1110_0000          = 1110_0011 = 0xE3

結果: E3 81 82
```

実験の結果と一致します。

```
String に 'あ' を push 後: len=3 cap=8 bytes=[E3, 81, 82]
```

（`cap=8` になっているのは、`Vec<u8>` の要素のサイズが 1 バイトなので、ツアー02の `min_non_zero_cap` により最小の容量が 8 になったからです。）

`last3` にはすでに `TAG_CONT`（`1000_0000`）が付いていますが、`TAG_THREE_B`（`1110_0000`）を重ねても結果は `1110_xxxx` になるので、問題ありません。4バイトの場合だけ `last4` に最初から `TAG_FOUR_B` を付けているのも、同じ理由です。

---

## 5. `is_char_boundary`: 文字の途中で切れない理由

CS基礎 05 では、`&s[0..2]` のように文字の途中でスライスすると panic することを見ました。その判定をしているのが `is_char_boundary` です（`core/src/str/mod.rs`）。

```rust
    pub const fn is_char_boundary(&self, index: usize) -> bool {
        // 0 is always ok.
        // ...
        if index == 0 {
            return true;
        }

        if index >= self.len() {
            // ...
            index == self.len()
        } else {
            self.as_bytes()[index].is_utf8_char_boundary()
        }
    }
```

そして `u8` の `is_utf8_char_boundary`（`core/src/num/mod.rs`）は、こうです。

```rust
    pub(crate) const fn is_utf8_char_boundary(self) -> bool {
        // This is bit magic equivalent to: b < 128 || b >= 192
        (self as i8) >= -0x40
    }
```

3節で見たのと同じ「`as i8` の技」です。**そのバイトが継続バイト（`10xxxxxx`）でなければ、文字の境界**です。
文字の途中かどうかを、前後を見ずに、そのバイト1つだけで判定できます。これも UTF-8 の設計の良いところでした（CS基礎 05）。

```
"aあ" のバイト列: 61 E3 81 82
is_char_boundary(0) = true    ← 'a' の先頭
is_char_boundary(1) = true    ← 'あ' の先頭 (E3)
is_char_boundary(2) = false   ← 81 は継続バイト
is_char_boundary(3) = false   ← 82 は継続バイト
is_char_boundary(4) = true    ← 末尾
```

---

## まとめ

| 学んだこと | ソースのどこに現れていたか |
|---|---|
| `String` は `Vec<u8>` | `pub struct String { vec: Vec<u8> }` |
| `len()` はバイト数 | `self.vec.len()` をそのまま返している |
| UTF-8 であることの保証 | フィールドを非公開にし、外から作る道を `from_utf8`（検査あり）に限っている |
| `&str` と `&[u8]` の違い | メモリ上は同じ。違いは型の約束だけ（`transmute`） |
| 検査の高速化 | ASCII の部分は8バイトずつまとめて調べる |
| 文字 → バイト列 | 6ビットずつ切り出して `10` を付ける |
| 文字の境界の判定 | `as i8 >= -64`（2の補数の技） |

---

## 問い（`answers/reading.md` へ）

1. 「é」（U+00E9）を、`encode_utf8_raw_unchecked` の手順に従って手でバイト列に変換してください。CS基礎 05 の実験結果（`C3 A9`）と一致しますか？
2. `String` に `as_mut_vec()` というメソッドがあるかを調べてください。あるなら、なぜそれは `unsafe` なのでしょうか？
3. `from_utf8` が失敗したときに `bytes` を返さない設計だったとしたら、呼び出し側でどんな不便が起きますか？
4. ASCII を8バイトずつ調べる工夫は、日本語ばかりのテキストでは効果がありますか？ 理由も答えてください。
5. 選球眼の観点から: `String` と `Vec<u8>` を別の型にせず、「中身が UTF-8 かどうかは使う人が気を付ける」という設計（C 言語の `char*` のように）にしていたら、何が起きたと思いますか？
