// 実験: JavaScript と WASM の「境界を越えるコスト」と「中身の計算の速さ」を比べる。
// ex01 と ex02 を解いた後に実行してください。
//
//   cargo build -p s17_wasm --target wasm32-unknown-unknown --release
//   node stages/s17_wasm/host/bench.mjs
//
// 時間は PC や実行ごとに変わります。見るべきなのは「どちらが何倍か」という傾向です。

import { readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import path from "node:path";

const here = path.dirname(fileURLToPath(import.meta.url));
const wasmPath = path.join(
  here, "..", "..", "..", "target", "wasm32-unknown-unknown", "release", "s17_wasm.wasm",
);
const bytes = await readFile(wasmPath);
const { instance } = await WebAssembly.instantiate(bytes, {
  env: { host_log() {}, host_random: Math.random },
});
const wasm = instance.exports;

function time(label, f) {
  f(); // ウォームアップ（JIT コンパイルを済ませておく）
  const start = performance.now();
  const result = f();
  const ms = performance.now() - start;
  console.log(`${label.padEnd(34)} ${ms.toFixed(1).padStart(8)} ms  (結果: ${result})`);
  return ms;
}

// ---- 1. 中身がほぼ空の関数を、たくさん呼ぶ ----------------------------------------
console.log("== 1. add を 1000 万回呼ぶ（計算はほぼゼロ。境界を越えるコストが目立つ）==");
const jsAdd = (a, b) => (a + b) | 0;
time("JavaScript の add", () => { let s = 0; for (let i = 0; i < 1e7; i++) s = jsAdd(s, i); return s; });
time("WASM の add (毎回 境界を越える)", () => { let s = 0; for (let i = 0; i < 1e7; i++) s = wasm.add(s, i); return s; });

// ---- 2. 中身の計算が重い関数 -------------------------------------------------------
console.log("\n== 2. is_prime を 1〜300 万で呼ぶ（1回あたりの計算が多い）==");
function jsIsPrime(n) {
  if (n < 2) return false;
  for (let i = 2; i * i <= n; i++) if (n % i === 0) return false;
  return true;
}
time("JavaScript の is_prime", () => { let c = 0; for (let n = 1; n <= 3e6; n++) if (jsIsPrime(n)) c++; return c; });
time("WASM の is_prime", () => { let c = 0; for (let n = 1; n <= 3e6; n++) if (wasm.is_prime(n)) c++; return c; });

// ---- 3. 大きなデータを渡す ---------------------------------------------------------
console.log("\n== 3. 約 10MB の文字列の文字数を数える（データのコピーが必要）==");
const text = "Rustは楽しい🦀 ".repeat(500_000);
time("JavaScript だけで数える", () => { let c = 0; for (const _ of text) c++; return c; });
let copyMs = 0;
time("WASM で数える (UTF-8 変換+コピー込み)", () => {
  const t0 = performance.now();
  const encoded = new TextEncoder().encode(text);
  const ptr = wasm.alloc(encoded.length);
  if (ptr === 0) throw new Error("alloc が 0 (null) を返しました。先に ex02 を解いてください");
  new Uint8Array(wasm.memory.buffer).set(encoded, ptr);
  copyMs = performance.now() - t0;
  const n = wasm.count_chars(ptr, encoded.length);
  wasm.dealloc(ptr, encoded.length);
  return n;
});
console.log(`  └ そのうち UTF-8 への変換とコピー: ${copyMs.toFixed(1)} ms`);
