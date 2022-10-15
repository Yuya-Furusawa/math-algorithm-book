// 問題ID: 045のコード
use proconio::input;
use std::collections::HashMap;

fn main() {
  input! {
    n: usize,
    m: usize,
  }

  // 隣接リストを作成する
  let mut neighbors_map = HashMap::new();
  for _i in 0..m {
    input! {
      a_i: usize,
      b_i: usize,
    }

    let vec_0 = neighbors_map.entry(a_i).or_insert(Vec::new());
    vec_0.push(b_i);
    let vec_1 = neighbors_map.entry(b_i).or_insert(Vec::new());
    vec_1.push(a_i);
  }

  let mut ans = Vec::new();

  for i in 1..=n {
    let neighbors = match &neighbors_map.get(&i) {
      Some(x) => x.to_vec(),
      None => Vec::new(),
    };
    let small_index_vec: Vec<usize> = neighbors.iter().filter(|&x| x < &i).cloned().collect();
    if small_index_vec.len() == 1 {
      ans.push(i);
    }
  }

  println!("{}", ans.len());
}
