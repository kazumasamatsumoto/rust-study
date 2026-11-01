//! ## 課題 03: トレイトオブジェクトと Expression Problem
//!
//! Stage 03 では Shape を **enum** で作りました。今回は同じものを **trait** で作ります。
//! 解き終わったら、README の「Expression Problem をもう一度」を読んで、
//! 2つの設計を比べてください。
//!
//! 今回は Stage 03 になかった2つの変更を加えます。
//! - 新しい**種類**: 正六角形 (`Hexagon`)
//! - 新しい**操作**: 周の長さ (`perimeter`)
//!
//! どちらの変更が楽で、どちらが大変だったかを覚えておいてください。

use std::collections::BTreeMap;
use std::f64::consts::PI;

pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn name(&self) -> &'static str;

    /// "<name> (area 12.57)" のように、面積を小数点以下2桁で表示する。
    fn describe(&self) -> String {
        todo!()
    }
}

pub struct Circle {
    pub radius: f64,
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

/// 3辺の長さ
pub struct Triangle {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

/// 正六角形。面積は (3√3 / 2) × side²
pub struct Hexagon {
    pub side: f64,
}

// name() はそれぞれ "circle", "rectangle", "triangle", "hexagon" を返すこと。
// 三角形の面積は Stage 03 と同じくヘロンの公式です。

impl Shape for Circle {
    fn area(&self) -> f64 {
        todo!()
    }

    fn perimeter(&self) -> f64 {
        todo!()
    }

    fn name(&self) -> &'static str {
        todo!()
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        todo!()
    }

    fn perimeter(&self) -> f64 {
        todo!()
    }

    fn name(&self) -> &'static str {
        todo!()
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        todo!()
    }

    fn perimeter(&self) -> f64 {
        todo!()
    }

    fn name(&self) -> &'static str {
        todo!()
    }
}

impl Shape for Hexagon {
    fn area(&self) -> f64 {
        todo!()
    }

    fn perimeter(&self) -> f64 {
        todo!()
    }

    fn name(&self) -> &'static str {
        todo!()
    }
}

/// 全ての図形の面積の合計
pub fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    todo!()
}

/// 面積が最大の図形を返す。空なら None。
///
/// ヒント: f64 は Ord ではないので `max_by_key` は使えない（CS基礎 02）。
/// `max_by` と `f64::total_cmp` を使おう。`Box<dyn Shape>` から `&dyn Shape` を
/// 取り出すには `.as_ref()` を使う。
pub fn largest(shapes: &[Box<dyn Shape>]) -> Option<&dyn Shape> {
    todo!()
}

/// 種類ごとの個数を数える（Stage 05 の復習）。
pub fn count_by_name(shapes: &[Box<dyn Shape>]) -> BTreeMap<&'static str, usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    fn sample() -> Vec<Box<dyn Shape>> {
        vec![
            Box::new(Circle { radius: 1.0 }),
            Box::new(Rectangle {
                width: 2.0,
                height: 3.0,
            }),
            Box::new(Triangle {
                a: 3.0,
                b: 4.0,
                c: 5.0,
            }),
            Box::new(Hexagon { side: 2.0 }),
            Box::new(Circle { radius: 0.5 }),
        ]
    }

    #[test]
    fn test_area_and_perimeter() {
        let s = sample();
        assert!(approx(s[0].area(), PI));
        assert!(approx(s[0].perimeter(), 2.0 * PI));
        assert!(approx(s[1].area(), 6.0));
        assert!(approx(s[1].perimeter(), 10.0));
        assert!(approx(s[2].area(), 6.0));
        assert!(approx(s[2].perimeter(), 12.0));
        assert!(approx(s[3].area(), 6.0 * 3f64.sqrt()));
        assert!(approx(s[3].perimeter(), 12.0));
    }

    #[test]
    fn test_describe() {
        assert_eq!(Circle { radius: 2.0 }.describe(), "circle (area 12.57)");
        assert_eq!(Hexagon { side: 1.0 }.describe(), "hexagon (area 2.60)");
    }

    #[test]
    fn test_total_and_largest() {
        let s = sample();
        let expected = PI + 6.0 + 6.0 + 6.0 * 3f64.sqrt() + PI * 0.25;
        assert!(approx(total_area(&s), expected));
        assert_eq!(largest(&s).map(|x| x.name()), Some("hexagon"));
        assert!(largest(&[]).is_none());
    }

    #[test]
    fn test_count_by_name() {
        let c = count_by_name(&sample());
        let expected: BTreeMap<&str, usize> = [
            ("circle", 2),
            ("hexagon", 1),
            ("rectangle", 1),
            ("triangle", 1),
        ]
        .into_iter()
        .collect();
        assert_eq!(c, expected);
    }
}
