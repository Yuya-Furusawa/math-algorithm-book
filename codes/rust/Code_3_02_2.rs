// 問題ID:015のコード
// ユークリッドの互除法の実装
use proconio::input;

fn main() {
  input! {
    a: i32,
    b: i32,
  }

  let mut x = a;
  let mut y = b;

  while x >= 1 && y >= 1 {
    if x > y {
      x = x % y;
    } else {
      y = y % x;
    }
  }

  if x >= 1 {
    println!("{}", x);
  } else {
    println!("{}", y);
  }
}
