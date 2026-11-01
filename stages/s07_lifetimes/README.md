# Stage 07: ライフタイム — 解説と設計判断

> **前提知識**: [03 スタックとヒープ](../../docs/fundamentals/03_memory.md) / [04 ポインタとメモリ安全性](../../docs/fundamentals/04_pointers.md) の「ダングリングポインタ」
> ライフタイムは「参照が、消えたデータを指さないようにする仕組み」です。何を防いでいるのかを先に知っておくと、理解がずっと楽になります。

---

## 1. ライフタイムとは何か

**ライフタイム**とは、「その参照が有効でいられる、コード上の範囲」のことです。

参照は、データのアドレスを持っているだけです（[CS基礎 04](../../docs/fundamentals/04_pointers.md)）。データが先に消えてしまうと、参照は**消えたメモリを指す**ことになります。これを**ダングリング参照**と呼びます。

```rust
fn main() {
    let r;
    {
        let s = String::from("hello");
        r = &s;
    }               // s はここで消える（ヒープも解放される）
    println!("{r}"); // r は消えた s を指している
}
```

Rust はこれをコンパイルエラーにします。実際のメッセージはこうです。

```
error[E0597]: `s` does not live long enough
 --> examples/err2.rs:5:13
  |
4 |         let s = String::from("hello");
  |             - binding `s` declared here
5 |         r = &s;
  |             ^^ borrowed value does not live long enough
6 |     }
  |     - `s` dropped here while still borrowed
7 |     println!("{r}");
  |                - borrow later used here
```

エラーメッセージが「どこで作られ」「どこで消え」「どこでまだ使われているか」の3点を示しているのに注目してください。**ライフタイムのエラーは、この3点を読めば必ず理解できます。**

コンパイラ（の中の**借用チェッカー**）は、全ての参照について「参照が使われている範囲 ⊆ データが生きている範囲」になっているかを確かめています。

## 2. 注釈 `'a` は「関係」を書くもの

ほとんどの場合、ライフタイムはコンパイラが自動で推論します。注釈が必要になるのは、**関数のシグネチャだけを見ても、戻り値がどの引数から借りたものなのか分からない**場合です。

```rust
fn longest(a: &str, b: &str) -> &str {
    if a.len() >= b.len() { a } else { b }
}
```

```
error[E0106]: missing lifetime specifier
 --> examples/err1.rs:1:33
  |
1 | fn longest(a: &str, b: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `a` or `b`
```

`help` の文がそのまま答えです。「戻り値が `a` と `b` のどちらから借りたものか、シグネチャに書かれていない」。

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str
```

これは「戻り値は、a と b の**両方が生きている間**だけ有効」と読みます。

⭐ **注釈はライフタイムを変えません。** 「長生きさせる」効果はありません。すでにある関係を、コンパイラと読む人に**説明しているだけ**です。

### なぜ関数の中身から推論しないのか

コンパイラは関数の中身を読めば、どの引数を返しているか分かるはずです。それでもシグネチャに書かせるのは、**シグネチャを「約束」にするため**です。

- 呼び出す側は、中身を読まずにシグネチャだけで使い方を判断できる
- 中身を書き換えたときに、呼び出し側が勝手に壊れない（約束が変わるならシグネチャを変える必要がある）
- コンパイラは関数ごとに検査すればよく、プログラム全体を解析しなくて済む（コンパイルが速くなる）

### 注釈が守ってくれること

```rust
let a = String::from("long string");
let result;
{
    let b = String::from("xyz");
    result = longest(&a, &b);
}
println!("{result}");
```

```
error[E0597]: `b` does not live long enough
  --> examples/err4.rs:9:30
   |
 8 |         let b = String::from("xyz");
   |             - binding `b` declared here
 9 |         result = longest(&a, &b);
   |                              ^^ borrowed value does not live long enough
10 |     }
   |     - `b` dropped here while still borrowed
11 |     println!("{result}");
   |                ------ borrow later used here
```

実行すれば `a` のほうが長いので、`result` は `a` を指すはずです。それでもエラーになります。**シグネチャが「b から借りている可能性もある」と約束しているから**です。コンパイラは実行時の値ではなく、型と約束だけを見て判断します。

## 3. 省略規則（elision）

毎回 `'a` を書くのは大変なので、よくあるパターンでは省略できます。コンパイラは次の3つの規則で、書かれていないライフタイムを補います。

1. 引数の参照には、それぞれ別のライフタイムが付く
2. 引数の参照が**1つだけ**なら、戻り値はそれと同じライフタイムになる
3. メソッドで `&self` か `&mut self` があれば、戻り値は `self` と同じライフタイムになる

```rust
fn first_line(s: &str) -> &str           // 規則2: 戻り値は s から借りている
fn name(&self) -> &str                   // 規則3: 戻り値は self から借りている
fn longest(a: &str, b: &str) -> &str     // どの規則でも決まらない → エラー
```

⭐ **規則3が罠になることがあります**（課題 02 の `Tokenizer` を参照）。

## 4. 構造体が参照を持つとき

```rust
struct Excerpt {
    part: &str,
}
```

```
error[E0106]: missing lifetime specifier
 --> examples/err5.rs:2:11
  |
2 |     part: &str,
  |           ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct Excerpt<'a> {
2 ~     part: &'a str,
```

`struct Excerpt<'a>` は「この構造体は `'a` の間だけ生きているデータを借りていて、**それより長く生きられない**」という意味です。

### ⭐ 規則3の罠: 戻り値を `&self` に結びつけてしまう

課題 02 の `Tokenizer::next_word` を、省略規則に任せて書いたとします。

```rust
impl<'a> Parser<'a> {
    fn next_word(&mut self) -> Option<&str> { ... }  // 規則3で、戻り値は &mut self に結びつく
}

let first = p.next_word();
let second = p.next_word();
println!("{first:?} {second:?}");
```

```
error[E0499]: cannot borrow `p` as mutable more than once at a time
  --> examples/err6.rs:14:18
   |
13 |     let first = p.next_word();
   |                 - first mutable borrow occurs here
14 |     let second = p.next_word();
   |                  ^ second mutable borrow occurs here
15 |     println!("{first:?} {second:?}");
   |                ----- first borrow later used here
```

返した単語は、本当は**入力の文字列**から借りています。ところがシグネチャでは「パーサ（`&mut self`）から借りている」と約束してしまったので、`first` を持っている間はパーサがずっと借りられたままになり、2回目の呼び出しができません。

`-> Option<&'a str>` と書けば「入力から借りている」という正しい関係になり、この問題は消えます。
**ライフタイムのエラーの多くは、「本当は何から借りているのか」をシグネチャに正しく書いていないことが原因です。**

## 5. `'static`

`'static` は「プログラムが終わるまで有効」という意味です。

```rust
let s: &'static str = "hello";
```

文字列リテラルは**実行ファイルの中に埋め込まれていて**、プログラムが動いている間ずっとメモリ上にあります。ヒープでもスタックでもない、読み取り専用の領域です。だから `'static` になれます。

関数の中で作った `String` の参照を `'static` として返すことはできません。

```
error[E0515]: cannot return reference to local variable `s`
 --> examples/err3.rs:3:5
  |
3 |     &s
  |     ^^ returns a reference to data owned by the current function
```

### よくある誤解: `T: 'static`

`&'static T`（「static な参照」）と `T: 'static`（トレイト境界）は意味が違います。

- `&'static T`: 永遠に有効なデータへの参照
- `T: 'static`: 「T は、`'static` でない参照を**中に含まない**」という意味。**`String` や `Vec<i32>` も `'static` を満たします**（参照を含まず、全部を所有しているので）

スレッドに値を渡すとき（`std::thread::spawn`）に `T: 'static` が要求されます。これは「スレッドがいつまで動くか分からないので、どこかの変数から借りた参照は渡せない。所有したデータを渡しなさい」という意味です（Stage 10）。

## 6. なぜ GC のある言語にはライフタイムがないのか

Java や Go や Python では、ダングリング参照は起きません。**参照されている間は、GC がデータを解放しないから**です。データの寿命が「参照が残っているかどうか」で実行時に決まります。

| | GC のある言語 | Rust |
|---|---|---|
| データの寿命を決めるもの | 実行時に、参照が残っているか | コンパイル時に、所有者のスコープ |
| ダングリング参照 | 起きない | 起きない（コンパイル時に拒否） |
| 実行時のコスト | GC の処理、一時停止 | なし |
| 書く人の負担 | 小さい | **ライフタイムを考える必要がある** |
| 気付きにくいバグ | 不要になったデータを参照し続けてメモリが減らない | — |

C++ にもライフタイムの概念はありますが、**コンパイラが検査しません**。`std::string_view`（Rust の `&str` に相当）が消えた文字列を指すバグは、C++ では今でもよく起きます。

Rust のライフタイムは「C++ のプログラマが頭の中でやっていた寿命の管理を、型として書き出し、コンパイラに検査させるもの」だと言えます。

## 7. ⭐ 選球眼: 借りるか、所有するか

構造体に `&'a str` を持たせるか、`String` を持たせるかは、Rust の設計で毎回迷うポイントです。

| | 借りる（`&'a str`） | 所有する（`String`） |
|---|---|---|
| コピー | なし（ゼロコピー）| 作るときにコピーが起きる |
| 構造体の寿命 | 元のデータより長く生きられない | 自由 |
| 型の扱いやすさ | `<'a>` が、使う側の型にまで広がっていく | シンプル |
| 関数から返す、スレッドに渡す、保存する | 難しい | 簡単 |

**実務での目安**
- **迷ったら所有する**。コピーのコストは、ほとんどの場合は問題になりません。計測して問題になってから借りる版にする
- **借りるのが向いているもの**: パーサやトークナイザ、イテレータ、一時的な「ビュー」。入力が大きく、処理が短い間だけ使うもの
- **所有するのが向いているもの**: 設定、ドメインのデータ、長く保持するもの、関数から返すもの、スレッドに渡すもの
- 両方の良いところが欲しいなら `Cow<'a, str>`（Stage 08）

`serde` も同じ選択肢を用意しています。`#[derive(Deserialize)]` した構造体に `&'a str` を持たせればゼロコピーで解析できますが、ほとんどの人は `String` を使います。

---

## 設計の問い

`answers/s07.md` に答えてください。

1. **シグネチャを書く問題**: 次の関数のシグネチャには、ライフタイム注釈が足りません。正しい注釈を書いてください（スクラッチのファイルで、コンパイルが通るか確かめること）。また、注釈が「なぜその形になるのか」を言葉で説明してください。
   ```rust
   // text の中から、key の後ろに続く値を返す。例: ("name=rust;ver=1", "ver=") -> Some("1")
   fn value_after(text: &str, key: &str) -> Option<&str> {
       let start = text.find(key)? + key.len();
       let rest = &text[start..];
       Some(rest.split(';').next().unwrap_or(rest))
   }
   ```
2. `longest` を `fn longest<'a, 'b>(a: &'a str, b: &'b str) -> &'a str` とすることはできますか？ できないなら、その理由は？
3. Web サーバで、リクエストの本文を解析した結果を構造体にします。この構造体は `&'a str` と `String` のどちらでフィールドを持つべきですか？ 解析の結果を別のスレッドのキューに渡す場合はどうですか？
4. 「`String` は `'static` を満たす」と聞いて、意外に思いましたか？ `T: 'static` が「T が永遠に生きる」という意味ではない理由を説明してください。
