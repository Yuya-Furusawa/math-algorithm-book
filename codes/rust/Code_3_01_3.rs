// 問題ID:013のコード
use proconio::input;

fn main() {
  input! {
    n: u64,
  }

  let mut v = Vec::new();

  let mut i = 1;
  while i * i <= n {
    if n % i == 0 {
      v.push(i);
      v.push(n/i);
    }
    i += 1;
  }

  for x in v {
    println!("{}", x);
  }
}
