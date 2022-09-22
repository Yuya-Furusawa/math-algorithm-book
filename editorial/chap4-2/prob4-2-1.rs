// 問題ID:040のコード
use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n-1],
    m: usize,
  }

  let mut b = Vec::new();
  for _i in 0..m {
    input! {
      b_i: usize,
    }
    b.push(b_i);
  }

  // 累積和のベクトル
  let mut x = Vec::new();
  x.push(0);
  for i in 0..n-1 {
    x.push(x[i] + a[i]);
  }

  let mut ans: isize = 0;
  for i in 1..m {
    ans += (x[b[i]-1] as isize - x[b[i-1]-1] as isize).abs();
  }

  print!("{}", ans);
}
