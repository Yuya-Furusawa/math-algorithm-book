// 問題ID:021のコード
use proconio::input;

fn main() {
  input! {
    n: usize,
    r: usize,
  }

  let numerator = (n-r+1..n+1).fold(1, |prod, x| prod * x);
  let denominator = (1..r+1).fold(1, |prod, x| prod * x);

  println!("{}", numerator / denominator);
}
