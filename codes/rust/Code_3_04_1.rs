// 問題ID:023のコード
use proconio::input;

fn main() {
  input! {
    n: usize,
  }

  let mut bs = Vec::new();
  for i in 0..n {
    input! {
      b: usize
    }
    bs.push(b);
  }

  let mut rs = Vec::new();
  for i in 0..n {
    input! {
      r: usize,
    }
    rs.push(r);
  }

  // いったんf64にキャストしてから割り算しないとf64にならない
  // 割り算した後にf64にキャストしても意味ない
  let exBs = bs.iter().fold(0, |sum, x| sum + x) as f64 / n as f64;
  let exRs = rs.iter().fold(0, |sum, x| sum + x) as f64 / n as f64;

  println!("{}", exBs + exRs);
}
