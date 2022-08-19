// 問題ID:022のコード
// 普通にnC2をしてしまうと実行時間エラーで通らない
// 自分で考えた実装（HashMapをつかう実装）
use proconio::input;
use std::collections::HashMap;

fn main() {
  input! {
    n: usize,
  }

  let mut v = Vec::new();

  for _i in 0..n {
    input! {
      a: usize,
    }
    v.push(a);
  }

  let mut map = HashMap::new();

  for i in 0..n {
    let tup: (usize, usize) = if v[i] <= 50000 { (v[i], 100000 - v[i]) } else { (100000 - v[i], v[i]) };
    let value = map.get(&tup);
    if value == None {
      let newValue: (usize, usize) = if v[i] <= 50000 { (1, 0) } else { (0, 1) };
      map.insert(tup, newValue);
    } else {
      let (mut first, mut second) = value.unwrap();
      if v[i] <= 50000 {
        first += 1;
      } else {
        second += 1;
      }
      map.insert(tup, (first, second));
    }
  }

  let mut cnt = 0;

  for (key, value) in map {
    // 50000のときだけは特別扱いする必要あり
    if key == (50000, 50000) {
      let num = value.0;
      cnt += num * (num - 1) / 2;
    } else {
      let (first, second) = value;
      cnt += first * second;
    }
  }

  println!("{}", cnt);
}
