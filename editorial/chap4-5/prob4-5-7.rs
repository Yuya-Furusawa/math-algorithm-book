// 問題ID: 047のコード
// 与えられたグラフが二部グラフかどうか判定するプログラム
use proconio::input;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
  input! {
    n: usize,
    m: usize,
  }

  // 隣接リストの作成
  let mut neighbors_map = HashMap::new();

  let mut start = 0;

  for i in 0..m {
    input! {
      a_i: usize,
      b_i: usize,
    }

    if i == 0 { start = a_i }

    let vec_0 = neighbors_map.entry(a_i).or_insert(Vec::new());
    vec_0.push(b_i);
    let vec_1 = neighbors_map.entry(b_i).or_insert(Vec::new());
    vec_1.push(a_i);
  }

  let mut ans = String::from("Yes");

  let mut side_a = Vec::new();
  let mut side_b = Vec::new();

  // どっちのサイドに属するか
  // 0: undefined, 1: a, 2:b
  let mut side_flag = [0; 200009];

  let mut que_a: VecDeque<usize> = VecDeque::new();
  let mut que_b: VecDeque<usize> = VecDeque::new();

  // 最初の点に関する処理

  // 最初の点はside_aに入れる
  side_a.push(start);
  let start_neighbors = match &neighbors_map.get(&start) {
    Some(x) => x.to_vec(),
    None => Vec::new(),
  };
  side_flag[start] = 1;

  // 最初の点と隣接する点はside_bに入れる
  for i in start_neighbors {
    side_b.push(i);
    que_b.push_back(i);
    side_flag[i] = 2;
  }

  'outer_loop: while !que_a.is_empty() || !que_b.is_empty() {
    // side Aに属する点を調査
    if !que_a.is_empty() {
      let front_a = que_a.pop_front().unwrap();
      let neighbors_a = match &neighbors_map.get(&front_a) {
        Some(x) => x.to_vec(),
        None => Vec::new(),
      };

      for i in neighbors_a {
        // 同じサイドの点とつながっていたらそれは2部グラフではない
        if side_flag[i] == 1 {
          ans = String::from("No");
          break 'outer_loop;
        }
        // まだ捜索してない点なら、キューとベクトルに入れておく
        if side_flag[i] == 0 {
          side_b.push(i);
          que_b.push_back(i);
          side_flag[i] = 2;
        }
      }
    }
    // side Bに属する点を調査
    if !que_b.is_empty() {
      let front_b = que_b.pop_front().unwrap();
      let neighbors_b = match &neighbors_map.get(&front_b) {
        Some(x) => x.to_vec(),
        None => Vec::new(),
      };

      for i in neighbors_b {
        // 同じサイドの点とつながっていたらそれは2部グラフではない
        if side_flag[i] == 2 {
          ans = String::from("No");
          break 'outer_loop;
        }
        // まだ捜索してない点なら、キューとベクトルに入れておく
        if side_flag[i] == 0 {
          side_a.push(i);
          que_a.push_back(i);
          side_flag[i] = 1;
        }
      }
    }
  }

  println!("{}", ans);
}
