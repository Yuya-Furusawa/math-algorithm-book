// 問題ID: 060のコード
use proconio::input;

fn main() {
  input! {
    n: usize,
  }

  if n % 4 == 0 {
    println!("Second");
  } else {
    println!("First");
  }
}
