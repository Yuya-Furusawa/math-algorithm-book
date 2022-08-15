// 問題ID:009のコード
use proconio::input;
use itertools::Itertools;

fn main() {
  input! {
    N: usize,
    S: u32,
  }

  let mut ans = String::from("No");

  let mut v = Vec::new();

  for i in 0..N {
    input! {
      A: u32,
    }
    v.push(A);
  }

  'outer: for i in 0..N+1 {
    for comb in v.iter().combinations(i) {
      let total = comb.iter()
                      .map(|x| **x)
                      .fold(0, |sum, x| sum + x);
      if total == S {
        ans = String::from("Yes");
        break 'outer;
      }
    }
  }

  println!("{}", ans);
}
