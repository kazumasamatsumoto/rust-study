//! ## 課題 02: Rc と RefCell / Drop
//!
//! - `Rc<T>`: 1つの値を複数の場所から所有する。最後の所有者が消えたときに解放される
//! - `RefCell<T>`: `&self` しかなくても中身を書き換えられる（内部可変性）。
//!   借用ルールのチェックは**実行時**に行われ、違反すると panic する
//! - 2つを組み合わせた `Rc<RefCell<T>>` は「共有されていて、書き換えられる」値

use std::cell::RefCell;
use std::rc::Rc;

/// 複数の場所から書き込める共有ログ。
///
/// `handle()` で作ったコピーは、全て同じ `Vec` を指している。
#[derive(Debug)]
pub struct SharedLog {
    entries: Rc<RefCell<Vec<String>>>,
}

impl SharedLog {
    pub fn new() -> Self {
        todo!()
    }

    /// 同じログを指す、新しいハンドルを作る（Vec はコピーしない）
    pub fn handle(&self) -> SharedLog {
        todo!()
    }

    /// ⭐ `&mut self` ではなく `&self` で書き込めることに注目。
    pub fn push(&self, msg: &str) {
        todo!()
    }

    /// 今のログの中身のコピーを返す
    pub fn snapshot(&self) -> Vec<String> {
        todo!()
    }

    /// このログを指しているハンドルの数
    pub fn handle_count(&self) -> usize {
        todo!()
    }
}

impl Default for SharedLog {
    fn default() -> Self {
        Self::new()
    }
}

/// スコープを抜けたときに、ログに "end: <name>" を書き込むガード。
/// 作成時には "start: <name>" を書き込む。
///
/// 「スコープを抜けたら必ず後片付けする」パターン（RAII）の練習。
/// ファイルを閉じる、ロックを外す、計測を終える、などに使われる。
pub struct ScopeGuard {
    name: String,
    log: SharedLog,
}

impl ScopeGuard {
    pub fn new(name: &str, log: &SharedLog) -> Self {
        todo!()
    }
}

impl Drop for ScopeGuard {
    fn drop(&mut self) {
        todo!()
    }
}

/// cell を読み取っている最中に、書き込み用に借りられるかを試す。
/// `RefCell` の実行時チェックがどう働くかを確かめるための関数。
///
/// 手順:
/// 1. `cell.borrow()` で読み取り用に借りる
/// 2. その参照を持ったまま `cell.try_borrow_mut()` を試し、成功したかを記録する
/// 3. 読み取りの参照を手放してから、もう一度 `try_borrow_mut()` を試す
///
/// (1回目が成功したか, 2回目が成功したか) を返す。
pub fn borrow_check_demo(cell: &RefCell<Vec<i32>>) -> (bool, bool) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_log() {
        let log = SharedLog::new();
        let a = log.handle();
        let b = log.handle();
        assert_eq!(log.handle_count(), 3);
        a.push("from a");
        b.push("from b");
        assert_eq!(log.snapshot(), vec!["from a", "from b"]);
        drop(a);
        assert_eq!(log.handle_count(), 2);
    }

    #[test]
    fn test_scope_guard() {
        let log = SharedLog::new();
        {
            let _outer = ScopeGuard::new("outer", &log);
            {
                let _inner = ScopeGuard::new("inner", &log);
                log.push("work");
            }
        }
        assert_eq!(
            log.snapshot(),
            vec![
                "start: outer",
                "start: inner",
                "work",
                "end: inner",
                "end: outer"
            ]
        );
        // ガードが持っていたハンドルも解放されている
        assert_eq!(log.handle_count(), 1);
    }

    #[test]
    fn test_borrow_check_demo() {
        let cell = RefCell::new(vec![1, 2, 3]);
        assert_eq!(borrow_check_demo(&cell), (false, true));
    }
}
