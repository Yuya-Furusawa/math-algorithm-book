// 問題ID: 062のコード
use proconio::input;
use std::collections::HashMap;

fn main() {
  input! {
    n: usize,
    k: usize,
  }

  let mut edges = [0; 200009];
  for i in 1..=n {
    input! {
      a_i: usize,
    }

    edges[i] = a_i;
  }

  // 到達した点かどうかを判断するフラグ
  let mut visited = [false; 200009];
  visited[1] = true;
  let mut now = 1;

  // Pathをベクトルで補足する（orderは大事）
  let mut path = Vec::new();
  path.push(1);

  // ループの開始地点
  let mut start_loop = 1;

  // コマを進めていく
  loop {
    let next = edges[now];

    // すでに訪れていたらそこでループが発生している
    if visited[next] {
      start_loop = next;
      break;
    }
    // 訪れていなかったらpathに追加する
    else {
      path.push(next);
      visited[next] = true;
      now = next;
    }
  }

  let mut ans = 0;

  // 1がループの起点になっている場合
  if start_loop == 1 {
    let index = k % path.len();
    ans = path[index];
  }
  // どこかでループが発生している場合
  else {
    let start_loop_index = path.iter().position(|&x| x == start_loop).unwrap();
    let loop_path = path.split_off(start_loop_index);
    if k < path.len() {
      ans = path[k];
    } else {
      let index = (k - path.len()) % loop_path.len();
      ans = loop_path[index];
    }
  }

  println!("{}", ans);
}
