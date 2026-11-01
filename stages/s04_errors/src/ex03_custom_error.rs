//! ## 課題 03: 独自エラー型
//!
//! 実務では `String` ではなく、enum でエラーを表現します。
//! - 呼び出し側が `match` でエラーの種類ごとに処理できる
//! - `std::fmt::Display` と `std::error::Error` を実装すると行儀がよい
//!
//! （実務では `thiserror` クレートを使うことが多いですが、まずは手で書きます）

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    /// "=" が見つからない。行番号(1始まり)を持つ
    MissingEquals { line: usize },
    /// キーが空
    EmptyKey { line: usize },
    /// 同じキーが2回出てきた
    DuplicateKey(String),
}

impl fmt::Display for ConfigError {
    /// 表示形式:
    /// - MissingEquals -> "line 3: missing '='"
    /// - EmptyKey      -> "line 3: empty key"
    /// - DuplicateKey  -> "duplicate key: port"
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for ConfigError {}

/// "key = value" 形式の設定ファイルを解析し、(key, value) の Vec を返す。
/// - key と value の前後の空白は取り除く
/// - 空行と '#' で始まる行は無視する
/// - 出現順を保つ
pub fn parse_config(input: &str) -> Result<Vec<(String, String)>, ConfigError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let input = "# comment\nhost = localhost\n\nport=8080\n";
        assert_eq!(
            parse_config(input),
            Ok(vec![
                ("host".to_string(), "localhost".to_string()),
                ("port".to_string(), "8080".to_string()),
            ])
        );
    }

    #[test]
    fn test_errors() {
        assert_eq!(
            parse_config("a=1\nbroken"),
            Err(ConfigError::MissingEquals { line: 2 })
        );
        assert_eq!(parse_config(" = 1"), Err(ConfigError::EmptyKey { line: 1 }));
        assert_eq!(
            parse_config("a=1\na=2"),
            Err(ConfigError::DuplicateKey("a".to_string()))
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(
            ConfigError::MissingEquals { line: 3 }.to_string(),
            "line 3: missing '='"
        );
        assert_eq!(
            ConfigError::EmptyKey { line: 1 }.to_string(),
            "line 1: empty key"
        );
        assert_eq!(
            ConfigError::DuplicateKey("port".into()).to_string(),
            "duplicate key: port"
        );
    }
}
