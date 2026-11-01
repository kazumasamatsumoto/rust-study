//! # Stage 08: スマートポインタ
//!
//! 「ポインタ + 追加の機能」を持つ型をスマートポインタと呼びます。
//! - `Box<T>`    : ヒープに置く。所有者は1人
//! - `Rc<T>`     : 参照カウントで、所有者を複数にする
//! - `RefCell<T>`: 借用チェックを実行時に行い、`&` からでも書き換えられるようにする
//! - `Weak<T>`   : 所有しない参照。循環参照を断ち切る
//! - `Cow<T>`    : 借りるか所有するかを実行時に選ぶ
//!
//! 解説は `stages/s08_smart_pointers/README.md` を先に読んでください。
//!
//! 参考: The Rust Programming Language 第15章
//! <https://doc.rust-jp.rs/book-ja/ch15-00-smart-pointers.html>

#![allow(unused_variables, unused_imports, dead_code, clippy::ptr_arg)]

pub mod ex01_box_and_cow;
pub mod ex02_rc_refcell;
pub mod ex03_tree_with_weak;
