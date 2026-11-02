//! ## 課題 02: マクロでコードを生成する
//!
//! マクロは式だけでなく、**型定義や impl ブロック（アイテム）** も生成できます。
//! 「同じ形のボイラープレートを何度も書く」場面で威力を発揮します。
//!
//! この課題のマクロは、パターン（入力の形）は書いてあります。
//! 展開結果の中の `$crate::__todo()` を、繰り返し `$( ... )*` を使った本物のコードに置き換えてください。
//!
//! 便利な組み込みマクロ:
//! - `stringify!(Red)` → `"Red"`（トークンを文字列リテラルにする）
//! - `concat!("a", "b")` → `"ab"`

/// 「文字列と相互変換できる enum」を定義するマクロ。
///
/// ```text
/// enum_str! {
///     pub enum Color { Red, Green, Blue }
/// }
/// ```
///
/// と書くと、次のものが生成される:
/// - `enum Color { Red, Green, Blue }`（`Debug, Clone, Copy, PartialEq, Eq` を derive）
/// - `Color::as_str(&self) -> &'static str` … `Color::Red.as_str() == "Red"`
/// - `Color::all() -> Vec<Color>` … 定義した順に全バリアント
/// - `impl std::str::FromStr for Color` … `"Green".parse::<Color>() == Ok(Color::Green)`。
///   知らない文字列なら `Err(format!("unknown Color: {s}"))`（`Color` の部分は enum の名前）
///
/// これを手で書くと、バリアントを 1 つ足すたびに 3 か所を直す必要があります。
/// マクロなら 1 か所で済み、書き忘れも起きません。
#[macro_export]
macro_rules! enum_str {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        $vis enum $name {
            $($variant),*
        }

        impl $name {
            pub fn as_str(&self) -> &'static str {
                // TODO: match self { $name::Red => "Red", ... } を繰り返しで生成する
                $crate::__todo()
            }

            pub fn all() -> ::std::vec::Vec<$name> {
                // TODO
                $crate::__todo()
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = ::std::string::String;

            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                // TODO: エラーメッセージの enum 名にも stringify! が使える
                $crate::__todo()
            }
        }
    };
}

/// 「値をラップする型」(newtype) を作るマクロ。
///
/// ```text
/// newtype!(UserId: u64, "user-");
/// ```
///
/// と書くと:
/// - `pub struct UserId(pub u64);`（`Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash` を derive）
/// - `impl From<u64> for UserId`
/// - `impl Display for UserId` … 接頭辞を付けて表示する。`UserId(42)` → `"user-42"`
///
/// なぜ newtype が大事なのかは README を参照（`UserId` と `OrderId` を取り違えない）。
#[macro_export]
macro_rules! newtype {
    ($name:ident : $inner:ty, $prefix:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pub $inner);

        impl ::std::convert::From<$inner> for $name {
            fn from(v: $inner) -> Self {
                // TODO
                $crate::__todo()
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                // TODO
                $crate::__todo()
            }
        }
    };
}

#[cfg(test)]
mod tests {
    enum_str! {
        /// 信号の色（ドキュメントコメントも渡せることを確認する）
        pub enum Color { Red, Green, Blue }
    }

    enum_str! {
        enum Level { Low, High, }
    }

    newtype!(UserId: u64, "user-");
    newtype!(OrderId: u32, "order#");

    #[test]
    fn test_as_str() {
        assert_eq!(Color::Red.as_str(), "Red");
        assert_eq!(Color::Blue.as_str(), "Blue");
        assert_eq!(Level::High.as_str(), "High");
    }

    #[test]
    fn test_all() {
        assert_eq!(Color::all(), vec![Color::Red, Color::Green, Color::Blue]);
        assert_eq!(Level::all(), vec![Level::Low, Level::High]);
    }

    #[test]
    fn test_from_str() {
        assert_eq!("Green".parse::<Color>(), Ok(Color::Green));
        assert_eq!(
            "Purple".parse::<Color>(),
            Err("unknown Color: Purple".to_string())
        );
        assert_eq!(
            "low".parse::<Level>(),
            Err("unknown Level: low".to_string())
        );
    }

    #[test]
    fn test_round_trip() {
        for c in Color::all() {
            assert_eq!(c.as_str().parse::<Color>(), Ok(c));
        }
    }

    #[test]
    fn test_newtype() {
        let u = UserId::from(42);
        assert_eq!(u, UserId(42));
        assert_eq!(u.to_string(), "user-42");
        assert_eq!(OrderId::from(7).to_string(), "order#7");
        assert!(UserId(1) < UserId(2));
    }
}
