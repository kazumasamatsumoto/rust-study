# ツアー04: `Iterator` とゼロコスト抽象化

> **出典**: rustc 1.95.0 の rust-src
> - `library/core/src/iter/traits/iterator.rs`
> - `library/core/src/iter/adapters/map.rs`
> - `library/core/src/iter/traits/accum.rs`
> - `library/core/src/iter/traits/collect.rs`
> - `library/alloc/src/vec/mod.rs`, `library/core/src/result.rs`
>
> **つながるところ**: Stage 05（イテレータ）、Stage 06（トレイト）、CS基礎 07（ディスパッチ）、CS基礎 10（最適化）、ツアー01（`Option::map`）

Stage 05 では、`for` ループの代わりに `.iter().map(...).filter(...).sum()` と書く練習をしました。
CS基礎 10 では、イテレータのコードが debug ビルドでは遅いのに、release ビルドでは手書きのループと同じ速さになることを見ました。
このツアーでは、その仕組みを読み解きます。

---

## 1. `Iterator` トレイト: 必須のメソッドは1つだけ

`core/src/iter/traits/iterator.rs` を開いてください。

```rust
#[rustc_const_unstable(feature = "const_iter", issue = "92476")]
pub const trait Iterator {
    /// The type of the elements being iterated over.
    #[rustc_diagnostic_item = "IteratorItem"]
    #[stable(feature = "rust1", since = "1.0.0")]
    type Item;
```

（`const trait` は、ツアー01で見た `[const]` と同じく、まだ不安定な機能の記法です。普通の `pub trait Iterator` と読み替えてください。）

`type Item;` は**関連型**です。「このイテレータが返す要素の型」を、実装する側が決めます。

そして、**実装しなければならないメソッドは `next` だけ**です。

```rust
    #[lang = "next"]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn next(&mut self) -> Option<Self::Item>;
```

本体（`{ ... }`）がないので、実装する側が必ず書く必要があります。それ以外の `map`, `filter`, `fold`, `sum`, `collect` などは、すべて**デフォルト実装**付きで提供されています。このトレイトの中には、70 以上のメソッドが定義されています。

Stage 06 で `Countdown` や `Fibonacci` に `next` だけを実装したら、`take`, `sum`, `collect` などが全部使えるようになりました。それは、このデフォルト実装のおかげです。
**「最小限のものを実装させ、残りは全部提供する」**。これはトレイトを設計するときの強力なパターンです。

`#[lang = "next"]` は、`for` ループの構文がこのメソッドを使うための目印です。`for x in iter { ... }` は、おおよそ次のように書き換えられます。

```rust
let mut it = IntoIterator::into_iter(iter);
loop {
    match it.next() {
        Some(x) => { /* ループの本体 */ }
        None => break,
    }
}
```

## 2. `fold`: すべての集計の土台

```rust
    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        let mut accum = init;
        while let Some(x) = self.next() {
            accum = f(accum, x);
        }
        accum
    }
```

`fold` は、「初期値から始めて、要素を1つずつ取り出し、関数で積み上げていく」メソッドです。デフォルト実装は、`next` を呼び続ける `while let` ループにすぎません。

## 3. ⭐ `map`: 「何もしない」アダプタ

`fn map<B, F>(self, f: F) -> Map<Self, F>` を検索してください。

```rust
    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        Map::new(self, f)
    }
```

`map` を呼んでも、**要素は1つも処理されません**。元のイテレータ（`self`）と関数（`f`）を、`Map` という構造体に詰めて返すだけです。

`core/src/iter/adapters/map.rs` を開いてください。

```rust
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Clone)]
pub struct Map<I, F> {
    // Used for `SplitWhitespace` and `SplitAsciiWhitespace` `as_str` methods
    pub(crate) iter: I,
    f: F,
}
```

`#[must_use]` のメッセージに注目してください。「**イテレータは怠惰（lazy）で、消費されるまで何もしない**」。
`v.iter().map(|x| println!("{x}"));` と書くと、何も表示されずに、この警告が出ます。`map` は処理の予約をしただけで、実行していないからです。

### 要素の処理は、`next` が呼ばれたときに初めて起きる

```rust
impl<B, I: Iterator, F> Iterator for Map<I, F>
where
    F: FnMut(I::Item) -> B,
{
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<B> {
        self.iter.next().map(&mut self.f)
    }
```

`Map` の `next` は、

1. 元のイテレータから次の要素を1つもらう（`self.iter.next()`）
2. それに関数を適用する（`.map(&mut self.f)`）

だけです。2. の `.map` は、**ツアー01で読んだ `Option::map`** です。`Some(x)` なら `Some(f(x))`、`None` なら `None`。

このように、要素は「1つずつ、必要になったときに」処理されます。これを**遅延評価**と呼びます。

- 途中で `take(3)` すれば、4つ目以降には関数が適用されない
- 無限に続くイテレータ（Stage 06 の `Fibonacci`）でも扱える
- 途中結果を格納する `Vec` を作らないので、メモリを使わない

### `Map` は `fold` を上書きしている

```rust
    fn fold<Acc, G>(self, init: Acc, g: G) -> Acc
    where
        G: FnMut(Acc, Self::Item) -> Acc,
    {
        self.iter.fold(init, map_fold(self.f, g))
    }
```

`Map` は、デフォルトの `fold`（`next` を繰り返し呼ぶ）を使わずに、**元のイテレータの `fold` に処理を丸ごと任せています**（`map_fold` は、`f` と `g` を1つの関数にまとめるヘルパーです）。
元のイテレータが、自分に合った速い `fold` を持っている場合（例えば、内部で `Vec` を順番に読むだけのループ）、`Map` を通してもその速さを活かせます。デフォルト実装で正しく動くようにしておき、**性能が大事なところだけを上書きする**。これもトレイト設計のパターンです。

## 4. `sum`: マクロで生成された実装

```rust
    fn sum<S>(self) -> S
    where
        Self: Sized,
        S: Sum<Self::Item>,
    {
        Sum::sum(self)
    }
```

`sum` 自体は、`Sum` トレイトに処理を任せているだけです。`Sum` は `core/src/iter/traits/accum.rs` にあります。

```rust
pub trait Sum<A = Self>: Sized {
    /// Takes an iterator and generates `Self` from the elements by "summing up"
    /// the items.
    #[stable(feature = "iter_arith_traits", since = "1.12.0")]
    fn sum<I: Iterator<Item = A>>(iter: I) -> Self;
}
```

**合計した結果の型（`Self`）が、合計の仕方を決める**設計になっています。そのため Stage 06 で `impl Sum for Money` を書くと、`Money` のイテレータに `.sum()` が使えるようになりました。

では、`i32` の `Sum` の実装はどこにあるのでしょうか。`impl Sum for i32` で検索しても見つかりません。[README](README.md) の「マクロで生成されているコード」で説明した通り、マクロで生成されているからです。

```rust
macro_rules! integer_sum_product {
    (@impls $zero:expr, $one:expr, #[$attr:meta], $($a:ty)*) => ($(
        #[$attr]
        impl Sum for $a {
            fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
                iter.fold(
                    $zero,
                    #[rustc_inherit_overflow_checks]
                    |a, b| a + b,
                )
            }
        }
        // ... Product や、参照版の Sum<&'a $a> も同様に生成する
```

```rust
integer_sum_product! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
```

- `$($a:ty)*` は「型をいくつでも」という意味です。`i8 i16 ... usize` の12個の型それぞれについて、`impl Sum for $a { ... }` が生成されます
- 中身は **`iter.fold(0, |a, b| a + b)`** です。2節の `fold` を使っています

### `#[rustc_inherit_overflow_checks]`

クロージャ `|a, b| a + b` に付いている属性に注目してください。
標準ライブラリは、通常は release 設定でコンパイルされて配布されています。そのままでは、標準ライブラリの中の `+` は、オーバーフローを検査しません。
この属性は、「この `+` のオーバーフロー検査は、**`sum` を呼んだ側のビルド設定に合わせる**」という意味です。

そのため、`.sum()` でも普通の `+` と同じように、**debug ビルドではオーバーフローで panic し、release ビルドではラップアラウンド**します（CS基礎 01）。
「`sum()` を使うと、オーバーフローの検査が消えてしまう」ということが起きないように、細かく配慮されています。

## 5. `collect`: 結果の型がすべてを決める

```rust
    fn collect<B: FromIterator<Self::Item>>(self) -> B
    where
        Self: Sized,
    {
        // ...（debug_assertions のときだけ size_hint を検査する処理）...
        FromIterator::from_iter(self)
    }
```

`collect` も、`FromIterator` トレイトに処理を任せているだけです。`sum` と同じく、**作る側の型が、作り方を決めます**。
`.collect::<Vec<_>>()` と `.collect::<HashSet<_>>()` と `.collect::<String>()` で、全く違う処理になるのはこのためです。

`Vec` の実装（`alloc/src/vec/mod.rs` で `FromIterator<T> for Vec<T>` を検索）はこうです。

```rust
impl<T> FromIterator<T> for Vec<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Vec<T> {
        <Self as SpecFromIter<T, I::IntoIter>>::from_iter(iter.into_iter())
    }
}
```

`SpecFromIter` は、標準ライブラリの内部でだけ使える**特殊化**（specialization）という仕組みのためのトレイトです。イテレータの種類に応じて、最適な作り方に切り替えます。例えば、要素数が前もって分かるイテレータなら、最初に必要な容量を確保して、ツアー02で見た「2倍ずつ伸ばす」コピーを避けます。

### `Result` に `collect` する

Stage 04・05 では、`Result` のイテレータを `Result<Vec<_>, _>` に `collect` すると、最初のエラーで止まることを使いました。`core/src/result.rs` で `FromIterator<Result<A, E>> for Result<V, E>` を検索してください。

```rust
impl<A, E, V: FromIterator<A>> FromIterator<Result<A, E>> for Result<V, E> {
    /// Takes each element in the `Iterator`: if it is an `Err`, no further
    /// elements are taken, and the `Err` is returned. Should no `Err` occur, a
    /// container with the values of each `Result` is returned.
```

ドキュメントコメントの通り、`Err` が出たらそれ以上の要素は取り出さずに、その `Err` を返します。

```
collect into Result: Err(ParseIntError { kind: InvalidDigit })
```

（`["1", "x", "3"]` を `parse` して `collect` した結果です。`"x"` で止まりました。）

---

## 6. ⭐ なぜ「ゼロコスト」なのか

ここまでで分かったことを、まとめてみます。

- `map` は、イテレータと関数を構造体に詰めるだけ
- `next` を呼ぶと、内側のイテレータの `next` が呼ばれ、関数が適用される
- メソッドの多くに `#[inline]` が付いている

### イテレータの「型」を見てみる

```rust
let it = vec![1u32, 2, 3].into_iter().map(|x| x * 2).filter(|x| x % 3 == 0);
println!("{}", std::any::type_name_of_val(&it));
```

実行すると、こう表示されます。

```
core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<u32>, sz::main::{{closure}}>, sz::main::{{closure}}>
```

整理すると、`Filter<Map<IntoIter<u32>, クロージャ1>, クロージャ2>` です。
**メソッドをつなげるたびに、型が入れ子になっていきます**。そしてクロージャは、それぞれが固有の型を持っています（Stage 05）。

つまり、このイテレータが何をするのかは、**すべて型としてコンパイル時に分かっています**。

- `dyn Iterator` のような動的ディスパッチ（CS基礎 07）を使っていないので、どの `next` を呼ぶかを実行時に調べる必要がない
- 単相化（CS基礎 07・10）によって、この組み合わせ専用のコードが生成される
- `#[inline]` によって、`Filter::next` → `Map::next` → `IntoIter::next` → `Option::map` → クロージャ が、すべて1つの関数の中に展開される

展開されたあとには、構造体もメソッドの呼び出しも残りません。手で書いたループと同じものになります。

### 実験: 機械語を比べる

次の2つの関数を、release ビルドで機械語（アセンブリ）に変換して比べました（Apple Silicon / arm64 の環境、rustc 1.95.0）。

```rust
#[unsafe(no_mangle)]
pub fn with_iter(v: &[u32]) -> u32 {
    v.iter().map(|x| x * 2).filter(|x| x % 3 == 0).sum()
}

#[unsafe(no_mangle)]
pub fn with_loop(v: &[u32]) -> u32 {
    let mut sum = 0;
    let mut i = 0;
    while i < v.len() {
        let x = v[i] * 2;
        if x % 3 == 0 {
            sum += x;
        }
        i += 1;
    }
    sum
}
```

```bash
cargo rustc --release -- --emit asm -C debuginfo=0
# target/release/deps/<クレート名>-<ハッシュ>.s に出力される
```

結果:

| | release ビルドの命令数 | debug ビルドの命令数 |
|---|---|---|
| `with_iter`（イテレータ） | 95 | 8 |
| `with_loop`（手書きのループ） | 95 | 74 |

- **release ビルドでは、どちらも 95 命令**でした。差分をとると、違いは関数の冒頭で「要素数が4未満か」を比べる書き方（`cmp #4` と `cmp #3`）が異なるなど、数行だけで、ループの本体は同じでした。どちらも、4つの要素を同時に処理する SIMD 命令（`add.4s` など）を使うループに変換されていました
- **debug ビルドでは、`with_iter` はわずか 8 命令**でした。インライン化されないので、`with_iter` は `Filter` や `Map` のメソッドを呼び出しているだけで、実際の処理は別の関数の中で行われています。関数呼び出しが何重にも重なるので、遅くなります

これが、CS基礎 10 で見た「イテレータは debug では約9倍遅いのに、release では手書きのループと同じ速さになる」の正体です。

**ゼロコスト抽象化**とは「抽象化を使っても、手で書いた場合と比べて、実行時の余分なコストがかからない」という意味です（選球眼 06 で、その思想の由来を扱います）。
ただし「コストがゼロ」なのは**実行時の話**です。その代わりに、

- **コンパイル時間**（単相化とインライン化には時間がかかる）
- **debug ビルドの速度**

というコストを払っています。何事にもトレードオフがあります。

---

## まとめ

| 学んだこと | ソースのどこに現れていたか |
|---|---|
| 実装が必要なのは `next` だけ | `fn next(&mut self) -> Option<Self::Item>;`（本体なし） |
| イテレータは遅延評価 | `map` は `Map::new(self, f)` を返すだけ。`#[must_use = "iterators are lazy ..."]` |
| `Map::next` の正体 | `self.iter.next().map(&mut self.f)`（ツアー01の `Option::map`） |
| `sum` の正体 | マクロで生成された `iter.fold(0, \|a, b\| a + b)` |
| `sum` でもオーバーフロー検査が効く | `#[rustc_inherit_overflow_checks]` |
| `collect` の作り方は、結果の型が決める | `FromIterator::from_iter(self)` |
| ゼロコスト | 型の入れ子 + 単相化 + `#[inline]`。実験で命令数が一致 |

---

## 問い（`answers/reading.md` へ）

1. `Iterator` に、`next` 以外にもう1つ必須のメソッドがあったとしたら、Stage 06 で `Countdown` を実装するときにどう困りましたか？ 逆に、デフォルト実装が多すぎることの欠点はありますか？
2. `v.iter().map(f).filter(g).take(3).collect::<Vec<_>>()` で、`f` は最大で何回呼ばれますか？ `Map::next` の実装をもとに説明してください。
3. `Box<dyn Iterator<Item = u32>>` を使うと、6節の「ゼロコスト」の条件のうち、どれが崩れますか？ それでも `dyn Iterator` を使いたくなるのは、どんな場面ですか？
4. `impl Sum for f64` がどこで、どう実装されているかを `accum.rs` で探してください。整数の実装と違うところはありますか？
5. 実験の命令数は、CPU の種類（arm64 / x86_64）やコンパイラのバージョンで変わります。手元で実験を再現し、`with_iter` と `with_loop` が同じになるかを確かめてください。もし違ったら、何が違うのかを調べてみましょう。
