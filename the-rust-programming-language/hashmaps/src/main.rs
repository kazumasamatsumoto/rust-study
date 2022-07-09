#![allow(unused)]
fn main() {
    practice1();
}

fn practice1() {
    let v = vec![5, 2, 5, 6, 7, 8, 5, 7, 8, 5, 3, 2, 6, 3];
    println!("vec: {:?}", v);
    println!("mean: {}", mean(&v));
    println!("median: {}", median(&v));
    println!("mode: {}", mode(&v));
}

fn mean(v: &Vec<i32>) -> f64 {
    let mut sum: f64 = 0.0;
    for i in v {
        sum += f64::from(*i);
    }
    sum / v.len() as f64 // v.lenはi32のためf64に変換する必要がある
}

fn median(v: &Vec<i32>) -> i32 {
    let mut s = Vec::new();
    for i in v { s.push(i); }
    s.sort();
    **s.get(s.len()/2).unwrap()
}

fn mode(v: &Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut h = HashMap::new();
    for i in v {
        let count = h.entry(i).or_insert(0);
        *count += 1;
    }
    let mut mode = 0;
    let mut max_count = 0;
    for (key, value) in h {
        if max_count < value { max_count = value; mode = *key; }
    }
    mode
}