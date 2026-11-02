// WASM モジュールを、コンパイラを使わずにバイト列から手で組み立てて動かす。
//
//   node stages/s17_wasm/host/handmade.mjs
//
// 作るのは、次の WAT（WASM のテキスト形式）と同じモジュール:
//
//   (module
//     (func (export "add") (param i32 i32) (result i32)
//       local.get 0
//       local.get 1
//       i32.add))

const bytes = new Uint8Array([
  // ヘッダ: マジックナンバー "\0asm" とバージョン 1
  0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00,

  // セクション 1 (type): 関数の型の一覧
  0x01, 0x07, //   セクション番号 1, 中身は 7 バイト
  0x01, //         型は 1 つ
  0x60, //         関数型
  0x02, 0x7f, 0x7f, // 引数 2 つ: i32, i32
  0x01, 0x7f, //   戻り値 1 つ: i32

  // セクション 3 (function): 各関数がどの型か
  0x03, 0x02, //   セクション番号 3, 中身は 2 バイト
  0x01, //         関数は 1 つ
  0x00, //         0 番の関数は、0 番の型

  // セクション 7 (export): 外に見せるもの
  0x07, 0x07, //   セクション番号 7, 中身は 7 バイト
  0x01, //         export は 1 つ
  0x03, 0x61, 0x64, 0x64, // 名前: 長さ 3, "add"
  0x00, 0x00, //   種類: 関数, 0 番

  // セクション 10 (code): 関数の本体
  0x0a, 0x09, //   セクション番号 10, 中身は 9 バイト
  0x01, //         本体は 1 つ
  0x07, //         この本体は 7 バイト
  0x00, //         ローカル変数の宣言: なし
  0x20, 0x00, //   local.get 0   ← 引数 0 をスタックに積む
  0x20, 0x01, //   local.get 1   ← 引数 1 をスタックに積む
  0x6a, //         i32.add       ← 2 つ取り出して足し、結果を積む
  0x0b, //         end
]);

console.log(`モジュールの大きさ: ${bytes.length} バイト`);
console.log(`WebAssembly.validate: ${WebAssembly.validate(bytes)}`);

const { instance } = await WebAssembly.instantiate(bytes);
console.log(`add(2, 3) = ${instance.exports.add(2, 3)}`);
console.log(`add(2147483647, 1) = ${instance.exports.add(2147483647, 1)}`);

// 1 バイトだけ書き換えて、i32.add (0x6a) を i32.mul (0x6c) にしてみる
const patched = bytes.slice();
patched[patched.indexOf(0x6a)] = 0x6c;
const { instance: mul } = await WebAssembly.instantiate(patched);
console.log(`0x6a → 0x6c に書き換えると: add(6, 7) = ${mul.exports.add(6, 7)}  ← 掛け算になった`);
