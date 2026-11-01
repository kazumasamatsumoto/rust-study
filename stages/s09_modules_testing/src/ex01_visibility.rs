//! ## 課題 01: モジュールと可視性
//!
//! Rust では、何も付けなければ**全て非公開**です。公開したいものにだけ `pub` を付けます。
//!
//! | 書き方 | 見える範囲 |
//! |---|---|
//! | (なし) | 同じモジュールとその子モジュール |
//! | `pub(super)` | 親モジュールまで |
//! | `pub(crate)` | 同じクレートの中 |
//! | `pub` | どこからでも |
//!
//! この課題では、`todo!()` を埋めるだけでなく、**どれを公開すべきか**も考えます。
//! `tests/public_api.rs`（統合テスト）は、クレートの外から `pub` なものだけを使います。

/// 温度を表すモジュール
pub mod temperature {
    /// 摂氏温度。**絶対零度 (-273.15℃) 未満の値は作れない**。
    ///
    /// フィールドを非公開にしているので、`Celsius(-500.0)` のように外から直接は作れません。
    /// 必ず `new` を通るので、不正な値が存在しないことを保証できます。
    ///
    /// ```
    /// use s09_modules_testing::Celsius;
    ///
    /// let boiling = Celsius::new(100.0).unwrap();
    /// assert_eq!(boiling.to_fahrenheit(), 212.0);
    /// assert!(Celsius::new(-300.0).is_none());
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    pub struct Celsius(f64);

    pub const ABSOLUTE_ZERO: f64 = -273.15;

    impl Celsius {
        /// 絶対零度未満、または NaN なら None。
        pub fn new(value: f64) -> Option<Self> {
            todo!()
        }

        pub fn value(self) -> f64 {
            todo!()
        }

        pub fn to_fahrenheit(self) -> f64 {
            todo!()
        }
    }
}

/// 統計を計算するモジュール
pub mod stats {
    use super::temperature::Celsius;

    /// 平均気温。空なら None。
    pub fn mean(temps: &[Celsius]) -> Option<Celsius> {
        todo!()
    }

    /// 最高気温と最低気温の差。空なら None。
    pub fn range(temps: &[Celsius]) -> Option<f64> {
        todo!()
    }

    /// 内部でだけ使う補助関数。クレートの外には見せない。
    ///
    /// 考えてみよう: この関数は `pub` にすべきか？
    /// 公開すると、利用者がこの関数に依存し始め、後から変更できなくなる。
    mod helpers {
        use super::super::temperature::Celsius;

        pub(super) fn values(temps: &[Celsius]) -> impl Iterator<Item = f64> + '_ {
            temps.iter().map(|t| t.value())
        }
    }

    #[cfg(test)]
    mod tests {
        // ユニットテストは同じファイルの子モジュールなので、非公開の helpers も使える
        use super::helpers::values;
        use super::*;

        #[test]
        fn test_helpers_values() {
            let t = [Celsius::new(1.0).unwrap(), Celsius::new(2.5).unwrap()];
            assert_eq!(values(&t).collect::<Vec<_>>(), vec![1.0, 2.5]);
        }

        #[test]
        fn test_mean() {
            let t = [
                Celsius::new(10.0).unwrap(),
                Celsius::new(20.0).unwrap(),
                Celsius::new(30.0).unwrap(),
            ];
            assert_eq!(mean(&t), Celsius::new(20.0));
            assert_eq!(mean(&[]), None);
        }

        #[test]
        fn test_range() {
            let t = [
                Celsius::new(-5.0).unwrap(),
                Celsius::new(12.5).unwrap(),
                Celsius::new(3.0).unwrap(),
            ];
            assert_eq!(range(&t), Some(17.5));
            assert_eq!(range(&[]), None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::temperature::*;

    #[test]
    fn test_new_rejects_below_absolute_zero() {
        assert!(Celsius::new(ABSOLUTE_ZERO).is_some());
        assert!(Celsius::new(-273.16).is_none());
        assert!(Celsius::new(f64::NAN).is_none());
    }

    #[test]
    fn test_conversion() {
        assert_eq!(Celsius::new(0.0).unwrap().to_fahrenheit(), 32.0);
        assert_eq!(Celsius::new(-40.0).unwrap().value(), -40.0);
    }
}
