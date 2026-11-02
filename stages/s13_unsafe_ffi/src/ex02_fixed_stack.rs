//! ## 課題 02: unsafe を安全な API で包む — 固定長スタック
//!
//! ヒープを使わず、最大 `N` 個の要素をスタック領域に置けるスタックを作ります。
//! （実在するクレート `arrayvec` や `heapless` と同じ考え方です。）
//!
//! ### 新しい道具
//!
//! - **const ジェネリクス** `const N: usize`: 型のパラメータとして「数」を受け取る。`FixedStack<i32, 4>` のように使う
//! - **`MaybeUninit<T>`**: 「まだ初期化されていないかもしれない `T`」を置く場所。
//!   `[T; N]` だと、最初から N 個の値で埋める必要があります。`[MaybeUninit<T>; N]` なら、空の箱だけを用意できます
//!   - `MaybeUninit::new(v)` / `slot.write(v)`: 値を入れる（安全）
//!   - `slot.assume_init_read()`: 値を取り出す（unsafe: 初期化済みであることを呼び出し側が保証する）
//!   - `slot.assume_init_ref()`: 参照を得る（unsafe）
//!   - `slot.assume_init_drop()`: その場で drop する（unsafe）
//!
//! ### 不変条件 (invariant)
//!
//! **`buf[0..len]` は全て初期化済み、`buf[len..N]` は全て未初期化。**
//!
//! 全てのメソッドは、この不変条件を前提にしてよく、終了時にこの不変条件を守らなければなりません。
//! SAFETY コメントには「不変条件により〜は初期化済みなので」と書くことになります。
//!
//! フィールドが非公開なのが重要です。外から `len` を書き換えられたら、不変条件が壊れてしまいます。

use std::mem::MaybeUninit;

pub struct FixedStack<T, const N: usize> {
    buf: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> FixedStack<T, N> {
    /// 空のスタックを作る（ここは実装済み）。
    pub fn new() -> Self {
        Self {
            // const ブロックで「未初期化の箱」を N 個並べる
            buf: [const { MaybeUninit::uninit() }; N],
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        N
    }

    /// 末尾に積む。満杯なら、受け取った値をそのまま `Err(value)` で返す
    /// （値を捨てずに呼び出し側に返すのが Rust 流）。
    pub fn push(&mut self, value: T) -> Result<(), T> {
        todo!()
    }

    /// 末尾から取り出す。空なら `None`。
    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }

    /// 末尾の要素への参照。空なら `None`。
    pub fn peek(&self) -> Option<&T> {
        todo!()
    }

    /// 初期化済みの部分 `buf[0..len]` を `&[T]` として見せる。
    ///
    /// ヒント: `self.buf.as_ptr()` は `*const MaybeUninit<T>`。`MaybeUninit<T>` は `T` と
    /// 同じメモリ配置であることが保証されているので、`.cast::<T>()` で `*const T` にできる。
    /// そこから `std::slice::from_raw_parts`。
    pub fn as_slice(&self) -> &[T] {
        todo!()
    }
}

impl<T, const N: usize> Default for FixedStack<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Drop for FixedStack<T, N> {
    fn drop(&mut self) {
        // TODO: 残っている要素 (buf[0..len]) を drop する。
        //
        // MaybeUninit<T> は、中身を自動では drop しません（初期化されているかどうか知らないので）。
        // 何もしないと、String や Vec を入れたときにメモリリークになります。
        //
        // 注意: ここに todo!() を書いてはいけません。他のテストが todo!() で panic して
        // スタックが drop されるときに、もう一度 panic すると、テスト全体が異常終了するからです。
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn test_push_pop_lifo() {
        let mut s: FixedStack<i32, 3> = FixedStack::new();
        assert!(s.is_empty());
        assert_eq!(s.push(1), Ok(()));
        assert_eq!(s.push(2), Ok(()));
        assert_eq!(s.peek(), Some(&2));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
        assert_eq!(s.peek(), None);
    }

    #[test]
    fn test_full_returns_value() {
        let mut s: FixedStack<String, 2> = FixedStack::new();
        s.push("a".to_string()).unwrap();
        s.push("b".to_string()).unwrap();
        assert_eq!(s.push("c".to_string()), Err("c".to_string()));
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn test_as_slice() {
        let mut s: FixedStack<u8, 4> = FixedStack::new();
        assert_eq!(s.as_slice(), &[] as &[u8]);
        s.push(7).unwrap();
        s.push(8).unwrap();
        assert_eq!(s.as_slice(), &[7, 8]);
    }

    #[test]
    fn test_zero_capacity() {
        let mut s: FixedStack<i32, 0> = FixedStack::new();
        assert_eq!(s.push(1), Err(1));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn test_drop_releases_remaining_elements() {
        // Rc::strong_count で「何人が持っているか」を数え、drop されたかを確認する
        let item = Rc::new(42);
        {
            let mut s: FixedStack<Rc<i32>, 4> = FixedStack::new();
            s.push(Rc::clone(&item)).unwrap();
            s.push(Rc::clone(&item)).unwrap();
            s.push(Rc::clone(&item)).unwrap();
            assert_eq!(Rc::strong_count(&item), 4);

            let popped = s.pop().unwrap(); // 1 つは外に取り出す
            drop(popped);
            assert_eq!(Rc::strong_count(&item), 3);
        } // ここで s が drop され、残りの 2 つも drop されるべき
        assert_eq!(Rc::strong_count(&item), 1);
    }
}
