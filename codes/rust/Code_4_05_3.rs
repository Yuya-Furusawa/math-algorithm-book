// 問題ID: 044のコード
// 幅優先探索（Breadth First Search, BFS）
use proconio::input;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
  input! {
    n: usize,
    m: usize,
  }

  // 隣接する点のHashMapを作る
  // 毎度隣接する点を計算するのはコストなので、一度すべての計算を行っておく
  let mut neighbors_map = HashMap::new();
  for _i in 0..m {
    input! {
      a_i: usize,
      b_i: usize,
    }
    let count_vec_0 = neighbors_map.entry(a_i).or_insert(Vec::new());
    count_vec_0.push(b_i);
    let count_vec_1 = neighbors_map.entry(b_i).or_insert(Vec::new());
    count_vec_1.push(a_i);
  }

  let mut dist = [-1; 100009];
  let mut visited = [false; 100009];

  // キューを用いる
  let mut que: VecDeque<usize> = VecDeque::new();

  // 頂点１に対する操作
  dist[1] = 0;
  visited[1] = true;
  que.push_back(1);

  while que.front() != None {
    // キューの先頭を取り出す
    let front = que.pop_front().unwrap();

    let neighbors = match &neighbors_map.get(&front) {
      Some(x) => x.to_vec(),
      None => Vec::new(),
    };

    for i in neighbors {
      if visited[i] == false {
        dist[i] = dist[front] + 1;
        que.push_back(i);
        visited[i] = true;
      }
    }
  }

  for j in 1..=n {
    println!("{}", dist[j]);
  }
}