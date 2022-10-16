// 問題ID: 051のコード
use proconio::input;

fn main() {
  input! {
    x: usize,
    y: usize,
  }

  // 計算コストを抑えるために、階乗（のmod）は予め計算しておく
  let mut fact = [0; 200009];
  fact[1] = 1;
  for i in 2..=x+y {
    fact[i] = (fact[i-1] * i) % 1000000007;
  }

  // 分母のmodを繰り返し二乗法で計算
  let mut ans = 1;
  let mut p = (fact[x] * fact[y]) % 1000000007;

  for i in 0..30 {
    if 1000000005 & (1 << i) != 0 {
      ans *= p;
      ans %= 1000000007;
    }
    p *= p;
    p %= 1000000007;
  }

  ans = (fact[x+y] * ans) % 1000000007;

  print!("{}", ans);
}