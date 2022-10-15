// 問題ID: 047模範解答のコード
use proconio::input;
use std::collections::HashMap;

fn main() {
  input! {
    n: usize,
    m: usize,
  }

  // 隣接リストの作成
  let mut neighbors_map = HashMap::new();

  let mut a = [0; 200009];
  let mut b = [0; 200009];

  for i in 1..=m {
    input! {
      a_i: usize,
      b_i: usize,
    }

    a[i] = a_i;
    b[i] = b_i;

    let vec_0 = neighbors_map.entry(a_i).or_insert(Vec::new());
    vec_0.push(b_i);
    let vec_1 = neighbors_map.entry(b_i).or_insert(Vec::new());
    vec_1.push(a_i);
  }

  // どっちのサイドに属するか
  // 0: undefined, 1: 白, 2:黒
  let mut colors = [0; 200009];

  // 深さ優先探索を行う
  for i in 1..=n {
    if colors[i] == 0 {
      colors[i] = 1;
      dfs(i, &neighbors_map, &mut colors);
    }
  }

  let mut ans = "Yes".to_string();

  for i in 1..=m {
    if colors[a[i]] == colors[b[i]] {
      ans = "No".to_string();
      break;
    }
  }

  println!("{}", ans);
}

// 深さ優先探索を行う関数
fn dfs(pos: usize, map: &HashMap<usize, Vec<usize>>, colors: &mut [isize; 200009]) {
  let neighbors = match map.get(&pos) {
    Some(x) => x.to_vec(),
    None => Vec::new(),
  };

  for i in neighbors {
    if colors[i] == 0 {
      colors[i] = 3 - colors[pos];
      dfs(i, map, colors);
    }
  }
}
