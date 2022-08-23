// 問題ID:025のコード
use proconio::input;

fn main() {
  input! {
    n: usize,
  }

  let mut a_vec = Vec::new();
  for _i in 0..n {
    input! {
      a: f64,
    }
    a_vec.push(a);
  }

  let mut b_vec = Vec::new();
  for _i in 0..n {
    input! {
      b: f64,
    }
    b_vec.push(b);
  }

  let mut ans = 0.0;
  for i in 0..n {
    ans += a_vec[i] * (1.0 / 3.0) + b_vec[i] * (2.0 / 3.0);
  }

  println!("{}", ans);
}
