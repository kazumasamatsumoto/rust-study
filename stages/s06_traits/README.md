# Stage 06: トレイトとジェネリクス — 解説と設計判断

> **前提知識**: [07 静的ディスパッチと動的ディスパッチ](../../docs/fundamentals/07_dispatch.md)（**必読**） / Stage 03 の「enum か trait か」 / Stage 04 の「エラー型の設計」

---

## 1. トレイトとは何か

トレイトとは「**この型は、これらのメソッドを持っている**」という約束です。
Java や C# の interface に似ていますが、いくつか重要な違いがあります。

```rust
trait Describe {
    fn name(&self) -> String;                 // 実装する側が必ず書くもの
    fn describe(&self) -> String {            // デフォルト実装（上書きもできる）
        format!("This is {}", self.name())
    }
}
```

### Java の interface との違い

| | Java の interface | Rust のトレイト |
|---|---|---|
| 実装を書く場所 | クラスの定義の中（`class Dog implements Describe`） | **型の定義とは別の場所**（`impl Describe for Dog`） |
| 既存の型に後から実装 | できない（`String` に自分の interface を実装できない） | **できる**（`impl Describe for i32` が書ける） |
| 呼び出し方 | 常に動的ディスパッチ | **デフォルトは静的**。`dyn` のときだけ動的 |
| 演算子や表示の仕組み | 言語に組み込み | `Add`, `Display` などの**トレイトとして**提供される |

「型の定義と、振る舞いの実装を分けられる」のが最大の違いです。標準ライブラリの型や、他のクレートの型に、自分のトレイトを実装できます。
この考え方は、Haskell の**型クラス**から来ています。

### 孤児ルール（orphan rule）

ただし制限が1つあります。**トレイトか型の、少なくともどちらかが自分のクレートのものでなければ、impl を書けません**。

```rust
impl Describe for i32 { ... }        // ○ Describe は自分のもの
impl Display for Money { ... }       // ○ Money は自分のもの
impl Display for Vec<i32> { ... }    // × どちらも標準ライブラリのもの
```

**なぜこの制限があるのか**: もし誰でも `impl Display for Vec<i32>` を書けたら、2つのクレートがそれぞれ別の実装を書いたときに、どちらを使えばよいのか決められなくなります。
「ある型とトレイトの組み合わせに対して、実装はプログラム全体で必ず1つ」という性質を**一貫性**（coherence）と呼びます。孤児ルールは、それを守るための仕組みです。
どうしても必要な場合は、`struct MyVec(Vec<i32>)` のように型を包んで自分の型にします（**newtype パターン**）。課題の `Money(i64)` も newtype です。

### Go の interface との違い

Go の interface は、**メソッドが揃っていれば、宣言しなくても自動的に実装したことになります**（構造的部分型）。Rust では、必ず `impl Trait for Type` と明示します（公称的）。

| | Go（暗黙的） | Rust（明示的） |
|---|---|---|
| 長所 | 宣言がいらず、気軽に使える | 「この型はこの約束を守るつもりだ」という**意図**がコードに残る |
| 短所 | 偶然メソッド名が一致しただけの型も実装したことになる | impl を書く手間がかかる |

## 2. ジェネリクスとトレイト境界

```rust
fn largest<T: PartialOrd + Copy>(v: &[T]) -> Option<T>
```

`T: PartialOrd + Copy` を**トレイト境界**と呼び、「T は比較ができて、コピーもできる型でなければならない」という意味です。
関数の中では、**境界に書いたトレイトのメソッドだけ**が使えます。逆に言えば、境界を見るだけで、その関数が引数に対して何をするのかが分かります。

**書き方のバリエーション**

```rust
fn f<T: Display>(x: T)              // 基本
fn f<T>(x: T) where T: Display      // 境界が長くなるときは where 句
fn f(x: impl Display)               // 引数の impl Trait（上と同じ意味の短縮形）
fn f() -> impl Display              // 戻り値の impl Trait（「具体的な型は隠す」）
```

全て**静的ディスパッチ**です。単相化され、型ごとに別の関数が作られます（[CS基礎 07](../../docs/fundamentals/07_dispatch.md)）。

### 境界は必要最小限にする

`largest` が `Ord` ではなく `PartialOrd` を要求しているので、`f64` にも使えます。
一方で、`PartialOrd` は「比較できない組がありうる」ことを許しているので、NaN が混ざったときの振る舞いは実装次第になります。
**境界をゆるくすると使える型が増え、その代わり、関数が守れる約束は弱くなります**。Stage 05 の「Fn / FnMut / FnOnce の選び方」と同じトレードオフです。

## 3. 標準のトレイトは「言語とつながる口」

Rust では、多くの言語機能がトレイトとして定義されています。トレイトを実装すると、自分の型がその機能を使えるようになります。

| トレイト | 実装すると使えるもの |
|---|---|
| `Display` | `println!("{}", x)`, `x.to_string()`, `format!` |
| `Debug` | `println!("{:?}", x)`, `assert_eq!` の失敗メッセージ |
| `From<T>` | `T` から `.into()` で変換できる。**`?` がエラーを変換するときにも使われる** |
| `Add`, `Sub`, `Mul` ... | `+`, `-`, `*` の演算子 |
| `PartialEq`, `PartialOrd` | `==`, `<` |
| `Iterator` | `for` ループ、`map` / `filter` / `sum` などの全てのアダプタ |
| `Sum` | `.sum()` |
| `Default` | `T::default()`, `#[derive(Default)]` |
| `Drop` | スコープを抜けるときの後片付け（Stage 08） |

### ⭐ `From` と `?` の関係（Stage 04 の続き）

```rust
fn load() -> Result<Config, MyError> {
    let text = std::fs::read_to_string("config.toml")?;  // io::Error が返ってくる
    ...
}
```

`?` は、Err を返すときに **`From::from` を呼んでエラーの型を変換します**。`impl From<io::Error> for MyError` を書いておけば、`io::Error` が自動で `MyError` に変換されます。
Stage 04 で `map_err` を書いて変換していた処理は、`From` を実装すれば消せます。`thiserror` の `#[from]` は、この impl を自動で生成しているだけです。

### ⭐ `Iterator` は next を1つ書くだけでいい

```rust
impl Iterator for Countdown {
    type Item = u32;
    fn next(&mut self) -> Option<u32> { ... }   // これだけ
}
```

`next` を1つ書くだけで、`map`, `filter`, `take`, `zip`, `sum`, `collect` など、**70以上のメソッドが全て使えるようになります**。
それらは全て、`Iterator` トレイトの**デフォルト実装**として、`next` を使って書かれているからです。
「必須のメソッドを最小限にして、残りはデフォルト実装で提供する」のは、良いトレイトを設計するときの定番のパターンです。

### `type Item` は関連型

`Iterator<Item = u32>` の `Item` は**関連型**と呼ばれます。ジェネリクス `Iterator<T>` にしなかったのは、「1つの型が返す要素の型は、1つに決まる」からです。
ジェネリクスにすると、`Countdown` が `Iterator<u32>` と `Iterator<String>` の両方を実装できてしまい、`for x in countdown` の `x` の型が決まらなくなります。

## 4. ⭐ Expression Problem をもう一度

課題 03 では、Stage 03 の Shape を trait で作り直し、2つの変更を加えました。

| 変更 | Stage 03（enum） | Stage 06（trait） |
|---|---|---|
| 新しい**種類**（Hexagon）を追加 | 全ての `match` に分岐を足す必要がある | **新しい struct と impl を1つ足すだけ**。既存のコードは変更しない |
| 新しい**操作**（perimeter）を追加 | **新しい関数を1つ書くだけ** | トレイトにメソッドを足し、**全ての型の impl を修正する** |

あなたが課題 03 で、`perimeter` を4つの型に書いたときの手間を思い出してください。enum なら、`match` が1つの関数で済んでいたはずです。

**どちらが良いかは、将来何が増えるかで決まります。**
- 図形描画ソフトのように、**プラグインで図形の種類が増える** → trait
- コンパイラの AST のように、**ノードの種類は決まっていて、処理（型検査、最適化、コード生成）が増える** → enum

そして性能の面では:
- enum: ヒープの確保がなく、`Vec<Shape>` に値が直接並ぶ。match もインライン化できる
- `Box<dyn Shape>`: 1つずつヒープに確保され、呼び出しは vtable を経由する

（[CS基礎 07](../../docs/fundamentals/07_dispatch.md) の実験で、この差を計測しました。）

---

## 設計の問い

`answers/s06.md` に答えてください。

1. 課題 03 で `perimeter` を追加したとき、enum 版と比べて何が大変でしたか？ 逆に `Hexagon` の追加はどうでしたか？ 自分の体験として書いてください。
2. ログの出力先（標準出力、ファイル、外部のサービス）を切り替えられるようにしたいとします。出力先は、ライブラリの利用者が自分で追加できるようにしたい。enum・ジェネリクス・`dyn Trait` のどれで設計しますか？ 設計のスケッチ（トレイトの定義など）も書いてください。
3. `Money` に `impl Add for Money` を実装しました。では `impl Mul for Money`（お金 × お金）は実装すべきですか？ 実装すべきでないとしたら、代わりにどんな演算を用意しますか？
4. Go の interface（暗黙的に実装される）と Rust のトレイト（明示的に実装する）では、大きなコードベースで、それぞれどんな問題が起きやすいと思いますか？
