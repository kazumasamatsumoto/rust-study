//! ## 課題 02: イテレータアダプタ
//!
//! この課題は、**`for` ループを使わずに**イテレータのメソッドだけで書いてみてください。
//! （書けたら、for ループ版と比べて読みやすさを考えてみましょう）
//!
//! 使えそうなメソッド: `filter`, `map`, `filter_map`, `sum`, `zip`, `scan`,
//! `chunks`, `lines`, `split_once`, `collect`

/// 奇数だけを取り出して、それぞれを2乗した合計を返す。
/// 例: [1, 2, 3, 4, 5] -> 1 + 9 + 25 = 35
///
/// 注意: 負の奇数も奇数です。`-3 % 2` の値に気をつけて（CS基礎 01 を参照）。
pub fn sum_of_squares_of_odds(v: &[i32]) -> i32 {
    todo!()
}

/// それぞれの位置までの最大値を並べる。
/// 例: [1, 3, 2, 5, 4] -> [1, 3, 3, 5, 5]
pub fn running_max(v: &[i32]) -> Vec<i32> {
    todo!()
}

/// 2つのベクトルの内積（対応する要素の積の合計）。長さが違えば None。
/// 例: [1, 2, 3] と [4, 5, 6] -> 1*4 + 2*5 + 3*6 = 32
pub fn dot(a: &[i32], b: &[i32]) -> Option<i32> {
    todo!()
}

/// size 個ずつに区切って、それぞれの平均を返す。最後の区切りは size 個未満でもよい。
/// size は 1 以上と仮定してよい。
/// 例: [1, 2, 3, 4, 5], size=2 -> [1.5, 3.5, 5.0]
pub fn chunk_averages(v: &[f64], size: usize) -> Vec<f64> {
    todo!()
}

/// "名前:点数" の行を解析する。形式が正しくない行は**読み飛ばす**。
/// 名前と点数の前後の空白は取り除く。名前が空の行も読み飛ばす。
///
/// 例: "alice:80\nbad line\nbob:x\ncarol: 95" -> [("alice", 80), ("carol", 95)]
pub fn parse_scores(input: &str) -> Vec<(String, u32)> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_squares_of_odds() {
        assert_eq!(sum_of_squares_of_odds(&[1, 2, 3, 4, 5]), 35);
        assert_eq!(sum_of_squares_of_odds(&[-3, -2]), 9);
        assert_eq!(sum_of_squares_of_odds(&[]), 0);
    }

    #[test]
    fn test_running_max() {
        assert_eq!(running_max(&[1, 3, 2, 5, 4]), vec![1, 3, 3, 5, 5]);
        assert_eq!(running_max(&[-5, -7]), vec![-5, -5]);
        assert_eq!(running_max(&[]), Vec::<i32>::new());
    }

    #[test]
    fn test_dot() {
        assert_eq!(dot(&[1, 2, 3], &[4, 5, 6]), Some(32));
        assert_eq!(dot(&[], &[]), Some(0));
        assert_eq!(dot(&[1], &[1, 2]), None);
    }

    #[test]
    fn test_chunk_averages() {
        assert_eq!(
            chunk_averages(&[1.0, 2.0, 3.0, 4.0, 5.0], 2),
            vec![1.5, 3.5, 5.0]
        );
        assert_eq!(chunk_averages(&[2.0, 4.0], 5), vec![3.0]);
        assert!(chunk_averages(&[], 3).is_empty());
    }

    #[test]
    fn test_parse_scores() {
        assert_eq!(
            parse_scores("alice:80\nbad line\nbob:x\ncarol: 95\n :10"),
            vec![("alice".to_string(), 80), ("carol".to_string(), 95)]
        );
    }
}
