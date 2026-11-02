//! アロケーション（ヒープの確保）の回数を数える仕組み。課題ではなく、テストのための道具です。
//!
//! Rust では、`#[global_allocator]` を使ってプログラム全体のアロケータを差し替えられます。
//! ここでは、本物のアロケータ（`System`）に処理を任せつつ、呼ばれた回数だけを数えています。
//!
//! 回数は**スレッドごと**に数えています。`cargo test` は複数のテストを別々のスレッドで
//! 同時に実行するので、全体で 1 つのカウンタを使うと、他のテストのアロケーションまで
//! 数えてしまい、結果が毎回変わってしまうからです。
//!
//! `unsafe` が出てきますが、これは `GlobalAlloc` トレイトが「メモリを正しく扱うことを
//! 実装する側が保証する」約束になっているためです（Stage 13）。ここでは System に
//! そのまま処理を渡しているだけなので、その約束は System が守っています。

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

thread_local! {
    static COUNT: Cell<usize> = const { Cell::new(0) };
}

/// 確保（alloc / realloc）のたびに、そのスレッドのカウンタを 1 増やすアロケータ。
pub struct CountingAlloc;

fn bump() {
    // スレッドの終了処理中はスレッドローカル変数が使えないことがあるので、try_with を使う
    let _ = COUNT.try_with(|c| c.set(c.get() + 1));
}

unsafe impl GlobalAlloc for CountingAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        bump();
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        bump();
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // Vec が伸びるとき（CS基礎 06 の「容量が足りなくなったら確保し直す」）はここが呼ばれる
        bump();
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

/// `f` を実行して、その間に（このスレッドで）起きたアロケーションの回数を返す。
///
/// `CountingAlloc` が `#[global_allocator]` として登録されていないプログラムでは、常に 0 になる。
pub fn count_allocs<R>(f: impl FnOnce() -> R) -> (R, usize) {
    let before = COUNT.with(|c| c.get());
    let result = f();
    let after = COUNT.with(|c| c.get());
    (result, after - before)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_allocations() {
        let (_, n) = count_allocs(|| vec![1u8; 10]);
        assert_eq!(n, 1);
        let (_, n) = count_allocs(|| 1 + 1);
        assert_eq!(n, 0);
        let (_, n) = count_allocs(|| {
            let mut v = Vec::new();
            for i in 0..100 {
                v.push(i); // 容量が 0→4→8→16→32→64→128 と伸びるたびに数えられる
            }
            v
        });
        assert!(n >= 5, "Vec の再確保が数えられていない: {n}");
    }
}
