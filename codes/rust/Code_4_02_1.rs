// 問題ID:038のコード
use proconio::input;

fn main() {
  input! {
    n: usize,
    q: usize,
    a: [usize; n],
  }

  let mut l = Vec::new();
  let mut r = Vec::new();

  for i in 0..q {
    input! {
      l_i: usize,
      r_i: usize,
    }

    l.push(l_i);
    r.push(r_i);
  }

  let mut b = Vec::new();

  b.push(a[0]);
  for i in 1..n {
    b.push(a[i] + b[i-1]);
  }

  for i in 0..q {
    let right = b[r[i] - 1];
    // isizeにしないとオーバーフローしちゃう
    let left = if l[i] as isize - 1 - 1 < 0 { 0 } else { b[l[i] - 1 - 1] };
    println!("{}", right - left);
  }
}
