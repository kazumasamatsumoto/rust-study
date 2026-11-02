//! ## 課題 03: 移動平均と、ハッシュ関数を自分で書く
//!
//! (1) 同じ計算を何度も繰り返していないかを疑う（計算の使い回し）
//! (2) HashMap の速さは、ハッシュ関数で大きく変わる。標準のハッシュ関数が
//!     なぜ「わざと遅め」なのかを、自分で速いハッシュ関数を書いて体感する

use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

// ---------------------------------------------------------------------------
// (1) 移動平均
// ---------------------------------------------------------------------------

/// 遅い実装: 窓をずらすたびに、窓の中を全部足し直している。O(n × w)。
pub fn moving_average_slow(v: &[f64], window: usize) -> Vec<f64> {
    if window == 0 || window > v.len() {
        return vec![];
    }
    v.windows(window)
        .map(|w| w.iter().sum::<f64>() / window as f64)
        .collect()
}

/// 長さ window の窓の平均を、窓を 1 つずつずらしながら求める。
/// window が 0 か、v より長ければ空の Vec。
///
/// O(n) で書くこと。ヒント: 窓を 1 つずらすと、出ていく要素が 1 つ、入ってくる要素が 1 つ。
///
/// 注意: 足し算の順番が変わるので、moving_average_slow と**ビット単位では**一致しない
/// ことがある（CS基礎 02）。テストでは、差が十分小さいことを確かめている。
pub fn moving_average(v: &[f64], window: usize) -> Vec<f64> {
    todo!()
}

// ---------------------------------------------------------------------------
// (2) FNV-1a ハッシュ
// ---------------------------------------------------------------------------

/// FNV-1a ハッシュ関数（64 ビット版）。
///
/// 仕組み: 状態を `FNV_OFFSET` で始め、入力のバイトごとに次の 2 つを繰り返す。
/// 最後の状態がハッシュ値になる。
///
/// 1. 状態 ^= バイト
/// 2. 状態 = 状態 × `FNV_PRIME`（溢れた分は捨てる＝`wrapping_mul`）
///
/// とても単純で速いが、**攻撃に弱い**（README の HashDoS を参照）。
pub struct Fnv1a {
    state: u64,
}

pub const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
pub const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

impl Default for Fnv1a {
    fn default() -> Self {
        todo!()
    }
}

impl Hasher for Fnv1a {
    fn write(&mut self, bytes: &[u8]) {
        todo!()
    }

    fn finish(&self) -> u64 {
        todo!()
    }
}

/// HashMap に渡すための「Fnv1a を作る方法」。
/// `BuildHasherDefault<H>` は「`H::default()` で作る」という意味の、標準ライブラリの型。
pub type FnvBuildHasher = BuildHasherDefault<Fnv1a>;

/// FNV-1a を使う HashMap。
pub type FnvHashMap<K, V> = HashMap<K, V, FnvBuildHasher>;

/// bytes の FNV-1a ハッシュ値を返す（Hasher を 1 回だけ使う）。
pub fn fnv1a(bytes: &[u8]) -> u64 {
    todo!()
}

/// 各値の出現回数を数える。FnvHashMap を使うこと。
pub fn count_with_fnv(v: &[u64]) -> FnvHashMap<u64, usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moving_average() {
        let v: Vec<f64> = (1..=10).map(|x| x as f64).collect();
        for w in [0, 1, 2, 3, 10, 11] {
            let fast = moving_average(&v, w);
            let slow = moving_average_slow(&v, w);
            assert_eq!(fast.len(), slow.len(), "window={w}");
            for (a, b) in fast.iter().zip(&slow) {
                assert!((a - b).abs() < 1e-9, "window={w}: {a} vs {b}");
            }
        }
        assert_eq!(
            moving_average(&[1.0, 2.0, 3.0, 4.0], 2),
            vec![1.5, 2.5, 3.5]
        );
    }

    #[test]
    fn test_moving_average_large() {
        // O(n × w) だと、この大きさでは遅すぎる
        let v: Vec<f64> = (0..200_000).map(|x| (x % 100) as f64).collect();
        let r = moving_average(&v, 50_000);
        assert_eq!(r.len(), 150_001);
        assert!((r[0] - 49.5).abs() < 1e-6);
    }

    #[test]
    fn test_fnv_known_values() {
        // FNV-1a 64 の、公開されているテスト値
        assert_eq!(fnv1a(b""), 0xcbf2_9ce4_8422_2325);
        assert_eq!(fnv1a(b"a"), 0xaf63_dc4c_8601_ec8c);
        assert_eq!(fnv1a(b"foobar"), 0x8594_4171_f739_67e8);
    }

    #[test]
    fn test_fnv_write_in_pieces() {
        // 分けて write しても、まとめて write しても同じ結果になる
        let mut h = Fnv1a::default();
        h.write(b"foo");
        h.write(b"bar");
        assert_eq!(h.finish(), fnv1a(b"foobar"));
    }

    #[test]
    fn test_count_with_fnv() {
        let m = count_with_fnv(&[3, 1, 3, 3, 2, 1]);
        assert_eq!(m.len(), 3);
        assert_eq!(m[&3], 3);
        assert_eq!(m[&1], 2);
        assert_eq!(m[&2], 1);
    }
}
