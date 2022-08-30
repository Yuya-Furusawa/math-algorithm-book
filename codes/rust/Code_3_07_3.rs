// 問題ID:030のコード
// ナップザック問題のコード
use proconio::input;
use std::cmp;

fn main() {
  input! {
    n: usize,
    w: usize,
  }

  let mut ws = Vec::new();
  let mut vs = Vec::new();

  for _i in 0..n {
    input! {
      w_i: usize,
      v_i: usize,
    }
    ws.push(w_i);
    vs.push(v_i);
  }

  // DPで使う二次元配列
  let mut dp = vec![vec![0; w+1]; n+1];

  for i in 1..=n {
    for j in 0..=w {
      if j < ws[i-1] {
        dp[i][j] = dp[i-1][j];
      } else {
        dp[i][j] = cmp::max(dp[i-1][j], dp[i-1][j-ws[i-1]]+vs[i-1]);
      }
    }
  }

  let mut ans = 0;
  for i in 0..=w {
    ans = cmp::max(ans, dp[n][i]);
  }

  println!("{}", ans);
}
