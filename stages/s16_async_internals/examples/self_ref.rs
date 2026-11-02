//! 実験: 自己参照する値をムーブすると何が起きるか
//! cargo run -p s16_async_internals --example self_ref
//!
//! このコードは unsafe を使っていません。ポインタの「アドレス」を表示するだけで、
//! そのポインタが指す先を読むことはしていないからです（読むと未定義動作になりえます）。

/// 自分のフィールドを指すポインタを持つ構造体
struct SelfRef {
    data: [u8; 4],
    /// data を指すつもりのポインタ
    ptr_to_data: *const u8,
}

impl SelfRef {
    fn new() -> Self {
        SelfRef {
            data: *b"rust",
            ptr_to_data: std::ptr::null(),
        }
    }

    /// 今の場所で、自分の data を指すようにする
    fn init(&mut self) {
        self.ptr_to_data = self.data.as_ptr();
    }

    fn report(&self, label: &str) {
        let real = self.data.as_ptr();
        let ok = if real == self.ptr_to_data {
            "一致"
        } else {
            "ずれている!"
        };
        println!("{label}");
        println!("  data の本当のアドレス     : {real:p}");
        println!(
            "  ptr_to_data が指すアドレス: {:p}  → {ok}",
            self.ptr_to_data
        );
    }
}

fn main() {
    let mut a = SelfRef::new();
    a.init();
    a.report("① 作った直後 (スタック上)");

    // ムーブ: 中身のバイト列が別の場所にコピーされる。ポインタの値はそのまま。
    let moved = Box::new(a);
    moved.report("② Box に入れた (ヒープに移動した)");

    // &mut さえあれば、mem::swap で中身を入れ替える（= ムーブする）ことができる
    let mut x = SelfRef::new();
    x.init();
    let mut y = SelfRef::new();
    y.init();
    std::mem::swap(&mut x, &mut y);
    x.report("③ &mut を2つ使って mem::swap した後の x");
}
