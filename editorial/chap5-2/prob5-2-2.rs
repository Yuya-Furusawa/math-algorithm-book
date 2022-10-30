// 問題ID: 061のコード
use proconio::input;

fn main() {
  input! {
    n: usize,
  }

  if (n+1).is_power_of_two() {
    println!("Second");
  } else {
    println!("First");
  }
}
