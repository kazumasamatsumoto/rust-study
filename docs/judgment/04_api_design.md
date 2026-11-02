# 04. API の設計 — 使う人のための型を選ぶ

> **前提**: Stage 01〜06（特に s02 の借用、s04 のエラー、s06 のトレイト）を終えていること。Stage 09 の前後に読むのがおすすめです。
> **参考**: [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)（Rust のライブラリチームがまとめた設計の指針）、[Cargo Book: SemVer Compatibility](https://doc.rust-lang.org/cargo/reference/semver.html)

**API**（Application Programming Interface）とは、関数のシグネチャ、型、トレイトなど、**コードの外から見える部分**のことです。
これまでの課題では、シグネチャ（関数の名前・引数の型・戻り値の型）は最初から与えられていました。Part 3 からは、それを自分で決めることになります。

API の設計が大事なのは、次の2つの理由からです。

1. **使う人の方が多い**。関数を書くのは1回でも、呼び出されるのは何十回、何百回です。使いにくい API は、そのたびに使う人の時間を奪います
2. **公開した API は、簡単には変えられない**。ライブラリとして公開した API を変えると、それを使っている全員のコードが壊れます（6節）

---

## 1. 名前の付け方: 名前でコストを伝える

Rust には、メソッド名の頭に付ける言葉で「**どれくらいコストがかかるか**」「**所有権がどうなるか**」を伝える慣習があります。

| 接頭辞 | コスト | 所有権 | 標準ライブラリの例 |
|---|---|---|---|
| `as_` | ほぼゼロ（見方を変えるだけ） | 借用 → 借用 | `str::as_bytes(&self) -> &[u8]` |
| `to_` | かかる（新しく作ることが多い） | 借用 → 所有 | `str::to_lowercase(&self) -> String` |
| `into_` | 状況による（多くは安い） | 所有 → 所有（`self` を消費する） | `String::into_bytes(self) -> Vec<u8>` |

ツアー03で読んだ通り、`as_bytes` は `String` の中身をそのまま見せるだけ（コピーなし）で、`into_bytes` は中の `Vec<u8>` を取り出すだけです。一方 `to_lowercase` は、新しい `String` を作ります。
**名前を見ただけで、どれが重い処理なのかが分かる**ようになっています。

### その他の慣習

| 慣習 | 例 | 理由 |
|---|---|---|
| ゲッター（値を取り出すメソッド）に `get_` を付けない | `Account::owner()`, `Account::balance()`（s03） | Rust では `get` は `HashMap::get` のように「あるかもしれないものを探す」意味で使うことが多いため |
| コンストラクタは `new` | `Rectangle::new`（s03） | 引数が違うものは `with_capacity`, `from_...` のように名前で区別する |
| イテレータを返すメソッドは `iter` / `iter_mut` / `into_iter` | `Vec::iter` | `&self` / `&mut self` / `self` の3種類に対応する |
| `bool` を返すメソッドは `is_` / `has_` | `is_empty`, `is_char_boundary` | 文として読める（`if s.is_empty()`） |
| 型は `UpperCamelCase`、関数と変数は `snake_case` | `HitCounter`, `parallel_sum` | `cargo clippy` と rustc が警告してくれる |

## 2. ⭐ 引数: 「必要な分だけ要求する」

Rust API Guidelines には「**呼び出し側に、コピーする場所を決めさせる**」（caller decides where to copy and place data）という指針があります。

- 関数の中で**読むだけ**なら、**借用**（`&T`）で受け取る
- 関数の中で**所有する必要がある**（保存する、返す、別のスレッドに送る）なら、**所有権**（`T`）で受け取る

### 例1: 読むだけなら借用（s02）

```rust
pub fn doubled(v: &[i32]) -> Vec<i32>
```

`v` を読むだけなので借用で受け取り、新しく作った `Vec` を所有権ごと返しています。そして Stage 02 で学んだ通り、`&Vec<i32>` より `&[i32]` の方が、Vec も配列も受け取れるので**一般的**です。

### 例2: 中でコピーするなら、所有権で受け取る方がよいこともある（s03）

```rust
impl Account {
    pub fn new(owner: &str) -> Self { /* owner.to_string() して保存する */ }
}
```

`Account` は名前を `String` として保存するので、中で `owner.to_string()` が必要です。ここで問題になるのが、**呼び出し側がすでに `String` を持っている場合**です。

```rust
let name: String = read_name_from_input();
let a = Account::new(&name);   // name を借用して渡す → 中でもう一度コピーされる
// name はこの後使わないのに、コピーのコスト（ヒープの確保）を払っている
```

代わりの設計は、次の通りです。

| シグネチャ | 呼び出し側の書き方 | コスト |
|---|---|---|
| `new(owner: &str)` | `new("kazu")`, `new(&name)` | 常に中でコピーする |
| `new(owner: String)` | `new("kazu".to_string())`, `new(name)` | `String` を持っていればムーブで済む。リテラルからは呼び出し側でコピーする |
| `new(owner: impl Into<String>)` | `new("kazu")`, `new(name)` | どちらも書ける。`String` ならムーブ、`&str` なら中でコピー |

`impl Into<String>` は便利ですが、ジェネリクスなので単相化されるコード（CS基礎 07・10）が増え、シグネチャも少し読みにくくなります。
**s03 の課題では、学習のために分かりやすい `&str` にしていました。** ライブラリとして公開する API なら、`impl Into<String>` を検討する価値があります。

### 例3: 標準ライブラリの柔らかい引数

```rust
// std/src/fs.rs
pub fn read<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>>
```

`AsRef<Path>` を実装している型（`&str`, `String`, `&Path`, `PathBuf` など）なら、何でも渡せます。ファイルのパスはいろいろな型で持たれるので、使う人に変換させないための工夫です。

## 3. ⭐ 戻り値: 失敗をどう伝えるか

| 戻り値の型 | 向いている場面 | 課題の例 |
|---|---|---|
| `bool` | 成功か失敗かだけが分かればよい | s03 `Account::withdraw(&mut self, amount: u64) -> bool` |
| `Option<T>` | 失敗の理由が1つしかない（または自明） | s09 `Celsius::new(value: f64) -> Option<Self>` |
| `Result<T, E>` | 失敗の理由が複数あり、呼び出し側が区別したい | s10 `Bank::transfer(...) -> Result<(), TransferError>` |

s10 の `TransferError` は、`SameAccount`, `NoSuchAccount`, `InsufficientFunds` の3種類の失敗を区別しています。呼び出し側は、「口座番号の入力ミス」と「残高不足」で、ユーザーに見せるメッセージを変えられます。

s03 の `withdraw` は `bool` を返していました。これには問題が2つあります。

1. 戻り値を無視しても、警告が出ない（`a.withdraw(100);` と書いて、引き出せたかを確かめ忘れる）
2. 将来「1日の上限を超えた」という失敗の理由が増えたときに、`bool` では表せない

1. は、`#[must_use]` を付ければ防げます。2. まで考えるなら、`Result<(), WithdrawError>` にすべきです。

s09 の `Celsius::new` も、「絶対零度未満」と「NaN」の2つの理由で失敗します。今は `Option` ですが、使う人がその2つを区別したくなるなら、`Result` にした方がよいでしょう。
**「呼び出し側は、この失敗をどう処理したいか？」**から逆算して、戻り値の型を選びましょう。

### エラー型の作法

Stage 04 と [選球眼 02](02_choosing_crates.md) の内容に加えて、ライブラリのエラー型には次のことが求められます。

- `std::error::Error` と `Display` を実装する（`anyhow` などで扱えるようにするため）
- 可能なら `Send + Sync + 'static` にする（別のスレッドや async のタスクに渡せるようにするため。Stage 10・11）
- `String` ではなく型（enum など）で返す（呼び出し側が `match` できるように）

## 4. ⭐ 標準トレイトは「先回りして」実装する

自分の型には、意味が通る範囲で、標準ライブラリの基本的なトレイトを実装しておきましょう。

| トレイト | あると何ができるか |
|---|---|
| `Debug` | `{:?}` で表示できる。`assert_eq!` や `unwrap` のエラーメッセージに必要 |
| `Clone`, `Copy` | 値をコピーできる |
| `PartialEq`, `Eq` | `==` で比べられる。テストの `assert_eq!` に必要 |
| `PartialOrd`, `Ord` | 並べ替えられる。`BTreeMap` のキーにできる |
| `Hash` | `HashMap` のキーにできる |
| `Default` | デフォルト値を作れる |
| `Display` | `{}` で表示できる。ユーザー向けの表示 |
| `From` / `TryFrom` | 他の型から変換できる |

### なぜ「先回り」が必要なのか: 孤児ルール

Rust には **孤児ルール**（orphan rule）という決まりがあります。

> トレイトを型に実装できるのは、**そのトレイトか、その型のどちらかが、自分のクレートで定義されている**場合だけ。

あなたがライブラリ `mylib` で `pub struct Point` を公開し、`Hash` を実装し忘れたとします。`mylib` を使う人は、`Point` を `HashMap` のキーにしたくても、**自分で `impl Hash for Point` を書くことができません**。`Hash` も `Point` も、その人のクレートのものではないからです。
（使う人ができるのは、`struct MyPoint(Point)` という newtype で包んで実装することくらいです。）

このルールがないと、2つのクレートがそれぞれ `impl Hash for Point` を書いた場合に、どちらを使えばよいのか決められなくなります。孤児ルールは、その衝突を防ぐためのものです。
その代わり、**型を定義する側が、必要になりそうなトレイトを先に実装しておく責任**を負います。

### `From` を実装すれば、`Into` は自動で付いてくる

s06 では `impl From<i64> for Money` を書きました。すると `let m: Money = 100.into();` とも書けるようになりました。
標準ライブラリに「`From` を実装している型には、逆向きの `Into` を自動で実装する」という包括的な実装（blanket implementation）があるからです。
そのため、**実装するのは常に `From` の方**です（ツアー01で見た `?` のエラー変換も、`From` を使っていました）。

## 5. Builder パターン: 引数が多いとき

Rust には、引数のデフォルト値も、関数のオーバーロード（同じ名前で引数の違う関数を複数定義すること）もありません。
そのため、設定項目が多い型を作るときは、**Builder パターン**を使うのが定番です。

```rust
let output = std::process::Command::new("ls")
    .arg("-l")
    .current_dir("/tmp")
    .output()?;
```

標準ライブラリの `std::process::Command` や `std::thread::Builder` が、この形をしています。

自分で書くと、次のようになります。

```rust
pub struct ServerConfig { port: u16, threads: usize, timeout_secs: u64 }

pub struct ServerConfigBuilder { port: u16, threads: usize, timeout_secs: u64 }

impl ServerConfigBuilder {
    pub fn new(port: u16) -> Self {               // 必須の項目だけを引数にする
        Self { port, threads: 4, timeout_secs: 30 } // 残りはデフォルト値
    }
    pub fn threads(mut self, n: usize) -> Self { self.threads = n; self }
    pub fn timeout_secs(mut self, s: u64) -> Self { self.timeout_secs = s; self }
    pub fn build(self) -> ServerConfig {
        ServerConfig { port: self.port, threads: self.threads, timeout_secs: self.timeout_secs }
    }
}

let config = ServerConfigBuilder::new(8080).threads(8).build();
```

| 方法 | 長所 | 短所 |
|---|---|---|
| 引数を全部並べる `new(port, threads, timeout)` | 単純 | 引数が増えると、どれが何か分からなくなる。同じ型が並ぶと順番を間違えやすい |
| Builder | 名前付きで設定できる。項目を追加しても既存のコードが壊れない | 書くコードが多い |
| `Default` + 構造体更新構文 `Config { port: 8080, ..Default::default() }` | 短い | フィールドを公開する必要がある（6節の互換性の問題が出る） |

p03 の HTTP サーバの設定などで、使う機会があるはずです。

## 6. ⭐ 公開 API を変えるとき: semver

### semver（セマンティックバージョニング）

クレートのバージョンは `MAJOR.MINOR.PATCH`（例: `1.4.2`）の形をしています。

| 上げる番号 | 意味 |
|---|---|
| MAJOR（`1.4.2` → `2.0.0`） | **互換性のない変更**（使う側のコードが壊れうる） |
| MINOR（`1.4.2` → `1.5.0`） | 互換性を保った機能の追加 |
| PATCH（`1.4.2` → `1.4.3`） | 互換性を保ったバグ修正 |

Cargo は、`Cargo.toml` に `serde = "1.0"` と書かれていれば、`1.x.y` の範囲で最新のものを自動で選びます。**MINOR と PATCH の更新では、コードが壊れないことを信じている**からです。
なお `0.x` のバージョンでは、Cargo は `0.4` → `0.5` を MAJOR の変更として扱います（1.0 未満は、MINOR の番号が MAJOR の代わりになります）。

### 何が「互換性のない変更」なのか

Cargo Book の SemVer Compatibility の章に、詳しい一覧があります。代表的なものを挙げます。

| 変更 | 互換性 | 理由 |
|---|---|---|
| 公開している関数・型を削除する | **壊れる** | 使っているコードがコンパイルできなくなる |
| 関数の引数の型を変える | **壊れる** | 同上 |
| 公開している enum に**バリアントを追加する** | **壊れる** | 使う側の `match` が網羅的でなくなる（s03 の網羅性チェック） |
| 全フィールドが公開されている struct に**フィールドを追加する** | **壊れる** | 使う側が `Point { x: 1, y: 2 }` と書いて作っていた場合、フィールドが足りなくなる |
| トレイトに**デフォルト実装のないメソッドを追加する** | **壊れる** | そのトレイトを実装していた側が、新しいメソッドを実装していない |
| 新しい関数・型を追加する | 互換 | 既存のコードには影響しない |
| トレイトにデフォルト実装付きのメソッドを追加する | 基本的に互換 | （名前の衝突など、まれに壊れることがある） |

**enum にバリアントを足すと壊れる**というのは、s03 で学んだ網羅性チェックの裏返しです。網羅性チェックは、enum を**使う側**にとっては安全網ですが、enum を**公開する側**にとっては足かせになります。

### `#[non_exhaustive]`: 将来の追加に備える

```rust
#[non_exhaustive]
pub enum ConfigError {
    MissingEquals { line: usize },
    EmptyKey { line: usize },
}
```

`#[non_exhaustive]` を付けると、**クレートの外からこの enum を `match` するときに、`_ =>` の腕が必須**になります。そのため、後でバリアントを追加しても、使う側のコードは壊れません。
struct に付けた場合は、クレートの外から `ConfigError { ... }` の形で直接作ることができなくなります（フィールドを追加しても壊れなくなります）。

標準ライブラリの `std::io::ErrorKind` には `#[non_exhaustive]` が付いています。OS のエラーの種類は将来増える可能性があるからです。

| | `#[non_exhaustive]` なし | `#[non_exhaustive]` あり |
|---|---|---|
| 使う側 | 全てのケースを網羅しているか、コンパイラが確かめてくれる | 新しいケースが追加されても、`_` の処理に吸い込まれて気付けない |
| 公開する側 | バリアントを足すと MAJOR バージョンを上げる必要がある | MINOR バージョンで足せる |

**どちらを選ぶかは、「この enum の種類は、今後増えうるか？」で決めます。** Stage 03 の Expression Problem と同じ問いです。

---

## 7. 振り返り: 課題のシグネチャを読み直す

課題で与えられていたシグネチャにも、それぞれ理由がありました。

| 課題 | シグネチャ | なぜこの型なのか | 別の案 |
|---|---|---|---|
| s01 | `sum_to(n: u32) -> u64` | 入力より大きな型で返して、オーバーフローを防ぐ（CS基礎 01） | `checked_` 系で `Option<u32>` を返す |
| s01 | `is_prime(n: u32) -> bool` | 負の数を型で排除する | `i32` を受け取り、負なら `false` を返す |
| s02 | `first_word(s: &str) -> &str` | 新しい文字列を作らず、元の文字列の一部を借りて返す（ゼロコピー） | `String` を返す（元の文字列より長く生きられるが、コピーのコストがかかる） |
| s02 | `exclaim(s: &mut String)` | `push_str` で伸ばす必要があるので、`&mut str` ではなく `&mut String` | 新しい `String` を返す |
| s04 | `parse_config(input: &str) -> Result<Vec<(String, String)>, ConfigError>` | 失敗の理由を enum で区別する。結果は入力から独立して生きられるように所有する | 入力を借りる `Vec<(&str, &str)>`（s07 の `parse_borrowed`） |
| s05 | `make_adder(k: i32) -> impl Fn(i32) -> i32` | クロージャの具体的な型は書けないので `impl Fn` で返す。`Box<dyn Fn>` と違ってヒープを使わない | `Box<dyn Fn(i32) -> i32>`（複数の種類のクロージャを返し分けたいとき） |
| s06 | `total_area(shapes: &[Box<dyn Shape>]) -> f64` | 種類の違う図形を1つの `Vec` に入れるために、トレイトオブジェクトを使う | enum（s03 の版） |
| s07 | `load_config(contents_from_disk: String) -> HashMap<String, String>` | 関数の中で作った（または受け取った）データから借りた値は返せないので、所有する版を返す | `parse_borrowed` のように、呼び出し側に元データを持たせ続ける |
| s08 | `escape_html(s: &str) -> Cow<'_, str>` | 変更が不要なときは借用のまま返し、必要なときだけ新しい `String` を作る | 常に `String` を返す（単純だが、毎回コピーする） |
| s09 | `greeting(clock: &impl Clock)` | 時計をトレイトで受け取り、テストでは偽物の時計を渡せるようにする（依存の注入） | 関数の中で直接 `SystemTime::now()` を呼ぶ（テストできない） |
| s10 | `parallel_map(data: &[u64], n_threads: usize, f: fn(u64) -> u64)` | `thread::scope` を使えば、借用したデータをスレッドに渡せる | `Vec<u64>` を所有権ごと受け取る（`parallel_sum` の形） |
| s11 | `total_concurrent(items: Vec<String>)` | `tokio::spawn` するタスクは `'static` でなければならないので、借用ではなく所有するデータが必要 | `&[&str]` を受け取り、中で `to_string()` する |

---

## 問い（`answers/judgment.md` へ）

1. s03 の `Account::new(owner: &str)` を、あなたがライブラリとして公開するなら、どのシグネチャにしますか？ 2節の表の3つの案から選び、理由を答えてください。
2. s03 の `Account::withdraw` を `Result` を返す形に設計し直してください。エラー型も定義しましょう。その enum に `#[non_exhaustive]` を付けますか？
3. 孤児ルールがなかったら、どんな問題が起きるでしょうか？ 具体的な2つのクレートを想像して説明してください。
4. あなたが公開しているライブラリ（バージョン `1.3.0`）で、次の変更をするとき、次のバージョンはいくつにすべきですか？
   (a) 公開している関数のバグを直した (b) 新しい関数を追加した (c) 公開している enum にバリアントを追加した（`#[non_exhaustive]` なし） (d) 関数の引数を `&String` から `&str` に変えた
5. (d) は、呼び出し側のコードを本当に壊すでしょうか？ 考えてから、実際に試して確かめてください。
