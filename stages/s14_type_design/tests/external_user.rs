//! 統合テストは、このクレートを「外から使う」別のクレートとしてコンパイルされます。
//! そのため、`#[non_exhaustive]` や sealed trait の効果は、ここで初めて現れます。
//! （同じクレートの中では、`#[non_exhaustive]` は何の制限にもなりません。）

use s14_type_design::ex03_money::{Jpy, Money, ParseMoneyError, Usd};

/// 外のクレートから見た、エラーの扱い方の例。
/// `#[non_exhaustive]` の enum なので、最後の `_` の腕が必須。
fn user_message(e: &ParseMoneyError) -> String {
    match e {
        ParseMoneyError::Empty => "金額を入力してください".to_string(),
        ParseMoneyError::InvalidDigit => "数字で入力してください".to_string(),
        ParseMoneyError::TooManyDecimals { max } => format!("小数点以下は {max} 桁までです"),
        ParseMoneyError::Overflow => "金額が大きすぎます".to_string(),
        _ => "金額の形式が正しくありません".to_string(),
    }
}

#[test]
fn messages_for_users() {
    let e = Money::<Jpy>::parse("1.5").unwrap_err();
    assert_eq!(user_message(&e), "小数点以下は 0 桁までです");
    let e = Money::<Usd>::parse("").unwrap_err();
    assert_eq!(user_message(&e), "金額を入力してください");
}

#[test]
fn generic_code_over_currencies() {
    // 外のクレートでも、Currency を境界にしたジェネリックな関数は書ける（実装はできないが、使うことはできる）
    fn double<C: s14_type_design::ex03_money::Currency>(m: Money<C>) -> Money<C> {
        m + m
    }
    assert_eq!(
        double(Money::<Usd>::from_minor(150)).to_string(),
        "3.00 USD"
    );
}
