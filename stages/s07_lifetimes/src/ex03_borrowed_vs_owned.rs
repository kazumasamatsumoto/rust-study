//! ## 課題 03: 借りるか、所有するか / `'static`
//!
//! 同じ「設定」を、借りる版と所有する版の2通りで書いて比べます。
//! どちらが良いかは用途で決まります（README の選球眼の節を参照）。

use std::collections::HashMap;

/// ログレベルの名前を返す。1=error, 2=warn, 3=info, 4=debug, それ以外="unknown"
///
/// `&'static str` は「プログラムが終わるまで有効な文字列」。文字列リテラルは
/// 実行ファイルの中に埋め込まれているので、どこにも消えない。
pub fn level_name(level: u8) -> &'static str {
    todo!()
}

/// 借りる版: "key=value" の行を解析して、入力から借りた (&str, &str) を返す。
/// コピーが起きないので速いが、戻り値は input より長く生きられない。
/// "=" を含まない行と空行は無視する。key と value の前後の空白は取り除く。
pub fn parse_borrowed(input: &str) -> Vec<(&str, &str)> {
    todo!()
}

/// 所有する版: 同じ解析をして、所有した String の HashMap を返す。
/// 同じキーが複数あれば、後のものが勝つ。
/// コピーが起きるが、戻り値は input と無関係にいつまでも使える。
///
/// `parse_borrowed` を再利用すること。
pub fn parse_owned(input: &str) -> HashMap<String, String> {
    todo!()
}

/// 設定ファイルを読み込んで、所有したまま返す関数を想定したもの。
/// 関数の中で作った String から借りた値は返せないので、所有する版が必要になる。
/// （借りる版を返そうとすると E0515 になる。README を参照）
pub fn load_config(contents_from_disk: String) -> HashMap<String, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_name() {
        assert_eq!(level_name(1), "error");
        assert_eq!(level_name(4), "debug");
        assert_eq!(level_name(9), "unknown");
    }

    #[test]
    fn test_parse_borrowed() {
        let input = String::from("host = localhost\n\nbroken\nport=8080");
        let pairs = parse_borrowed(&input);
        assert_eq!(pairs, vec![("host", "localhost"), ("port", "8080")]);
        // 戻り値は input の中を指している（コピーされていない）
        let start = input.as_ptr() as usize;
        let end = start + input.len();
        let p = pairs[0].1.as_ptr() as usize;
        assert!(start <= p && p < end);
    }

    #[test]
    fn test_parse_owned_outlives_input() {
        let map;
        {
            let input = String::from("a=1\nb=2\na=3");
            map = parse_owned(&input);
        } // input はここで消える
        assert_eq!(map.get("a").map(String::as_str), Some("3"));
        assert_eq!(map.get("b").map(String::as_str), Some("2"));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_load_config() {
        let map = load_config("name = rust".to_string());
        assert_eq!(map["name"], "rust");
    }
}
