//! ## 課題 02: 列挙型と match
//!
//! Rust の enum は各バリアントにデータを持たせられます（代数的データ型）。
//! `match` は全てのケースを網羅しないとコンパイルエラーになるので安全です。

use std::f64::consts::PI;

#[derive(Debug, PartialEq)]
pub enum Shape {
    Circle {
        radius: f64,
    },
    Rectangle {
        width: f64,
        height: f64,
    },
    /// 3辺の長さ
    Triangle(f64, f64, f64),
}

impl Shape {
    /// 面積。三角形はヘロンの公式: s = (a+b+c)/2, √(s(s-a)(s-b)(s-c))
    pub fn area(&self) -> f64 {
        todo!()
    }

    /// 形の名前を返す: "circle" / "rectangle" / "triangle"
    pub fn name(&self) -> &'static str {
        todo!()
    }
}

/// テキストコマンド
#[derive(Debug, PartialEq)]
pub enum Command {
    /// "move <x> <y>"
    Move { x: i32, y: i32 },
    /// "say <message...>"（スペースを含む残り全部）
    Say(String),
    /// "quit"
    Quit,
}

/// 1行のコマンドを解析する。解析できなければ None。
///
/// ヒント: `s.split_once(' ')`, `str::parse::<i32>()`, `.ok()`
pub fn parse_command(s: &str) -> Option<Command> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn test_area() {
        assert!(approx(Shape::Circle { radius: 1.0 }.area(), PI));
        assert!(approx(
            Shape::Rectangle {
                width: 2.0,
                height: 3.5
            }
            .area(),
            7.0
        ));
        assert!(approx(Shape::Triangle(3.0, 4.0, 5.0).area(), 6.0));
    }

    #[test]
    fn test_name() {
        assert_eq!(Shape::Circle { radius: 1.0 }.name(), "circle");
        assert_eq!(Shape::Triangle(1.0, 1.0, 1.0).name(), "triangle");
    }

    #[test]
    fn test_parse_command() {
        assert_eq!(parse_command("quit"), Some(Command::Quit));
        assert_eq!(
            parse_command("move 3 -4"),
            Some(Command::Move { x: 3, y: -4 })
        );
        assert_eq!(
            parse_command("say hello rust"),
            Some(Command::Say("hello rust".to_string()))
        );
        assert_eq!(parse_command("move 3"), None);
        assert_eq!(parse_command("move a b"), None);
        assert_eq!(parse_command("jump"), None);
    }
}
