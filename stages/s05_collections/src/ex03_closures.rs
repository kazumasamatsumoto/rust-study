//! ## 課題 03: クロージャと Fn / FnMut / FnOnce
//!
//! クロージャは「周りの変数を捕まえる（キャプチャする）無名関数」です。
//! 捕まえた変数をどう使うかによって、3つのトレイトのどれを実装するかが決まります。
//!
//! | トレイト | 捕まえた変数を | 呼べる回数 |
//! |---|---|---|
//! | `Fn`     | 読むだけ       | 何回でも |
//! | `FnMut`  | 書き換える     | 何回でも（ただし `mut` が必要） |
//! | `FnOnce` | 消費する（ムーブして手放す） | 1回だけ |

/// f を x に n 回適用する。
/// 例: apply_n(|x| x * 2, 3, 1) -> 8
pub fn apply_n<F: Fn(i32) -> i32>(f: F, n: u32, x: i32) -> i32 {
    todo!()
}

/// 「k を足す関数」を作って返す。
/// 例: make_adder(5)(10) -> 15
///
/// ヒント: 関数が終わった後も k を使い続けるので、`move` が必要です。
pub fn make_adder(k: i32) -> impl Fn(i32) -> i32 {
    // 下の行を書き換えてください。（戻り値の型を決めるために、仮のクロージャを置いています）
    move |x| todo!()
}

/// 呼ぶたびに 1, 2, 3, ... と返すカウンタを作る。
///
/// ヒント: クロージャの**外**に `let mut count = 0;` を用意し、`move` で捕まえます。
/// 捕まえた変数は、クロージャの中に住み続けます（README の「クロージャの正体」）。
pub fn make_counter() -> impl FnMut() -> u32 {
    move || todo!()
}

/// f を適用してから g を適用する関数を作る。
/// 例: compose(|x| x + 1, |x| x * 10)(2) -> 30
pub fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |a| todo!()
}

/// f を最大 max_attempts 回まで呼び、最初に成功した結果を返す。
/// 全て失敗したら、最後のエラーを返す。max_attempts は 1 以上と仮定してよい。
///
/// 考えてみよう: なぜ `Fn` ではなく `FnMut` を受け取るのか？
pub fn retry<T, E>(max_attempts: u32, mut f: impl FnMut() -> Result<T, E>) -> Result<T, E> {
    todo!()
}

/// opt が Some ならその中身を、None なら f() の結果を返す。
/// f は「必要なときだけ」呼ぶこと（`Option::unwrap_or_else` と同じ動き）。
///
/// 考えてみよう: なぜ `FnOnce` で十分なのか？ `Fn` にすると何が困るのか？
pub fn or_compute(opt: Option<String>, f: impl FnOnce() -> String) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn test_apply_n() {
        assert_eq!(apply_n(|x| x * 2, 3, 1), 8);
        assert_eq!(apply_n(|x| x + 1, 0, 5), 5);
    }

    #[test]
    fn test_make_adder() {
        let add5 = make_adder(5);
        assert_eq!(add5(10), 15);
        assert_eq!(add5(-5), 0);
    }

    #[test]
    fn test_make_counter() {
        let mut a = make_counter();
        let mut b = make_counter();
        assert_eq!(a(), 1);
        assert_eq!(a(), 2);
        assert_eq!(b(), 1); // a と b は別々の状態を持つ
        assert_eq!(a(), 3);
    }

    #[test]
    fn test_compose() {
        let f = compose(|x: i32| x + 1, |x: i32| x * 10);
        assert_eq!(f(2), 30);
        let len_then_double = compose(|s: &str| s.len(), |n: usize| n * 2);
        assert_eq!(len_then_double("rust"), 8);
    }

    #[test]
    fn test_retry() {
        let mut calls = 0;
        let r: Result<&str, String> = retry(3, || {
            calls += 1;
            if calls < 3 {
                Err(format!("fail {calls}"))
            } else {
                Ok("ok")
            }
        });
        assert_eq!(r, Ok("ok"));
        assert_eq!(calls, 3);

        let mut calls = 0;
        let r: Result<(), String> = retry(2, || {
            calls += 1;
            Err(format!("fail {calls}"))
        });
        assert_eq!(r, Err("fail 2".to_string()));
        assert_eq!(calls, 2);
    }

    #[test]
    fn test_or_compute_is_lazy() {
        let called = Cell::new(false);
        let r = or_compute(Some("cached".into()), || {
            called.set(true);
            "computed".into()
        });
        assert_eq!(r, "cached");
        assert!(!called.get(), "Some のときは f を呼ばないこと");

        let name = String::from("computed");
        // name をムーブして返すクロージャは FnOnce にしかなれない
        assert_eq!(or_compute(None, move || name), "computed");
    }
}
