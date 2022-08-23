// 問題ID:024のコード
use proconio::input;

fn main() {
  input! {
    n: usize,
  }

  let mut ps = Vec::new();
  let mut qs = Vec::new();

  for _i in 0..n {
    input! {
      p: f64,
      q: f64,
    }

    ps.push(p);
    qs.push(q);
  }

  let mut ans: f64 = 0.0;
  for i in 0..n {
    ans += qs[i] / ps[i];
  }

  println!("{}", ans);
}
