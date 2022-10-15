// 問題ID: 046のコード
use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;
use std::str;

fn main() {
  input! {
    r: usize,
    c: usize,
    start: [usize; 2],
    goal: [usize; 2],
  }

  // 迷路のマップをベクトルとして保存する
  let mut map: Vec<Vec<char>> = Vec::new();

  // 距離のベクトル
  let mut dist: Vec<Vec<isize>> = Vec::new();

  // すでに訪れているマスかどうかの確認
  let mut visited: Vec<Vec<bool>> = Vec::new();

  // マップと距離を作成する
  // インデックスを調整するためにrowとcolumnを1つ増やして作成する
  for i in 0..=r {
    let mut map_row = Vec::new();
    let mut dist_row = Vec::new();
    let mut visited_row = Vec::new();

    // 0行目は#で埋める
    if i == 0 {
      for j in 0..=c {
        map_row.push('#');
        dist_row.push(-1);
        visited_row.push(false);
      }
    } else {
      // 0列目は'#'を入れておく
      map_row.push('#');

      input! {
        c_row: Chars,
      }
      for i in c_row {
        map_row.push(i);
      }

      // distに初期値を入れておく
      for j in 0..=c {
        dist_row.push(-1);
        visited_row.push(false);
      }
    }
    map.push(map_row);
    dist.push(dist_row);
    visited.push(visited_row);
  }

  // キューを用いる
  let mut que: VecDeque<[usize; 2]> = VecDeque::new();

  // スタートの点に対する操作
  dist[start[0]][start[1]] = 0;
  visited[start[0]][start[1]] = true;
  que.push_back([start[0], start[1]]);

  while que.front() != None {
    // キューの先頭を取り出す
    let front: [usize; 2] = que.pop_front().unwrap();

    let mut neighbors: Vec<[usize; 2]> = Vec::new();

    // 右のマスを調べる
    if map[front[0]+1][front[1]] == '.' {
      neighbors.push([front[0]+1, front[1]]);
    }
    // 左のマスを調べる
    if map[front[0]-1][front[1]] == '.' {
      neighbors.push([front[0]-1, front[1]]);
    }
    // 下のマスを調べる
    if map[front[0]][front[1]+1] == '.' {
      neighbors.push([front[0], front[1]+1]);
    }
    // 上のマスを調べる
    if map[front[0]][front[1]-1] == '.' {
      neighbors.push([front[0], front[1]-1]);
    }

    for neighbor in neighbors {
      if visited[neighbor[0]][neighbor[1]] == false {
        dist[neighbor[0]][neighbor[1]] = dist[front[0]][front[1]] + 1;
        que.push_back([neighbor[0], neighbor[1]]);
        visited[neighbor[0]][neighbor[1]] = true;
      }
    }
  }

  println!("{}", dist[goal[0]][goal[1]]);
}
