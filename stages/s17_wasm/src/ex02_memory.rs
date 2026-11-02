//! ## 課題 02: 線形メモリを通して、文字列や配列をやり取りする
//!
//! WASM の関数は数値しか受け取れません。では文字列はどう渡すのか。答えは
//! **「メモリに書いておいて、その場所（ポインタ）と長さを数値として渡す」**です。
//!
//! WASM のメモリ（線形メモリ）は、ただの大きなバイト配列です。JavaScript からも
//! `new Uint8Array(memory.buffer)` として読み書きできます。ポインタとは、この配列の
//! 何バイト目かを表す数値（オフセット）にすぎません。
//!
//! ```text
//! JavaScript                          WASM の線形メモリ
//!   1. ptr = alloc(5)        ──→  [ ........ | ? ? ? ? ? | ........ ]
//!                                             ↑ptr
//!   2. memory[ptr..] に書く  ──→  [ ........ | h e l l o | ........ ]
//!   3. count_chars(ptr, 5)   ──→  Rust が ptr から 5 バイト読む → 5 を返す
//!   4. dealloc(ptr, 5)       ──→  領域を返す
//! ```
//!
//! ここで問題になるのが「**このメモリを解放する責任は誰にあるのか**」です。
//! JavaScript には Rust の所有権がありません。境界を越えた瞬間に、コンパイラの保証は消えます。
//! だから、ポインタを受け取る関数は全て `unsafe fn` になり、「呼ぶ側が守るべき約束」を
//! `# Safety` に書きます（Stage 13 と同じ考え方です）。
//!
//! ヒント:
//! - 確保: `vec![0u8; len].into_boxed_slice()` と `Box::into_raw`
//! - 解放: `std::ptr::slice_from_raw_parts_mut` と `Box::from_raw`
//! - ポインタと長さからスライスを作る: `std::slice::from_raw_parts`（`_mut`）
//! - Rust 2024 では、`unsafe fn` の中でも unsafe な操作には `unsafe { }` ブロックが必要です

/// len バイトの領域を確保し、その先頭のポインタを返す（中身は 0 で埋める）。
/// 確保した領域は、呼び出し側が `dealloc(ptr, len)` で返す責任を持つ。
///
/// 考えてみよう: 関数を抜けたあとも、確保した領域が解放されないようにするにはどうするか？
#[unsafe(no_mangle)]
pub extern "C" fn alloc(len: usize) -> *mut u8 {
    // TODO: 仮の値です。自分の実装に置き換えてください（todo!() を使わない理由は lib.rs を参照）
    std::ptr::null_mut()
}

/// `alloc` で確保した領域を返す。
///
/// # Safety
///
/// - `ptr` と `len` は、`alloc(len)` が返したポインタと、そのときに渡した長さであること
///   （`greet` が返したポインタと長さでもよい）
/// - 同じ領域を2回 `dealloc` しないこと。また、`dealloc` した後に使わないこと
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dealloc(ptr: *mut u8, len: usize) {
    // TODO: 仮の値です。自分の実装に置き換えてください（todo!() を使わない理由は lib.rs を参照）
}

/// ptr から len バイトを UTF-8 として解釈し、文字（`char`）の数を返す。
/// UTF-8 として不正なら -1 を返す。
///
/// # Safety
///
/// `ptr` から `len` バイトが、読み取り可能な有効なメモリであること。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn count_chars(ptr: *const u8, len: usize) -> i32 {
    // TODO: 仮の値です。自分の実装に置き換えてください（todo!() を使わない理由は lib.rs を参照）
    0
}

/// ptr から len バイトのうち、ASCII の小文字を大文字に、その場で書き換える。
/// ASCII 以外のバイトはそのままにする（だから、UTF-8 の文字が壊れることはない。なぜか？）。
///
/// # Safety
///
/// `ptr` から `len` バイトが、書き込み可能な有効なメモリであり、
/// この関数の実行中に他から読み書きされないこと。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn to_upper_in_place(ptr: *mut u8, len: usize) {
    // TODO: 仮の値です。自分の実装に置き換えてください（todo!() を使わない理由は lib.rs を参照）
}

/// ptr から始まる len 個の i32 の合計を返す。合計は i64 で計算する。
///
/// # Safety
///
/// `ptr` から `len` 個の `i32` が、読み取り可能な有効なメモリであり、
/// `ptr` が 4 の倍数のアドレス（`i32` のアラインメント）であること。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sum_i32(ptr: *const i32, len: usize) -> i64 {
    // TODO: 仮の値です。自分の実装に置き換えてください（todo!() を使わない理由は lib.rs を参照）
    0
}

/// 名前を受け取り、`"Hello, {name}!"` という新しい文字列をメモリ上に作って、そのポインタを返す。
/// 作った文字列のバイト数は `out_len` が指す場所に書き込む。
/// 名前が UTF-8 として不正なら、`"Hello, ?!"` を返す。
///
/// 返した領域は、呼び出し側が `dealloc(ptr, *out_len)` で返す責任を持つ。
///
/// 考えてみよう: WASM の関数は戻り値を1つしか返せない（`extern "C"` の場合）。
/// ポインタと長さの2つを返すために、この関数はどんな工夫をしているか？
///
/// # Safety
///
/// - `name_ptr` から `name_len` バイトが、読み取り可能な有効なメモリであること
/// - `out_len` が、書き込み可能な有効な `usize` を指していること
#[unsafe(no_mangle)]
pub unsafe extern "C" fn greet(
    name_ptr: *const u8,
    name_len: usize,
    out_len: *mut usize,
) -> *mut u8 {
    // TODO: 仮の値です。自分の実装に置き換えてください（todo!() を使わない理由は lib.rs を参照）
    std::ptr::null_mut()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// テスト用: バイト列を alloc した領域にコピーして、(ptr, len) を返す。
    /// JavaScript 側がやることを、Rust で真似している。
    fn put(bytes: &[u8]) -> (*mut u8, usize) {
        let ptr = alloc(bytes.len());
        assert!(
            !ptr.is_null(),
            "alloc が null を返しました。先に alloc を実装してください"
        );
        // SAFETY: alloc(bytes.len()) で確保した直後の、bytes.len() バイトの領域に書き込んでいる。
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len()) };
        (ptr, bytes.len())
    }

    #[test]
    fn test_alloc_is_zeroed_and_dealloc() {
        let ptr = alloc(16);
        assert!(
            !ptr.is_null(),
            "alloc が null を返しました。先に alloc を実装してください"
        );
        // SAFETY: alloc(16) で確保した 16 バイトを読んでいる。
        let bytes = unsafe { std::slice::from_raw_parts(ptr, 16) };
        assert_eq!(bytes, &[0u8; 16]);
        // SAFETY: alloc(16) が返したポインタと長さを、1回だけ返している。
        unsafe { dealloc(ptr, 16) };
    }

    #[test]
    fn test_alloc_zero_length() {
        let ptr = alloc(0);
        // SAFETY: alloc(0) が返したポインタと長さを、1回だけ返している。
        unsafe { dealloc(ptr, 0) };
    }

    #[test]
    fn test_count_chars() {
        for (s, expected) in [("hello", 5), ("あいう", 3), ("🦀 Rust", 6), ("", 0)] {
            let (ptr, len) = put(s.as_bytes());
            // SAFETY: put で確保して書き込んだ len バイトを読ませている。
            assert_eq!(unsafe { count_chars(ptr, len) }, expected, "{s:?}");
            // SAFETY: put（中身は alloc）で確保した領域を、1回だけ返している。
            unsafe { dealloc(ptr, len) };
        }
    }

    #[test]
    fn test_count_chars_invalid_utf8() {
        // 「あ」(E3 81 82) の途中で切れている
        let (ptr, len) = put(&[0x61, 0xE3, 0x81]);
        // SAFETY: put で確保して書き込んだ len バイトを読ませている。
        assert_eq!(unsafe { count_chars(ptr, len) }, -1);
        // SAFETY: put で確保した領域を、1回だけ返している。
        unsafe { dealloc(ptr, len) };
    }

    #[test]
    fn test_to_upper_in_place() {
        let (ptr, len) = put("hello, rust! あ".as_bytes());
        // SAFETY: put で確保した len バイトを書き換えさせている。他からは触っていない。
        unsafe { to_upper_in_place(ptr, len) };
        // SAFETY: 同じ領域を読んでいる。
        let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
        assert_eq!(std::str::from_utf8(bytes), Ok("HELLO, RUST! あ"));
        // SAFETY: put で確保した領域を、1回だけ返している。
        unsafe { dealloc(ptr, len) };
    }

    #[test]
    fn test_sum_i32() {
        let v: Vec<i32> = vec![1, 2, 3, i32::MAX, i32::MAX];
        // SAFETY: v は v.len() 個の i32 を持ち、アラインメントも正しい。
        let total = unsafe { sum_i32(v.as_ptr(), v.len()) };
        assert_eq!(total, 6 + 2 * i32::MAX as i64);

        let empty: Vec<i32> = vec![];
        // SAFETY: 長さ 0 なので、何も読まれない。Vec のポインタは長さ 0 でもアラインメントが正しい。
        assert_eq!(unsafe { sum_i32(empty.as_ptr(), 0) }, 0);
    }

    #[test]
    fn test_greet() {
        let (name_ptr, name_len) = put("Ferris".as_bytes());
        let mut out_len = 0usize;
        // SAFETY: name は put で用意した有効な領域。out_len は有効な usize を指している。
        let ptr = unsafe { greet(name_ptr, name_len, &mut out_len) };
        assert!(!ptr.is_null(), "greet が null を返しました");
        // SAFETY: greet が返した ptr から out_len バイトを読んでいる。
        let bytes = unsafe { std::slice::from_raw_parts(ptr, out_len) };
        assert_eq!(std::str::from_utf8(bytes), Ok("Hello, Ferris!"));
        // SAFETY: greet が返した領域と、put で確保した領域を、それぞれ1回だけ返している。
        unsafe {
            dealloc(ptr, out_len);
            dealloc(name_ptr, name_len);
        }
    }

    #[test]
    fn test_greet_invalid_utf8() {
        let (name_ptr, name_len) = put(&[0xFF]);
        let mut out_len = 0usize;
        // SAFETY: test_greet と同じ。
        let ptr = unsafe { greet(name_ptr, name_len, &mut out_len) };
        assert!(!ptr.is_null(), "greet が null を返しました");
        // SAFETY: test_greet と同じ。
        let bytes = unsafe { std::slice::from_raw_parts(ptr, out_len) };
        assert_eq!(bytes, b"Hello, ?!");
        // SAFETY: test_greet と同じ。
        unsafe {
            dealloc(ptr, out_len);
            dealloc(name_ptr, name_len);
        }
    }
}
