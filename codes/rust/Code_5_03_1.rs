// 問題ID: 063のコード
use proconio::input;

fn main() {
  input! {
    n: usize,
  }

  if n % 2 == 0 {
    println!("Yes")
  } else {
    println!("No")
  }
}
