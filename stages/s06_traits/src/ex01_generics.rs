//! ## 課題 01: トレイトの定義とジェネリクス
//!
//! - トレイトを定義して、複数の型に実装する
//! - デフォルト実装と、その上書き
//! - トレイト境界 `T: Trait` と `where` 句
//! - 引数の `impl Trait`

use std::fmt::Display;

/// 自己紹介ができるもの
pub trait Describe {
    /// 名前を返す（必須のメソッド）
    fn name(&self) -> String;

    /// 自己紹介の文を返す。デフォルトでは "This is <name>"。
    fn describe(&self) -> String {
        todo!()
    }
}

pub struct Dog {
    pub name: String,
}

pub struct Robot {
    pub id: u32,
}

impl Describe for Dog {
    /// 犬の名前をそのまま返す。describe はデフォルト実装を使う。
    fn name(&self) -> String {
        todo!()
    }
}

impl Describe for Robot {
    /// "Robot #<id>" を返す。
    fn name(&self) -> String {
        todo!()
    }

    /// デフォルト実装を上書きして "Beep. I am Robot #<id>" を返す。
    /// （name() を再利用すること）
    fn describe(&self) -> String {
        todo!()
    }
}

/// 全ての要素の describe() を集める。
pub fn describe_all<T: Describe>(items: &[T]) -> Vec<String> {
    todo!()
}

/// describe() の結果を大文字にして返す。
/// 引数の `&impl Describe` は `<T: Describe>(x: &T)` の短縮形です。
pub fn shout(x: &impl Describe) -> String {
    todo!()
}

/// 最大の要素を返す。空なら None。
///
/// `Ord` ではなく `PartialOrd` を要求しているので、f64 にも使えます。
/// 考えてみよう: NaN が混ざっていたら、あなたの実装はどう振る舞うか？
pub fn largest<T: PartialOrd + Copy>(v: &[T]) -> Option<T> {
    todo!()
}

/// "label: value" の形式の文字列を作る。
pub fn labeled<T>(label: &str, value: T) -> String
where
    T: Display,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe() {
        let d = Dog {
            name: "Pochi".into(),
        };
        let r = Robot { id: 7 };
        assert_eq!(d.describe(), "This is Pochi");
        assert_eq!(r.name(), "Robot #7");
        assert_eq!(r.describe(), "Beep. I am Robot #7");
    }

    #[test]
    fn test_describe_all_and_shout() {
        let robots = [Robot { id: 1 }, Robot { id: 2 }];
        assert_eq!(
            describe_all(&robots),
            vec!["Beep. I am Robot #1", "Beep. I am Robot #2"]
        );
        let d = Dog {
            name: "Hachi".into(),
        };
        assert_eq!(shout(&d), "THIS IS HACHI");
    }

    #[test]
    fn test_largest() {
        assert_eq!(largest(&[3, 9, 2]), Some(9));
        assert_eq!(largest(&[1.5, -2.0, 0.3]), Some(1.5));
        assert_eq!(largest(&['a', 'z', 'm']), Some('z'));
        assert_eq!(largest::<i32>(&[]), None);
    }

    #[test]
    fn test_labeled() {
        assert_eq!(labeled("age", 30), "age: 30");
        assert_eq!(labeled("ratio", 2.5), "ratio: 2.5");
        assert_eq!(labeled("lang", "Rust"), "lang: Rust");
    }
}
