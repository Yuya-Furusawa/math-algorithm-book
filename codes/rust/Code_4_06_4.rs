// 問題ID: 050のコード
// 繰り返し二乗法を使うパターン
use proconio::input;

fn main() {
  input! {
    a: usize,
    b: usize,
  }

  let mut ans = 1;
  let mut p = a;

  for i in 0..30 {
    if b & (1 << i) != 0 {
      ans *= p;
      ans %= 1000000007;
    }
    p *= p;
    p %= 1000000007;
  }

  print!("{}", ans);
}
