//! 統合テスト: クレートの「外」から使う。
//!
//! `tests/` ディレクトリのファイルは、それぞれ**別のクレート**としてコンパイルされます。
//! そのため、`pub` なものしか使えません。利用者から見た API が使いやすいかを確かめる場所です。

use s09_modules_testing::Celsius;
use s09_modules_testing::ex01_visibility::stats;

#[test]
fn reexport_is_usable() {
    // 再エクスポートのおかげで、深いパスを書かなくてよい
    let t = Celsius::new(25.0).expect("25℃は有効な温度");
    assert_eq!(t.value(), 25.0);
}

#[test]
fn stats_from_outside() {
    let temps: Vec<Celsius> = [18.0, 22.0, 26.0]
        .into_iter()
        .filter_map(Celsius::new)
        .collect();
    assert_eq!(stats::mean(&temps).map(|c| c.value()), Some(22.0));
    assert_eq!(stats::range(&temps), Some(8.0));

    // 次の行のコメントを外すとコンパイルエラーになる。なぜか説明できるようにしよう。
    // let _ = stats::helpers::values(&temps);
    // let _ = Celsius(-500.0);
}
