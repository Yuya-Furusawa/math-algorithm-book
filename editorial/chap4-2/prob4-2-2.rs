// 問題ID:041
use proconio::input;

fn main() {
  input! {
    t: usize,
    n: usize,
  }

  let mut l = Vec::new();
  let mut r = Vec::new();

  for _i in 0..n {
    input! {
      l_i: usize,
      r_i: usize,
    }

    l.push(l_i);
    r.push(r_i);
  }

  // 差だけに注目する
  let mut x = Vec::new();
  for _i in 0..=t-1 {
    x.push(0_isize);
  }

  for i in 0..n {
    x[l[i]] += 1;
    if r[i] < t {
      x[r[i]] -= 1;
    }
  }

  let mut ans = x[0];
  for i in 0..t-1 {
    println!("{}", ans);
    ans += x[i+1];
  }
  println!("{}", ans);
}
