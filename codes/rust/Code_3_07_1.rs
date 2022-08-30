// 問題ID:028のコード
use proconio::input;
use std::cmp;

fn main() {
  input! {
    n: usize,
    h: [isize; n],
  }

  let mut dp = vec![0; n];

  for i in 0..n {
    if i == 0 { dp[i] = 0; }
    else if i == 1 { dp[i] = (h[i] - h[i-1]).abs(); }
    else {
      let v1 = dp[i-1] + (h[i] - h[i-1]).abs();
      let v2 = dp[i-2] + (h[i] - h[i-2]).abs();
      dp[i] = cmp::min(v1, v2);
    }
  }

  println!("{}", dp[n-1]);
}
