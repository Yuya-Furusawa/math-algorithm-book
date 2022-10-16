// 問題ID: 049のコード
use proconio::input;

fn main() {
  input! {
    n: usize,
  }

  let mut a = [0; 10000009];
  a[1] = 1;
  a[2] = 1;

  for i in 3..=n {
    // この段階でmodを計算しておく
    a[i] = (a[i-1] + a[i-2]) % 1000000007;
  }

  println!("{}", a[n]);
}
