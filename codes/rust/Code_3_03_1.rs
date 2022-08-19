// 問題ID:020のコード
use proconio::input;
use itertools::Itertools;

fn main() {
  input! {
    n: usize,
  }

  let mut v = Vec::new();

  for _i in 0..n {
    input! {
      a: u32,
    }
    v.push(a);
  }

  let mut cnt = 0;

  for comb in v.iter().combinations(5) {
    let total = comb.iter().fold(0, |sum, x| sum + **x);
    if total == 1000 {
      cnt += 1;
    }
  }

  println!("{}", cnt);
}
