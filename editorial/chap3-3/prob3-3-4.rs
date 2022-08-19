// 問題ID:018のコード
use proconio::input;

fn main() {
  input! {
    n: usize,
  }

  let mut v = Vec::new();

  for _i in 0..n {
    input! {
      a: i32,
    }
    v.push(a);
  }

  // i64にしないとオーバーフローしてしまう
  let mut a: i64 = 0;
  let mut b: i64 = 0;
  let mut c: i64 = 0;
  let mut d: i64 = 0;
  for i in 0..n {
    if v[i] == 100 {
      a += 1;
    }
    if v[i] == 200 {
      b += 1;
    }
    if v[i] == 300 {
      c += 1;
    }
    if v[i] == 400 {
      d += 1;
    }
  }

  println!("{}", a * d + b * c);
}
