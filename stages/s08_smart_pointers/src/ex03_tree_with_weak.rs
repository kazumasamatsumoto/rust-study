//! ## 課題 03: Weak で親を指す木
//!
//! 親は子を `Rc`（所有する参照）で持ち、子は親を `Weak`（所有しない参照）で持つ。
//! 両方を `Rc` にすると循環参照になり、メモリが解放されなくなる（README を参照）。

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node {
    pub name: String,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    /// 親も子もないノードを作る。
    /// ヒント: 何も指していない Weak は `Weak::new()` で作れる。
    pub fn new(name: &str) -> Rc<Node> {
        todo!()
    }

    /// parent の子として child を追加し、child の親を parent にする。
    ///
    /// ヒント: `Rc::downgrade(parent)` で Rc から Weak を作れる。
    pub fn add_child(parent: &Rc<Node>, child: Rc<Node>) {
        todo!()
    }

    /// 親の名前。親がいない、または親がもう解放されていれば None。
    /// ヒント: `Weak::upgrade` は `Option<Rc<T>>` を返す。
    pub fn parent_name(&self) -> Option<String> {
        todo!()
    }

    /// 根からこのノードまでの名前を "/" で繋げたパス。例: "root/usr/bin"
    pub fn path(&self) -> String {
        todo!()
    }

    /// 子孫（自分を含む）の数
    pub fn count(&self) -> usize {
        todo!()
    }

    /// 名前で子を探す（直接の子だけ）
    pub fn child(&self, name: &str) -> Option<Rc<Node>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Rc<Node> {
        let root = Node::new("root");
        let usr = Node::new("usr");
        Node::add_child(&root, Rc::clone(&usr));
        Node::add_child(&usr, Node::new("bin"));
        Node::add_child(&usr, Node::new("lib"));
        Node::add_child(&root, Node::new("etc"));
        root
    }

    #[test]
    fn test_structure() {
        let root = sample();
        assert_eq!(root.count(), 5);
        let usr = root.child("usr").unwrap();
        let bin = usr.child("bin").unwrap();
        assert_eq!(bin.parent_name().as_deref(), Some("usr"));
        assert_eq!(root.parent_name(), None);
        assert_eq!(bin.path(), "root/usr/bin");
        assert_eq!(root.path(), "root");
    }

    #[test]
    fn test_counts() {
        let root = sample();
        let usr = root.child("usr").unwrap();
        // usr を所有しているのは root の children と、ここの変数 usr の2つ
        assert_eq!(Rc::strong_count(&usr), 2);
        // usr を Weak で指しているのは、子の bin と lib
        assert_eq!(Rc::weak_count(&usr), 2);
    }

    #[test]
    fn test_no_leak() {
        let root = sample();
        let bin = root.child("usr").unwrap().child("bin").unwrap();
        drop(root);
        // root と usr は解放された。bin は変数がまだ所有しているので生きている
        assert_eq!(bin.parent_name(), None);
        assert_eq!(bin.path(), "bin");
        assert_eq!(Rc::strong_count(&bin), 1);
    }
}
