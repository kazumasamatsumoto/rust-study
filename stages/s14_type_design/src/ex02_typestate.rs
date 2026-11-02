//! ## 課題 02: typestate パターン — 状態を型パラメータで表す
//!
//! Stage 03 の自販機は、状態を enum で表し、`match` で遷移を書きました。
//! その方式では「Idle のときに PushButton を押す」のような不正な操作は、
//! **実行時に**「何もしない」として処理するしかありませんでした。
//!
//! typestate パターンでは、状態ごとに**別の型**を用意します。
//! 「閉まっているドアにだけ `lock` メソッドがある」のようにすれば、
//! 不正な操作は**コンパイル時に**エラーになります。
//!
//! ```compile_fail
//! use s14_type_design::ex02_typestate::Door;
//! let door = Door::new(1234);   // Door<Closed>
//! let door = door.lock();       // Door<Locked>
//! let door = door.open();       // 鍵がかかったドアは開けられない → コンパイルエラー
//! ```
//!
//! ```compile_fail
//! use s14_type_design::ex02_typestate::Door;
//! let door = Door::new(1234).open(); // Door<Open>
//! let door = door.lock();            // 開いたドアに鍵はかけられない → コンパイルエラー
//! ```
//!
//! ```compile_fail
//! use s14_type_design::ex02_typestate::RequestBuilder;
//! // URL を設定せずに build はできない → コンパイルエラー
//! let req = RequestBuilder::new().method("POST").build();
//! ```
//!
//! ```compile_fail
//! use s14_type_design::ex02_typestate::Door;
//! let closed = Door::new(1234);
//! let open = closed.open();
//! // open() は self を受け取る（ムーブする）ので、古い状態のドアはもう使えない
//! let again = closed.open();
//! ```

use std::marker::PhantomData;

// ---------------------------------------------------------------------------
// (1) ドア
// ---------------------------------------------------------------------------

// 状態を表す型。中身は空（サイズ 0 バイト）で、型として区別するためだけに存在する。
#[derive(Debug)]
pub struct Open;
#[derive(Debug)]
pub struct Closed;
#[derive(Debug)]
pub struct Locked;

/// 状態 `S` のドア。
///
/// `S` はフィールドとしては使っていないので、そのままではコンパイラに
/// 「使っていない型パラメータがある」と怒られる。
/// `PhantomData<S>` は「S を持っているふりをする、サイズ 0 のフィールド」。
#[derive(Debug)]
pub struct Door<S> {
    code: u32,
    /// これまでの操作の記録（テストで確認するため）
    history: Vec<&'static str>,
    _state: PhantomData<S>,
}

impl Door<Closed> {
    /// 暗証番号 code を持つ、閉まったドアを作る。history は空。
    pub fn new(code: u32) -> Door<Closed> {
        todo!()
    }

    /// 開ける。history に "open" を追加する。
    pub fn open(self) -> Door<Open> {
        todo!()
    }

    /// 鍵をかける。history に "lock" を追加する。
    /// 鍵をかけるときは暗証番号を確かめない（ここでは「設定済みの番号でかける」という仕様にする）。
    pub fn lock(self) -> Door<Locked> {
        todo!()
    }
}

impl Door<Open> {
    /// 閉める。history に "close" を追加する。
    pub fn close(self) -> Door<Closed> {
        todo!()
    }
}

impl Door<Locked> {
    /// 暗証番号が合っていれば、history に "unlock" を追加して Ok(閉まったドア)。
    /// 間違っていれば、history に "unlock failed" を追加して Err(鍵のかかったドアのまま)。
    ///
    /// 失敗しても「ドアそのもの」を返すのがポイント。
    /// Err にドアを入れないと、失敗した瞬間にドアが消えてしまう（所有権がムーブ済みのため）。
    pub fn unlock(self, code: u32) -> Result<Door<Closed>, Door<Locked>> {
        todo!()
    }
}

/// どの状態でも使えるメソッド
impl<S> Door<S> {
    pub fn history(&self) -> &[&'static str] {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// (2) ビルダー: 必須の項目を型で強制する
// ---------------------------------------------------------------------------

/// URL がまだ設定されていない状態
#[derive(Debug)]
pub struct NoUrl;
/// URL が設定された状態
#[derive(Debug)]
pub struct HasUrl(String);

#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub timeout_secs: u64,
}

/// HTTP リクエストのビルダー。
///
/// 型パラメータ `U` が「URL が設定済みかどうか」を表す。
/// `build()` は `RequestBuilder<HasUrl>` にしか定義しないので、
/// URL を忘れるとコンパイルエラーになる。
///
/// デフォルト値: method = "GET", headers = 空, timeout_secs = 30
#[derive(Debug)]
pub struct RequestBuilder<U> {
    url: U,
    method: String,
    headers: Vec<(String, String)>,
    timeout_secs: u64,
}

impl RequestBuilder<NoUrl> {
    pub fn new() -> Self {
        todo!()
    }
}

impl Default for RequestBuilder<NoUrl> {
    fn default() -> Self {
        Self::new()
    }
}

/// URL の有無に関係なく使えるメソッド。
/// どれも self を受け取って、同じ状態のビルダーを返す（メソッドチェーンにするため）。
impl<U> RequestBuilder<U> {
    /// URL を設定する。どの状態からでも呼べて、HasUrl 状態のビルダーになる。
    /// 2 回呼んだら、後から設定した方が使われる。
    pub fn url(self, url: &str) -> RequestBuilder<HasUrl> {
        todo!()
    }

    pub fn method(self, method: &str) -> Self {
        todo!()
    }

    /// ヘッダを追加する（追加した順に保存する）。
    pub fn header(self, name: &str, value: &str) -> Self {
        todo!()
    }

    pub fn timeout_secs(self, secs: u64) -> Self {
        todo!()
    }
}

impl RequestBuilder<HasUrl> {
    pub fn build(self) -> Request {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn test_door_happy_path() {
        let door = Door::new(1234).open().close().lock();
        let door = door.unlock(1234).expect("正しい番号なら開くはず");
        let door = door.open();
        assert_eq!(door.history(), ["open", "close", "lock", "unlock", "open"]);
    }

    #[test]
    fn test_door_wrong_code() {
        let door = Door::new(1234).lock();
        let door = match door.unlock(1111) {
            Ok(_) => panic!("間違った番号で開いてはいけない"),
            Err(still_locked) => still_locked,
        };
        let door = door.unlock(1234).unwrap();
        assert_eq!(door.history(), ["lock", "unlock failed", "unlock"]);
    }

    #[test]
    fn test_phantom_is_zero_sized() {
        // 状態の型は実行時には何のコストもない
        assert_eq!(size_of::<Door<Open>>(), size_of::<Door<Locked>>());
        assert_eq!(size_of::<PhantomData<Locked>>(), 0);
    }

    #[test]
    fn test_builder_defaults() {
        let req = RequestBuilder::new().url("https://example.com").build();
        assert_eq!(
            req,
            Request {
                url: "https://example.com".into(),
                method: "GET".into(),
                headers: vec![],
                timeout_secs: 30,
            }
        );
    }

    #[test]
    fn test_builder_any_order() {
        // URL は最初でも最後でも設定できる
        let req = RequestBuilder::new()
            .method("POST")
            .header("Content-Type", "application/json")
            .url("https://a.example")
            .header("X-Trace", "1")
            .timeout_secs(5)
            .url("https://b.example")
            .build();
        assert_eq!(req.url, "https://b.example");
        assert_eq!(req.method, "POST");
        assert_eq!(
            req.headers,
            vec![
                ("Content-Type".to_string(), "application/json".to_string()),
                ("X-Trace".to_string(), "1".to_string()),
            ]
        );
        assert_eq!(req.timeout_secs, 5);
    }
}
