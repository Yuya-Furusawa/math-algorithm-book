// 問題ID:019のコード
use proconio::input;

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

  let mut x: i64 = 0;
  let mut y: i64 = 0;
  let mut z: i64 = 0;

  for i in 0..n {
    if v[i] == 1 {
      x += 1;
    }
    if v[i] == 2 {
      y += 1;
    }
    if v[i] == 3 {
      z += 1;
    }
  }

  let cnt = x * (x - 1) / 2 + y * (y - 1) / 2 + z * (z - 1) /2;

  println!("{}", cnt);
}
