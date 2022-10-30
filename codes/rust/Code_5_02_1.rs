// 問題ID: 059のコード
use proconio::input;

fn main() {
  input! {
    n: usize,
  }

  if n % 4 == 0 { println!("6"); }
  if n % 4 == 1 { println!("2"); }
  if n % 4 == 2 { println!("4"); }
  if n % 4 == 3 { println!("8"); }
}