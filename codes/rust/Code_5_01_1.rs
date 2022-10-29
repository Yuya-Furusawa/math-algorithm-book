// 問題Id: 058のコード
use proconio::input;

fn main() {
  input! {
    n: isize,
    x: isize,
    y: isize,
  }

  // 条件1
  // |x| + |y| <= |n|
  let cond_1: bool = x.abs() + y.abs() <= n;

  // 条件2
  // x+yの偶奇とnの偶奇が一致する
  let cond_2: bool = (x + y) % 2 == n % 2;

  if cond_1 && cond_2 {
    println!("Yes");
  } else {
    println!("No");
  }
}
