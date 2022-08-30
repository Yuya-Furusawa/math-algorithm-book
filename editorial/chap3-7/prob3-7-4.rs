// 問題ID:009のコードをDPを使って解く
use proconio::input;
use std::cmp;

fn main() {
  input! {
    n: usize,
    s: usize,
    a: [usize; n],
  }

  let mut dp = vec![vec![0; s+1]; n+1];
  for i in 1..=n {
    for j in 0..=s {
      if j < a[i-1] {
        dp[i][j] = dp[i-1][j];
      } else {
        dp[i][j] = cmp::max(dp[i-1][j], dp[i-1][j - a[i-1]] + a[i-1]);
      }
    }
  }

  if dp[n][s] == s {
    print!("Yes");
  } else {
    print!("No");
  }
}
