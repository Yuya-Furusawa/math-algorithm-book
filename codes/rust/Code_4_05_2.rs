// 問題ID: 043のコード
// 深さ優先探索(Depth First Search, DFS)
use proconio::input;
use std::collections::HashMap;

fn main() {
  input! {
    n: usize,
    m: usize,
  }

  let mut edges = Vec::new();

  for _i in 0..m {
    input! {
      a_i: usize,
      b_i: usize,
    }

    edges.push([a_i, b_i]);
  }

  // 隣接する点のHashMapを作る
  // 毎度隣接する点を計算するのはコストなので、一度すべての計算を行っておく
  let mut neighbors_map = HashMap::new();
  for edge in edges {
    let count_vec_0 = neighbors_map.entry(edge[0]).or_insert(Vec::new());
    count_vec_0.push(edge[1]);
    let count_vec_1 = neighbors_map.entry(edge[1]).or_insert(Vec::new());
    count_vec_1.push(edge[0]);
  }

  // 訪れたかどうかを管理する配列をつくる
  // すべての頂点を白色に塗る
  let mut visited = [false; 100009];

  // 探索のアルゴリズムを再帰的に呼び出す
  dfs(1, &mut visited, &neighbors_map);

  let mut ans = true;
  for i in 1..=n {
    if visited[i] == false {
      ans = false;
      break;
    }
  }

  if ans {
    println!("The graph is connected.");
  } else {
    println!("The graph is not connected.");
  }

}

// 探索のメインアルゴリズム
fn dfs(
  point: usize,
  visited: &mut [bool; 100009], //可変参照を渡すことによって内部でvisitedの変更を行う
  neighbors_map: &HashMap<usize, Vec<usize>>
) {
  // 訪れた頂点を灰色に塗る
  visited[point] = true;

  // 頂点1が孤立している場合はインデックスアクセスでREになるので、getを使って丁寧に場合分けする必要がある
  let neighbors = match &neighbors_map.get(&point) {
    Some(x) => x.to_vec(),
    None => Vec::new(),
  };
  // 教科書的には小さい数字から探索しているが、本質的には変わらないのでソートしない
  // ソートは計算量大きい
  for i in neighbors {
    if visited[i] == false {
      dfs(i, visited, neighbors_map);
    }
  }
}
