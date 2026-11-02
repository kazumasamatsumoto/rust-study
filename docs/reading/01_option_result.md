# ツアー01: `Option` / `Result` と `?` の正体

> **出典**: rustc 1.95.0 の rust-src
> - `library/core/src/option.rs`
> - `library/core/src/result.rs`
> - `library/core/src/ops/try_trait.rs`
> - `library/core/src/ops/control_flow.rs`
>
> **つながるところ**: Stage 04（エラー処理）

Stage 04 では `map`, `and_then`, `unwrap_or_else`, `ok_or`, `map_err`, `?` を使いました。
このツアーの結論を先に書いておきます。**これらはすべて、ただの `match` です。**

---

## 1. `Option` の定義

`core/src/option.rs` で `pub enum Option<T>` を検索してください。

```rust
#[derive(Copy, Debug, Hash)]
#[derive_const(Eq)]
#[rustc_diagnostic_item = "Option"]
#[lang = "Option"]
#[stable(feature = "rust1", since = "1.0.0")]
#[allow(clippy::derived_hash_with_manual_eq)] // PartialEq is manually implemented equivalently
pub enum Option<T> {
    /// No value.
    #[lang = "None"]
    #[stable(feature = "rust1", since = "1.0.0")]
    None,
    /// Some value of type `T`.
    #[lang = "Some"]
    #[stable(feature = "rust1", since = "1.0.0")]
    Some(#[stable(feature = "rust1", since = "1.0.0")] T),
}
```

属性を取り除くと、こうなります。

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

Stage 03 で自分で書いた enum と、何も変わりません。**`Option` は言語に組み込まれた特別な型ではなく、標準ライブラリにある普通の enum です。**
`#[lang = "Option"]` は、`for` ループや `?` のような構文の側から `Option` を見つけるための目印です。

## 2. `map`: 中身があるときだけ関数を適用する

`pub const fn map<` を検索してください。

```rust
#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_const_unstable(feature = "const_option_ops", issue = "143956")]
pub const fn map<U, F>(self, f: F) -> Option<U>
where
    F: [const] FnOnce(T) -> U + [const] Destruct,
{
    match self {
        Some(x) => Some(f(x)),
        None => None,
    }
}
```

（`[const]` と `Destruct` の読み方は [README](README.md) を参照してください。`F: FnOnce(T) -> U` と読み替えて構いません。）

本体は5行の `match` です。注目してほしい点が3つあります。

- **`self` を値で受け取っている**（`&self` ではない）。`map` を呼ぶと、元の `Option` はムーブされます。中身の `T` を `f` に渡すためには、所有権が必要だからです
- **`F: FnOnce`**。`f` は高々1回しか呼ばれないので、最も制約がゆるい `FnOnce` で受け取っています（Stage 05 の「引数は最も一般的な型で受け取る」と同じ考え方）
- **`#[inline]`**。この小さな関数は、呼び出し元に展開されます。最適化されると、`opt.map(f)` は自分で `match` を書いたのと同じ機械語になります

## 3. `and_then`: 関数自身が `Option` を返す場合

```rust
pub const fn and_then<U, F>(self, f: F) -> Option<U>
where
    F: [const] FnOnce(T) -> Option<U> + [const] Destruct,
{
    match self {
        Some(x) => f(x),
        None => None,
    }
}
```

`map` との違いは、`Some(f(x))` が `f(x)` になっていることだけです。
`f` 自身が `Option` を返すので、`map` を使うと `Option<Option<U>>` になってしまいます。それを避けるのが `and_then` です。
このメソッドには `#[doc(alias = "flatmap")]` という属性も付いています。他の言語で `flatMap` と呼ばれているものと同じだ、という意味です。

## 4. `unwrap_or` と `unwrap_or_else`: 正格評価と遅延評価

```rust
pub const fn unwrap_or(self, default: T) -> T
where
    T: [const] Destruct,
{
    match self {
        Some(x) => x,
        None => default,
    }
}
```

```rust
pub const fn unwrap_or_else<F>(self, f: F) -> T
where
    F: [const] FnOnce() -> T + [const] Destruct,
{
    match self {
        Some(x) => x,
        None => f(),
    }
}
```

違いは「デフォルト値をいつ作るか」です。

- `unwrap_or(default)`: **呼び出す前に** `default` が計算されます（**正格評価**）。`Some` だった場合、その計算は無駄になります
- `unwrap_or_else(f)`: `None` だったときだけ `f()` が呼ばれます（**遅延評価**）

`unwrap_or` のドキュメントコメントにも、こう書かれています。

```rust
/// Arguments passed to `unwrap_or` are eagerly evaluated; if you are passing
/// the result of a function call, it is recommended to use [`unwrap_or_else`],
/// which is lazily evaluated.
```

`opt.unwrap_or(expensive())` と書くと、`opt` が `Some` でも `expensive()` が実行されます。clippy が `unwrap_or_else` を勧めてくるのはこのためです（Stage 05 の `or_compute` で、同じことを体験しました）。

## 5. `unwrap`: panic の処理を別の関数に追い出す

```rust
#[inline(always)]
#[track_caller]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_diagnostic_item = "option_unwrap"]
#[rustc_allow_const_fn_unstable(const_precise_live_drops)]
#[rustc_const_stable(feature = "const_option", since = "1.83.0")]
pub const fn unwrap(self) -> T {
    match self {
        Some(val) => val,
        None => unwrap_failed(),
    }
}
```

`None` のときは `unwrap_failed()` を呼んでいます。同じファイルで `fn unwrap_failed` を検索してください。

```rust
#[cfg_attr(not(panic = "immediate-abort"), inline(never))]
#[cfg_attr(panic = "immediate-abort", inline)]
#[cold]
#[track_caller]
const fn unwrap_failed() -> ! {
    panic("called `Option::unwrap()` on a `None` value")
}
```

ここには、性能のための工夫が詰まっています。

| 工夫 | 理由 |
|---|---|
| `unwrap` 本体は `#[inline(always)]` | よく通る道（`Some` の場合）は、呼び出し元に展開して速くする |
| panic の処理は別の関数に分け、`#[inline(never)]` | panic のメッセージを組み立てるコードまで展開すると、`unwrap` を呼んでいる全ての場所でプログラムが大きくなる。めったに通らない道は、1か所にまとめておく |
| `#[cold]` | 「ここはめったに呼ばれない」とコンパイラに伝えて、よく通る道を優先して最適化してもらう |
| `#[track_caller]` | panic したときに、標準ライブラリの中ではなく **`unwrap()` を呼んだあなたのコードの行**を表示する |
| 戻り値の型 `!` | 「この関数は決して戻ってこない」という意味の型（never 型）。そのため `match` の片方の腕が `T` で、もう片方が `!` でも型が合う |

`Result::unwrap` も同じ形です（`core/src/result.rs` で `pub fn unwrap(self) -> T` を検索）。

```rust
pub fn unwrap(self) -> T
where
    E: fmt::Debug,
{
    match self {
        Ok(t) => t,
        Err(e) => unwrap_failed("called `Result::unwrap()` on an `Err` value", &e),
    }
}
```

`E: fmt::Debug` という制約があるのは、panic のメッセージにエラーの中身を表示するためです。`unwrap` を使うには、エラー型が `Debug` を実装している必要があります。

## 6. `ok_or` と `map_err`: Option と Result の橋渡し

```rust
pub const fn ok_or<E: [const] Destruct>(self, err: E) -> Result<T, E> {
    match self {
        Some(v) => Ok(v),
        None => Err(err),
    }
}
```

```rust
// core/src/result.rs
pub const fn map_err<F, O>(self, op: O) -> Result<T, F>
where
    O: [const] FnOnce(E) -> F + [const] Destruct,
{
    match self {
        Ok(t) => Ok(t),
        Err(e) => Err(op(e)),
    }
}
```

どちらも、やはりただの `match` です。Stage 04 の `eval_division` で `map_err` を使ったとき、裏ではこれが動いていました。

---

## 7. ⭐ `?` の正体

いよいよ `?` です。`?` は構文なので、`fn ?` のような関数があるわけではありません。
コンパイラは `?` を、**`Try` というトレイトのメソッド呼び出しに書き換えます**。

### 7-1. `ControlFlow`: 「続ける」か「抜ける」か

まず、`core/src/ops/control_flow.rs` の `pub enum ControlFlow` を見てください。

```rust
pub enum ControlFlow<B, C = ()> {
    /// Move on to the next phase of the operation as normal.
    #[stable(feature = "control_flow_enum_type", since = "1.55.0")]
    #[lang = "Continue"]
    Continue(C),
    /// Exit the operation without running subsequent phases.
    #[stable(feature = "control_flow_enum_type", since = "1.55.0")]
    #[lang = "Break"]
    Break(B),
    // Yes, the order of the variants doesn't match the type parameters.
    // They're in this order so that `ControlFlow<A, B>` <-> `Result<B, A>`
    // is a no-op conversion in the `Try` implementation.
}
```

「処理を続ける（`Continue`）」か「ここで抜ける（`Break`）」かを表す、ただの enum です。

コメントにも注目してください。バリアントの並び順は、型パラメータの順番と逆になっています。こうしておくと、メモリ上の表現が `Result` と同じになり、`Result` から `ControlFlow` への変換が**何もしなくて済む**（no-op）からです。CS基礎 03 で見た「メモリ上の表現」が、ここでも意識されています。

### 7-2. `Try` トレイトの実装

`core/src/option.rs` で `Try for Option<T>` を検索してください。

```rust
#[unstable(feature = "try_trait_v2", issue = "84277", old_name = "try_trait")]
#[rustc_const_unstable(feature = "const_try", issue = "74935")]
impl<T> const ops::Try for Option<T> {
    type Output = T;
    type Residual = Option<convert::Infallible>;

    #[inline]
    fn from_output(output: Self::Output) -> Self {
        Some(output)
    }

    #[inline]
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Some(v) => ControlFlow::Continue(v),
            None => ControlFlow::Break(None),
        }
    }
}
```

`branch` が `?` の心臓部です。

- `Some(v)` なら `Continue(v)`: 中身を取り出して、処理を続ける
- `None` なら `Break(None)`: 関数から抜ける

`Result` の実装（`core/src/result.rs` で `Try for Result<T, E>` を検索）も、同じ形です。

```rust
impl<T, E> const ops::Try for Result<T, E> {
    type Output = T;
    type Residual = Result<convert::Infallible, E>;

    #[inline]
    fn from_output(output: Self::Output) -> Self {
        Ok(output)
    }

    #[inline]
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Ok(v) => ControlFlow::Continue(v),
            Err(e) => ControlFlow::Break(Err(e)),
        }
    }
}
```

### 7-3. `?` は何に書き換えられるのか

`core/src/ops/try_trait.rs` のドキュメントコメントに、答えが書いてあります（`simple_try_fold_3` を検索）。

```rust
///     for x in iter {
///         let cf = f(accum, x).branch();
///         match cf {
///             ControlFlow::Continue(a) => accum = a,
///             ControlFlow::Break(r) => return R::from_residual(r),
///         }
///     }
```

そして、その直後にこう書かれています。

```rust
/// But this "call `branch`, then `match` on it, and `return` if it was a
/// `Break`" is exactly what happens inside the `?` operator.
```

つまり、`let x = expr?;` は、おおよそ次のように書き換えられます。

```rust
let x = match Try::branch(expr) {
    ControlFlow::Continue(v) => v,
    ControlFlow::Break(r) => return FromResidual::from_residual(r),
};
```

**`?` も、結局は `match` と `return` でした。**

### 7-4. ⭐ `?` がエラー型を自動で変換できる理由

Stage 04 では、`?` を使うとエラー型が自動で変換されることがありました。その仕組みが、`Result` の `from_residual` にあります（`FromResidual<Result<convert::Infallible, E>>` を検索）。

```rust
impl<T, E, F: [const] From<E>> const ops::FromResidual<Result<convert::Infallible, E>>
    for Result<T, F>
{
    #[inline]
    #[track_caller]
    fn from_residual(residual: Result<convert::Infallible, E>) -> Self {
        match residual {
            Err(e) => Err(From::from(e)),
        }
    }
}
```

`Err(From::from(e))` に注目してください。

- `?` を使った式のエラー型は `E`
- 関数の戻り値のエラー型は `F`
- **`F: From<E>` が実装されていれば、`From::from(e)` で `E` を `F` に変換してから return します**

例えば、独自のエラー型に `impl From<ParseIntError> for MyError` を書いておけば、`s.parse::<i32>()?` と書くだけで、`ParseIntError` が `MyError` に変換されます。
`thiserror` の `#[from]` 属性は、この `From` の実装を自動生成するものです。

### 7-5. `Infallible`: 「起こりえない」を型で表す

`Residual` の型は `Option<Infallible>` や `Result<Infallible, E>` でした。`Infallible` の定義は、`core/src/convert/mod.rs` にあります。

```rust
pub enum Infallible {}
```

**バリアントが1つもない enum** です。値を1つも作れない型なので、

- `Option<Infallible>` は `None` しかありえない
- `Result<Infallible, E>` は `Err(e)` しかありえない

ということになります。だからこそ、上の `from_residual` の `match` は、`Err(e)` の腕1つだけで網羅できていました（`Ok` の腕を書く必要がない）。

「ありえない状態を型で表現できなくする」（Stage 03）が、標準ライブラリの奥深くでも使われています。

---

## まとめ

| 使ったもの | 正体 |
|---|---|
| `Option` | ただの enum |
| `map`, `and_then`, `unwrap_or_else`, `ok_or`, `map_err` | ただの `match`。`#[inline]` で展開されるので、手で `match` を書くのとコストは同じ |
| `unwrap` | `match` + めったに通らない panic 用の関数 |
| `?` | `Try::branch` を呼んで `match` し、`Break` なら `return` する |
| `?` のエラー型の自動変換 | `from_residual` の中の `From::from(e)` |

**コンビネータ（`map` など）と `match` のどちらで書いても、性能は変わりません。** 読みやすい方を選びましょう。

---

## 問い（`answers/reading.md` へ）

1. `Option::map` が `&self` ではなく `self` を受け取るのはなぜですか？ `&self` で書くとしたら、戻り値の型はどうなりますか？（ヒント: `as_ref()` を調べてみましょう）
2. `unwrap_failed` を `unwrap` と別の関数に分けず、`unwrap` の中に直接 `panic!` を書いたとしたら、何が困りますか？
3. Stage 04 の `eval_division` で `map_err` を使って `String` に変換していた部分を、`?` の自動変換だけで済ませるには、何を実装すればよいですか？
4. `pub enum Infallible {}` のように「値を作れない型」は、他にどんな場面で役に立ちそうですか？
5. `core/src/option.rs` を開いて、Stage 04 で使わなかったメソッドを1つ選び、その実装を読んで、何をするメソッドかを説明してください。
