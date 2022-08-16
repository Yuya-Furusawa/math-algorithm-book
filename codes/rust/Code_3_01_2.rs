// 問題ID:012のコード
use proconio::input;

fn main() {
  input! {
    n: u64,
  }

  let mut ans = String::from("Yes");

  let mut i = 2;
  while i * i <= n {
    if n % i == 0 {
      ans = String::from("No")
    }
    i += 1;
  }

  println!("{}", ans);
}
