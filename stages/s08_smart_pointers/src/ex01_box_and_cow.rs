//! ## 課題 01: Box と再帰型 / Cow
//!
//! 自分自身を含む型（再帰型）は、そのままではサイズが決まりません。
//! `Box` でヒープに置くと、フィールドは「ポインタ 8 バイト」になり、サイズが決まります。

use std::borrow::Cow;

/// 数式の木。`Box<Expr>` で子の式を持つ。
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

impl Expr {
    /// 便利なコンストラクタ（テストを読みやすくするため）。
    /// `add` という名前にしないのは、標準の `std::ops::Add` トレイトのメソッドと紛らわしいから
    /// （clippy の `should_implement_trait`）。演算子 `+` で式を組み立てたいなら
    /// `impl std::ops::Add for Expr` を書くのが Rust らしいやり方（Stage 06 の復習）。
    pub fn plus(a: Expr, b: Expr) -> Expr {
        Expr::Add(Box::new(a), Box::new(b))
    }

    pub fn times(a: Expr, b: Expr) -> Expr {
        todo!()
    }

    pub fn negate(a: Expr) -> Expr {
        todo!()
    }

    /// 式を計算する。
    ///
    /// ヒント: `Box<Expr>` は `Deref` を実装しているので、`a.eval()` のように
    /// 中身の `Expr` のメソッドをそのまま呼べる。
    pub fn eval(&self) -> i64 {
        todo!()
    }

    /// 木の深さ。`Num` は 1。
    pub fn depth(&self) -> usize {
        todo!()
    }

    /// 式を文字列にする。全ての二項演算をかっこで囲む。
    /// 例: `(1 + (2 * 3))`, 符号反転は `-(…)` ではなく `-5` や `-(1 + 2)`
    /// つまり Neg の中身が Num ならかっこなし、それ以外ならそのままの表記の前に `-`
    pub fn render(&self) -> String {
        todo!()
    }
}

/// HTML の特殊文字（`<`, `>`, `&`）をエスケープする。
/// `&` → `&amp;`, `<` → `&lt;`, `>` → `&gt;`
///
/// ⭐ エスケープが必要ない文字列（ほとんどの場合）では、**コピーせずに借りたまま**返すこと。
/// `Cow::Borrowed(s)` / `Cow::Owned(string)` を使い分ける。
pub fn escape_html(s: &str) -> Cow<'_, str> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn n(v: i64) -> Expr {
        Expr::Num(v)
    }

    #[test]
    fn test_eval() {
        // 1 + 2 * 3
        let e = Expr::plus(n(1), Expr::times(n(2), n(3)));
        assert_eq!(e.eval(), 7);
        assert_eq!(Expr::negate(Expr::plus(n(1), n(2))).eval(), -3);
    }

    #[test]
    fn test_depth() {
        assert_eq!(n(1).depth(), 1);
        assert_eq!(Expr::plus(n(1), Expr::times(n(2), n(3))).depth(), 3);
        assert_eq!(Expr::negate(n(1)).depth(), 2);
    }

    #[test]
    fn test_render() {
        assert_eq!(
            Expr::plus(n(1), Expr::times(n(2), n(3))).render(),
            "(1 + (2 * 3))"
        );
        assert_eq!(Expr::negate(n(5)).render(), "-5");
        assert_eq!(Expr::negate(Expr::plus(n(1), n(2))).render(), "-(1 + 2)");
    }

    #[test]
    fn test_escape_html() {
        assert_eq!(escape_html("a < b && c"), "a &lt; b &amp;&amp; c");
        assert_eq!(escape_html("<p>"), "&lt;p&gt;");

        let plain = "hello world";
        let result = escape_html(plain);
        assert_eq!(result, "hello world");
        // エスケープ不要なら、コピーせずに借りていること
        assert!(matches!(result, Cow::Borrowed(_)));
        assert!(matches!(escape_html("1 < 2"), Cow::Owned(_)));
    }
}
