//! ## 課題 03: テストを書く
//!
//! この課題は逆向きです。**実装はもう書いてあります。あなたが書くのはテストです。**
//!
//! ⚠️ ここにある実装には、**全部で2つのバグ**が潜んでいます。
//! 仕様（doc コメント）をよく読み、テストを書いてバグを見つけ、実装を直してください。
//!
//! 進め方:
//! 1. 下の `mod tests` にある空のテスト関数を埋める（コメントに何を確かめるか書いてある）
//! 2. 必要だと思うテストを自分で追加する
//! 3. 失敗するテストが見つかったら、それがバグ。実装を直す
//! 4. 各関数の doc コメントに、doc テスト（` ``` ` で囲んだ使用例）を1つ以上書く

/// タイトルを URL 用の文字列（スラッグ）に変換する。
///
/// 仕様:
/// - ASCII の英字は小文字にする
/// - ASCII の英数字以外の文字（空白・記号・日本語など）は `-` に置き換える
/// - `-` が連続したら1つにまとめる
/// - 先頭と末尾の `-` は取り除く
///
/// 例: `"Hello, Rust World!"` → `"hello-rust-world"`
pub fn slugify(title: &str) -> String {
    let mut out = String::new();
    for c in title.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
        } else if !out.ends_with('-') {
            out.push('-');
        }
    }
    out.trim_start_matches('-').to_string()
}

/// 期間の解析で起きるエラー
#[derive(Debug, PartialEq)]
pub enum DurationError {
    /// 入力が空
    Empty,
    /// 数字のあとに単位がない（例: "10"）
    MissingUnit,
    /// 知らない単位（例: "10x"）
    UnknownUnit(char),
    /// 単位の前に数字がない（例: "h"）
    MissingNumber,
}

/// `"1h30m15s"` のような期間を秒数に変換する。
///
/// 仕様:
/// - 単位は `h`（時間）、`m`（分）、`s`（秒）
/// - 組み合わせてよい: `"1h30m"` → 5400
/// - 空文字列はエラー（`DurationError::Empty`）
/// - 前後の空白は無視する
pub fn parse_duration(input: &str) -> Result<u64, DurationError> {
    let input = input.trim();
    let mut total = 0u64;
    let mut number: Option<u64> = None;
    for c in input.chars() {
        if let Some(d) = c.to_digit(10) {
            number = Some(number.unwrap_or(0) * 10 + d as u64);
            continue;
        }
        let n = number.take().ok_or(DurationError::MissingNumber)?;
        let unit = match c {
            'h' => 3600,
            'm' => 60,
            's' => 1,
            other => return Err(DurationError::UnknownUnit(other)),
        };
        total += n * unit;
    }
    if number.is_some() {
        return Err(DurationError::MissingUnit);
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_basic() {
        // TODO: 仕様の例 "Hello, Rust World!" が "hello-rust-world" になることを確かめる
    }

    #[test]
    fn slugify_collapses_hyphens() {
        // TODO: 記号や空白が連続しても `-` が1つになることを確かめる
    }

    #[test]
    fn slugify_edges() {
        // TODO: 先頭・末尾の記号、空文字列、日本語だけの文字列など、端のケースを確かめる
    }

    #[test]
    fn parse_duration_ok() {
        // TODO: "1h30m15s", "45s", " 2m " などが正しい秒数になることを確かめる
    }

    #[test]
    fn parse_duration_errors() {
        // TODO: DurationError の4種類それぞれが返るケースを確かめる
    }
}
