//! ## 課題 01: Option を使いこなす
//!
//! `match` でも書けますが、`map`, `and_then`, `unwrap_or`, `filter`, `?`
//! などのコンビネータを使うと簡潔に書けます。両方試してみましょう。

/// (名前, 年齢) の一覧から名前で年齢を探す。
pub fn find_age(people: &[(&str, u32)], name: &str) -> Option<u32> {
    todo!()
}

/// 文字列を整数として解析し、2倍にして返す。失敗したら None。
pub fn parse_and_double(s: &str) -> Option<i32> {
    todo!()
}

/// 名前で探した人の年齢が 20 以上なら Some(年齢)、それ以外は None。
/// `find_age` を再利用すること。
pub fn find_adult_age(people: &[(&str, u32)], name: &str) -> Option<u32> {
    todo!()
}

/// "key=value" の value 部分を返す。key が一致しなければ None。
/// Option を返す関数の中では `?` が使えることを試そう。
pub fn get_value<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PEOPLE: &[(&str, u32)] = &[("alice", 30), ("bob", 17), ("carol", 20)];

    #[test]
    fn test_find_age() {
        assert_eq!(find_age(PEOPLE, "bob"), Some(17));
        assert_eq!(find_age(PEOPLE, "dave"), None);
    }

    #[test]
    fn test_parse_and_double() {
        assert_eq!(parse_and_double("21"), Some(42));
        assert_eq!(parse_and_double("-3"), Some(-6));
        assert_eq!(parse_and_double("abc"), None);
    }

    #[test]
    fn test_find_adult_age() {
        assert_eq!(find_adult_age(PEOPLE, "alice"), Some(30));
        assert_eq!(find_adult_age(PEOPLE, "carol"), Some(20));
        assert_eq!(find_adult_age(PEOPLE, "bob"), None);
        assert_eq!(find_adult_age(PEOPLE, "dave"), None);
    }

    #[test]
    fn test_get_value() {
        assert_eq!(get_value("port=8080", "port"), Some("8080"));
        assert_eq!(get_value("port=8080", "host"), None);
        assert_eq!(get_value("port8080", "port"), None);
    }
}
