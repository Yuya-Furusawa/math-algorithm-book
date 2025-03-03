// 問題ID:026のコード
use proconio::input;

fn main() {
  input! {
    n: usize,
  }

  let mut ans: f64 = 0.0;
  for i in 1..=n {
    ans += (1.0 / i as f64)
  }

  print!("{}", ans * n as f64);
}


// アルゴリズム解説
// =============
//
// 特定のコインが初めて出るまでに必要な試行回数の期待値
// Ep = 1 * 1/n + 2 * (n-1/n) * 1/n + 3 * (n-1/n)^2 * 1/n + ...
//    = n
// 更に一般化すると、特定のコイン（たち）が出る確率をPとすると、
// Ep = P^-1
//
// 残り1種類となった状態で必要な試行回数の期待値
// EX1 = n
// 残り2種類となった状態で必要な試行回数の期待値
// EX2 = (2/n)^-1 + EX1 = n/2 + n
//       ※ 第一項：出していない2種類のうちいずれかを出すのに必要な試行回数（一般化を用いる）
//       * 残りの1回の試行回数
// 残り3種類となった状態で必要な試行回数の期待値
// EX3 = (3/n)^-1 + EX2 = n/3 + n/2 + n
// ↓
// 残りN種類となった状態で必要な試行回数の期待値
// EXN = (n/n)^-1 + EXN-1 = n/n + n/(n-1) + ... + n/3 + n/2 + n
//
//
// ポイント：期待値の和を用いる
