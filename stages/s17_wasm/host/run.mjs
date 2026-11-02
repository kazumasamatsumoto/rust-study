// Rust で作った WASM を、wasm-bindgen を使わずに、素の WebAssembly API で呼び出す。
//
//   cargo build -p s17_wasm --target wasm32-unknown-unknown --release
//   node stages/s17_wasm/host/run.mjs
//
// 未実装の関数は、仮の値（0 など）を返します。
// panic したときにどう見えるかは、panic_demo で確かめられます。

import { readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import path from "node:path";

const here = path.dirname(fileURLToPath(import.meta.url));
const wasmPath = path.join(
  here, "..", "..", "..", "target", "wasm32-unknown-unknown", "release", "s17_wasm.wasm",
);

let bytes;
try {
  bytes = await readFile(wasmPath);
} catch {
  console.error(`${wasmPath} がありません。先にビルドしてください:`);
  console.error("  cargo build -p s17_wasm --target wasm32-unknown-unknown --release");
  process.exit(1);
}

// ---- ホストが WASM に渡す関数（ex03 で使う）----------------------------------
// WASM は、ここで渡したもの以外には何もアクセスできない。
let memory; // インスタンスを作った後に設定する

const imports = {
  env: {
    host_log(ptr, len) {
      // ptr と len は、ただの数値。WASM のメモリを覗いて、文字列を取り出す。
      const view = new Uint8Array(memory.buffer, ptr, len);
      console.log(`    [wasm] ${new TextDecoder().decode(view)}`);
    },
    host_random() {
      return Math.random();
    },
  },
};

const { instance } = await WebAssembly.instantiate(bytes, imports);
const wasm = instance.exports;
memory = wasm.memory;

console.log(`読み込んだ: ${path.relative(process.cwd(), wasmPath)} (${bytes.length} バイト)`);
console.log(`export されているもの: ${Object.keys(wasm).join(", ")}`);
console.log(`線形メモリの大きさ: ${memory.buffer.byteLength} バイト (${memory.buffer.byteLength / 65536} ページ)`);

// 1つ呼ぶごとに、失敗しても止まらないようにする
function tryCall(label, f) {
  try {
    console.log(`${label} = ${show(f())}`);
  } catch (e) {
    console.log(`${label} → ${e.constructor.name}: ${e.message}`);
  }
}
function show(v) {
  return typeof v === "bigint" ? `${v}n (BigInt)` : `${JSON.stringify(v)} (${typeof v})`;
}

// ---- panic するとどう見えるか -----------------------------------------------------
console.log("\n== WASM の中で panic すると ==");
tryCall("panic_demo()", () => wasm.panic_demo());

// ---- ex01: 数値だけ ------------------------------------------------------------
console.log("\n== ex01: 数値だけをやり取りする ==");
tryCall("add(2, 3)", () => wasm.add(2, 3));
tryCall("add(2147483647, 1)", () => wasm.add(2147483647, 1));
tryCall("add(2.9, 0)        ", () => wasm.add(2.9, 0)); // i32 に変換される
tryCall("fib(90)", () => wasm.fib(90));
tryCall("is_prime(7919)", () => wasm.is_prime(7919));
tryCall("celsius_to_fahrenheit(36.5)", () => wasm.celsius_to_fahrenheit(36.5));

// ---- ex02: 線形メモリ ----------------------------------------------------------
console.log("\n== ex02: 線形メモリを通して文字列を渡す ==");

// JavaScript の文字列を WASM のメモリに書き込み、(ptr, len) を返す
function putString(s) {
  const encoded = new TextEncoder().encode(s); // UTF-8 のバイト列にする
  const ptr = wasm.alloc(encoded.length);
  // null（0）が返ってきたら、確保に失敗している。0 番地に書き込むと、WASM のメモリを壊してしまう。
  if (ptr === 0) throw new Error("alloc が 0 (null) を返しました。alloc は実装しましたか？");
  // 注意: alloc の中でメモリが拡張されると、古い memory.buffer は使えなくなる。
  // だから、Uint8Array は alloc を呼んだ「後」に作り直す。
  new Uint8Array(memory.buffer).set(encoded, ptr);
  return [ptr, encoded.length];
}
function readString(ptr, len) {
  return new TextDecoder().decode(new Uint8Array(memory.buffer, ptr, len));
}

tryCall("count_chars(\"Rustは楽しい🦀\")", () => {
  const [ptr, len] = putString("Rustは楽しい🦀");
  const n = wasm.count_chars(ptr, len);
  wasm.dealloc(ptr, len);
  return `${n} 文字 (UTF-8 では ${len} バイト, ptr = ${ptr})`;
});

tryCall("to_upper_in_place(\"hello, wasm!\")", () => {
  const [ptr, len] = putString("hello, wasm!");
  wasm.to_upper_in_place(ptr, len);
  const s = readString(ptr, len);
  wasm.dealloc(ptr, len);
  return s;
});

tryCall("sum_i32([1, 2, 3, 2147483647])", () => {
  const values = [1, 2, 3, 2147483647];
  const len = values.length;
  const ptr = wasm.alloc(len * 4); // i32 は 4 バイト
  if (ptr === 0) throw new Error("alloc が 0 (null) を返しました。alloc は実装しましたか？");
  new Int32Array(memory.buffer, ptr, len).set(values);
  const total = wasm.sum_i32(ptr, len);
  wasm.dealloc(ptr, len * 4);
  return total;
});

tryCall("greet(\"Ferris\")", () => {
  const [namePtr, nameLen] = putString("Ferris");
  const outLenPtr = wasm.alloc(4); // usize は wasm32 では 4 バイト
  const ptr = wasm.greet(namePtr, nameLen, outLenPtr);
  if (ptr === 0) throw new Error("greet が 0 (null) を返しました。greet は実装しましたか？");
  const outLen = new DataView(memory.buffer).getUint32(outLenPtr, true); // リトルエンディアン
  const s = readString(ptr, outLen);
  wasm.dealloc(ptr, outLen);
  wasm.dealloc(outLenPtr, 4);
  wasm.dealloc(namePtr, nameLen);
  return s;
});

// ---- ex03: ホストの関数を呼んでもらう -------------------------------------------
console.log("\n== ex03: ホストの関数を import して呼ぶ ==");
tryCall("fizzbuzz_log(5)", () => wasm.fizzbuzz_log(5));
tryCall("roll_dice(3)", () => wasm.roll_dice(3));
