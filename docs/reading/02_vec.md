# ツアー02: `Vec<T>` の内部構造と push

> **出典**: rustc 1.95.0 の rust-src
> - `library/alloc/src/vec/mod.rs`
> - `library/alloc/src/raw_vec/mod.rs`
>
> **つながるところ**: Stage 02（所有権）、CS基礎 03（スタックとヒープ）、CS基礎 06（計算量）

CS基礎 03 では、`Vec<i32>` が「ポインタ + 長さ + 容量」の 24 バイトだと実験で確かめました。
CS基礎 06 では、push すると容量が 4 → 8 → 16 … と2倍ずつ伸び、そのたびに中身が「移動した」ことも見ました。
このツアーでは、それが実際にどう書かれているのかを読みます。

---

## 1. `Vec` は3層構造になっている

`alloc/src/vec/mod.rs` で `pub struct Vec<T` を検索してください。

```rust
pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
    buf: RawVec<T, A>,
    len: usize,
}
```

`Vec` が直接持っているのは、`buf`（`RawVec`）と `len`（長さ）だけです。ポインタと容量は `RawVec` の中にあります。

`alloc/src/raw_vec/mod.rs` を開いてください。

```rust
pub(crate) struct RawVec<T, A: Allocator = Global> {
    inner: RawVecInner<A>,
    _marker: PhantomData<T>,
}

/// Like a `RawVec`, but only generic over the allocator, not the type.
///
/// As such, all the methods need the layout passed-in as a parameter.
///
/// Having this separation reduces the amount of code we need to monomorphize,
/// as most operations don't need the actual type, just its layout.
#[allow(missing_debug_implementations)]
struct RawVecInner<A: Allocator = Global> {
    ptr: Unique<u8>,
    /// Never used for ZSTs; it's `capacity()`'s responsibility to return usize::MAX in that case.
    ///
    /// # Safety
    ///
    /// `cap` must be in the `0..=isize::MAX` range.
    cap: Cap,
    alloc: A,
}
```

図にすると、こうなります。

```
Vec<T>
├─ buf: RawVec<T>
│   ├─ inner: RawVecInner
│   │   ├─ ptr:   Unique<u8>   ← ヒープへのポインタ（8バイト）
│   │   ├─ cap:   Cap          ← 容量（8バイト）
│   │   └─ alloc: Global       ← アロケータ（0バイト）
│   └─ _marker: PhantomData<T> （0バイト）
└─ len: usize                  ← 長さ（8バイト）
                                  合計 24 バイト
```

### それぞれの層の役割

| 層 | 責任 |
|---|---|
| `Vec<T>` | **要素**の管理。何個入っているか（`len`）、要素の追加・削除、要素の破棄 |
| `RawVec<T>` | **メモリ**の管理。どれだけ確保したか（`cap`）、確保・拡張・解放。要素の中身には一切触れない |
| `RawVecInner` | `RawVec` のうち、型 `T` に依存しない部分 |

`RawVec` のドキュメントコメントには、こう書かれています。

```rust
/// This type does not in anyway inspect the memory that it manages. When dropped it *will*
/// free its memory, but it *won't* try to drop its contents. It is up to the user of `RawVec`
/// to handle the actual things *stored* inside of a `RawVec`.
```

「メモリを解放する責任」と「中身の値を破棄する責任」を、別々の型に分けています。責任を分けると、それぞれの型が守るべきことが単純になります。

### なぜ `RawVecInner` は `T` を持たないのか

コメントにある通り、**単相化（monomorphization）されるコードの量を減らすため**です。

CS基礎 07・10 で学んだように、ジェネリックな関数は、使われた型ごとにコピーが生成されます。`Vec<i32>`, `Vec<String>`, `Vec<User>` を使えば、`RawVec` のメソッドが3回コンパイルされます。
ところが、メモリを確保・拡張する処理に必要なのは、型そのものではなく「要素1個のサイズとアラインメント」（`Layout`）だけです。
そこで、その部分を `T` に依存しない `RawVecInner` に追い出し、`Layout` を引数で渡すようにしました。こうすれば、`RawVecInner` のメソッドは1回しかコンパイルされません。**コンパイル時間とバイナリのサイズを減らすための工夫**です。

### 0 バイトのフィールドたち

実験で確かめた結果がこちらです。

```
Vec<i32>          : 24
Option<Vec<i32>>  : 24
std::alloc::System: 0
```

- **`alloc: Global`**: デフォルトのアロケータ `Global` は、`pub struct Global;` と定義された、フィールドを持たない型です（`alloc/src/alloc.rs`）。中身がないので、サイズは 0 バイトです。上の実験では、同じくフィールドを持たないアロケータ `System` が 0 バイトになっています。
- **`_marker: PhantomData<T>`**: 「この型は `T` を所有しているものとして扱ってほしい」とコンパイラに伝えるための、0 バイトの印です。`RawVecInner` は `T` を持っていないので、これがないと「`Vec<T>` は `T` と無関係な型だ」とコンパイラに誤解されてしまいます。

### `Cap` と、`Option<Vec<T>>` が 24 バイトのままな理由

`Cap` の定義を探すと（`type Cap` を検索）、次のようになっています。

```rust
type Cap = core::num::niche_types::UsizeNoHighBit;
```

そして `core/src/num/niche_types.rs` には、こうあります。

```rust
    pub struct UsizeNoHighBit(usize is 0..=HALF_USIZE);
```

容量は `0..=isize::MAX` の範囲にしかなりえない（最上位ビットが必ず 0）ことを、型で表現しています。
こうしておくと、**範囲外の値（最上位ビットが 1 の値）を `None` の印として使えます**。CS基礎 03 で見た「ニッチ最適化」です。実験の通り、`Option<Vec<i32>>` は `Vec<i32>` と同じ 24 バイトになっています。

---

## 2. ⭐ `push` を読む

`pub fn push(&mut self, value: T)` を検索してください。

```rust
pub fn push(&mut self, value: T) {
    let _ = self.push_mut(value);
}
```

1.95 では、追加した要素への参照を返す `push_mut` が新しく安定化されました（`#[stable(feature = "push_mut", since = "1.95.0")]`）。`push` は、それを呼んで結果を捨てているだけです。`push_mut` を読みましょう。

```rust
pub fn push_mut(&mut self, value: T) -> &mut T {
    // Inform codegen that the length does not change across grow_one().
    let len = self.len;
    // This will panic or abort if we would allocate > isize::MAX bytes
    // or if the length increment would overflow for zero-sized types.
    if len == self.buf.capacity() {
        self.buf.grow_one();
    }
    unsafe {
        let end = self.as_mut_ptr().add(len);
        ptr::write(end, value);
        self.len = len + 1;
        // SAFETY: We just wrote a value to the pointer that will live the lifetime of the reference.
        &mut *end
    }
}
```

処理の流れはこうです。

1. **満杯なら拡張する**: `len == capacity` なら `grow_one()` で領域を広げる
2. **末尾の位置を計算する**: `as_mut_ptr().add(len)` で「先頭から `len` 個先」のアドレスを求める
3. **値を書き込む**: `ptr::write(end, value)` で、そのアドレスに値を書き込む
4. **長さを増やす**: `self.len = len + 1`

### なぜ `unsafe` なのか

- `ptr.add(len)` は、計算したアドレスが確保済みの領域の中にあることを、コンパイラが確認できない
- `ptr::write` は、書き込み先が有効なメモリであることを、コンパイラが確認できない

それでもこのコードが安全なのは、**1. で「`len < capacity`」を保証してから** 2〜3 を行っているからです。この保証を担っているのは、`unsafe` ブロックの外にある `if` 文です。
「`unsafe` の安全性は、`unsafe` ブロックの外のコードにも支えられている」というのは、`unsafe` を読むときの大事なポイントです。

### なぜ代入（`*end = value`）ではなく `ptr::write` なのか

`*end = value` と書くと、Rust は「`end` に入っていた古い値を破棄（drop）してから、新しい値を入れる」という処理をします。
しかし `end` の位置はまだ何も入っていない、初期化されていないメモリです。そこにあるゴミを「古い値」として drop すると、未定義動作になります。
`ptr::write` は「古い値を drop せずに、ただ書き込む」関数です。

## 3. ⭐ 容量の伸ばし方: `grow_one` → `grow_amortized`

`RawVec` の `grow_one` を見てください。

```rust
    /// A specialized version of `self.reserve(len, 1)` which requires the
    /// caller to ensure `len == self.capacity()`.
    #[cfg(not(no_global_oom_handling))]
    #[inline(never)]
    pub(crate) fn grow_one(&mut self) {
        // SAFETY: All calls on self.inner pass T::LAYOUT as the elem_layout
        unsafe { self.inner.grow_one(T::LAYOUT) }
    }
```

`#[inline(never)]` に注目してください。`push` 自体は `#[inline]` で呼び出し元に展開されますが、**めったに通らない拡張の処理は展開されません**。ツアー01の `unwrap_failed` と同じ考え方です。

`RawVecInner::grow_one` は `grow_amortized` を呼びます。ここが容量の伸ばし方を決めている部分です。

```rust
    unsafe fn grow_amortized(
        &mut self,
        len: usize,
        additional: usize,
        elem_layout: Layout,
    ) -> Result<(), TryReserveError> {
        // This is ensured by the calling contexts.
        debug_assert!(additional > 0);

        if elem_layout.size() == 0 {
            // Since we return a capacity of `usize::MAX` when `elem_size` is
            // 0, getting to here necessarily means the `RawVec` is overfull.
            return Err(CapacityOverflow.into());
        }

        // Nothing we can really do about these checks, sadly.
        let required_cap = len.checked_add(additional).ok_or(CapacityOverflow)?;

        // This guarantees exponential growth. The doubling cannot overflow
        // because `cap <= isize::MAX` and the type of `cap` is `usize`.
        let cap = cmp::max(self.cap.as_inner() * 2, required_cap);
        let cap = cmp::max(min_non_zero_cap(elem_layout.size()), cap);

        // SAFETY:
        // - cap >= len + additional
        // - other preconditions passed to caller
        let ptr = unsafe { self.finish_grow(cap, elem_layout)? };

        // SAFETY: `finish_grow` would have failed if `cap > isize::MAX`
        unsafe { self.set_ptr_and_cap(ptr, cap) };
        Ok(())
    }
```

ここまでのツアーで学んだことが、たくさん出てきます。

- `len.checked_add(additional)` — **CS基礎 01 のオーバーフロー対策**。溢れたら `None`
- `.ok_or(CapacityOverflow)?` — **ツアー01の `ok_or` と `?`**
- `self.cap.as_inner() * 2` — **2倍に伸ばす**。コメントにある通り、`cap` は `isize::MAX` 以下なので、2倍しても `usize` から溢れません。前の節で見た `Cap` 型の範囲の制約が、ここで効いています

新しい容量は、次の3つの最大値です。

1. 今の容量の **2倍**
2. 必要な容量（`len + additional`）
3. 最小の容量（`min_non_zero_cap`）

### 最小の容量

```rust
// Tiny Vecs are dumb. Skip to:
// - 8 if the element size is 1, because any heap allocator is likely
//   to round up a request of less than 8 bytes to at least 8 bytes.
// - 4 if elements are moderate-sized (<= 1 KiB).
// - 1 otherwise, to avoid wasting too much space for very short Vecs.
const fn min_non_zero_cap(size: usize) -> usize {
    if size == 1 {
        8
    } else if size <= 1024 {
        4
    } else {
        1
    }
}
```

「小さすぎる Vec は無駄（dumb）」というコメントが付いています。容量 1 → 2 → 4 と細かく伸ばすと、そのたびにメモリを確保し直すコストがかかるからです。

実験で確かめてみましょう。

```
Vec<u8> 最初の push 後の capacity : 8       ← 要素が1バイト → 8
Vec<[u8;2000]> 最初の push 後     : 1       ← 要素が1KiBを超える → 1
```

CS基礎 06 の `vec_growth` の実験で、`Vec<u64>` が `cap=4` から始まっていたのは、`u64` が 8 バイト（1 KiB 以下）だからでした。

### 実際の確保: `finish_grow`

```rust
        let memory = if let Some((ptr, old_layout)) = unsafe { self.current_memory(elem_layout) } {
            // FIXME(const-hack): switch to `debug_assert_eq`
            debug_assert!(old_layout.align() == new_layout.align());
            unsafe {
                // The allocator checks for alignment equality
                hint::assert_unchecked(old_layout.align() == new_layout.align());
                self.alloc.grow(ptr, old_layout, new_layout)
            }
        } else {
            self.alloc.allocate(new_layout)
        };
```

- まだ何も確保していなければ `allocate`（新しく確保する）
- すでに確保していれば `grow`（今の領域を大きくする）

`grow` は、今の場所のすぐ後ろが空いていれば、その場で領域を広げます。空いていなければ、**別の場所に新しい領域を確保して、中身をコピーし、古い領域を解放します**。
CS基礎 06 の実験で「移動した」と表示されたのは、後者が起きたときです。そしてこれが、**借用中の `Vec` に push できない理由**でした（CS基礎 04）。

## 4. ⭐ なぜ「2倍」なら償却 O(1) になるのか

容量を 2 倍ずつ伸ばす場合、要素を n 個 push するまでに起きるコピーの回数を数えてみましょう。容量が 4, 8, 16, …, n と伸びるとき、その時点で入っている要素をコピーします。

```
コピーする要素の数 = 4 + 8 + 16 + ... + n/2  <  n
```

等比数列の和なので、合計は n 未満です。**n 回 push しても、コピーは合計 n 回未満**。1回あたりに直すと、平均 1 回未満のコピーで済みます。これが**償却 O(1)** です。

もし「毎回 10 個ずつ」伸ばしたらどうなるでしょうか。

```
コピーする要素の数 = 10 + 20 + 30 + ... + n  ≈  n² / 20
```

こちらは n² に比例します。n が 10 倍になると、コピーは 100 倍になります。
**「一定量ずつ」ではなく「一定倍ずつ」伸ばすことが、償却 O(1) の鍵**です。

ただし、その代わりに払っているものがあります。最悪の場合、確保した容量の約半分が使われずに空いています。**メモリを多めに使うことで、時間を買っている**わけです。
最終的な要素数が分かっているなら、`Vec::with_capacity(n)` で最初に確保すれば、この無駄もコピーもなくせます。

## 5. `pop` と `Drop`: 取り出しと後片付け

```rust
pub fn pop(&mut self) -> Option<T> {
    if self.len == 0 {
        None
    } else {
        unsafe {
            self.len -= 1;
            core::hint::assert_unchecked(self.len < self.capacity());
            Some(ptr::read(self.as_ptr().add(self.len())))
        }
    }
}
```

`ptr::read` は、`ptr::write` の逆です。メモリから値を**ビットごとコピーして取り出します**。
取り出したあとも、メモリには同じビット列が残っています。しかし `len` を先に減らしているので、その位置は「もう使われていない場所」として扱われ、二度と読まれたり drop されたりしません。**所有権が、`Vec` から戻り値に移ったことになります。**

`Vec` が破棄されるときの処理は、`Drop for Vec<T, A>` を検索してください。

```rust
unsafe impl<#[may_dangle] T, A: Allocator> Drop for Vec<T, A> {
    fn drop(&mut self) {
        unsafe {
            // use drop for [T]
            // use a raw slice to refer to the elements of the vector as weakest necessary type;
            // could avoid questions of validity in certain cases
            ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.as_mut_ptr(), self.len))
        }
        // RawVec handles deallocation
    }
}
```

`Vec` は、**`len` 個の要素だけ**を drop します（容量いっぱいではありません。残りは未初期化なので）。
メモリの解放は「`RawVec handles deallocation`」とある通り、`RawVec` の `Drop` が担当します。

```rust
unsafe impl<#[may_dangle] T, A: Allocator> Drop for RawVec<T, A> {
    /// Frees the memory owned by the `RawVec` *without* trying to drop its contents.
    fn drop(&mut self) {
        // SAFETY: We are in a Drop impl, self.inner will not be used again.
        unsafe { self.inner.deallocate(T::LAYOUT) }
    }
}
```

`Vec<String>` が破棄されるときの順番は、次の通りです。

1. `Vec` の `drop`: 各 `String` を drop する → 各 `String` の文字データ（ヒープ）が解放される
2. フィールド `buf` が破棄される → `RawVec` の `drop`: `String` 本体が並んでいた領域が解放される

1節で見た「要素の責任」と「メモリの責任」の分担が、そのまま後片付けの順番になっています。

## 6. `Deref`: `Vec` がスライスのメソッドを全部使える理由

```rust
impl<T, A: Allocator> ops::Deref for Vec<T, A> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}
```

`Vec<T>` は `Deref<Target = [T]>` を実装しています。そのため、

- `v.iter()`, `v.contains(&x)`, `v.sort()`, `v.first()` などは、**実は `Vec` ではなくスライス `[T]` のメソッド**です。自動で `&[T]` に変換されてから呼ばれています
- Stage 02 で「`&Vec<i32>` より `&[i32]` で受け取る」と学びましたが、`&Vec<i32>` を `&[i32]` の引数に渡せるのは、この `Deref` のおかげです（**Deref 型強制**、deref coercion）

`Vec` に書かれているのは、長さが変わる操作（`push`, `pop`, `insert` など）だけです。長さが変わらない操作は、スライス側に一度だけ書かれ、`Vec` も配列も共有しています。

---

## まとめ

| 学んだこと | ソースのどこに現れていたか |
|---|---|
| `Vec` は 24 バイト | `Vec { buf, len }` + `RawVecInner { ptr, cap, alloc }`。`alloc` と `PhantomData` は 0 バイト |
| `Option<Vec>` も 24 バイト | `Cap` が `0..=isize::MAX` に制限された型（ニッチ最適化） |
| push は償却 O(1) | `grow_amortized` の `cap * 2` |
| 最初の容量は要素の大きさで変わる | `min_non_zero_cap` |
| push すると中身が移動することがある | `finish_grow` の `alloc.grow` |
| 責任の分離 | `Vec` = 要素、`RawVec` = メモリ |
| `unsafe` の閉じ込め方 | 外側の `if` で条件を保証し、内側で `unsafe` を使い、SAFETY コメントで根拠を書く |

---

## 問い（`answers/reading.md` へ）

1. `push_mut` で、`self.len = len + 1` を `ptr::write` より**前**に書いたとします。`ptr::write` の途中で何かが起きた場合に問題になりうるのは、どんな状況でしょうか？（ヒント: 今回の `ptr::write` は panic しませんが、もし値の書き込みの途中で panic する処理があったら？）
2. 容量を「1.5倍ずつ」伸ばす実装もあります（他の言語の標準ライブラリで採用例があります）。2倍と比べて、何が良くなり、何が悪くなりますか？
3. `Vec::pop` が返す値と、`Vec` の中に残っているビット列は同じです。それでも二重解放にならないのはなぜですか？
4. `Vec<T>` に `len()` メソッドが直接実装されているかを、ソースで確かめてください。スライス `[T]` にも `len()` があるのに、なぜ `Vec` にも書かれているのだと思いますか？
5. `alloc/src/vec/mod.rs` で `pub fn insert(` を探して読み、CS基礎 06 で「先頭への挿入は O(n)」だった理由を、コードで説明してください。
