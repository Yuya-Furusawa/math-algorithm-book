// 問題ID:034のコード
use proconio::input;

fn main() {
  input! {
    n: usize
  }

  let mut x = Vec::new();
  let mut y = Vec::new();

  for i in 0..n {
    input! {
      x_i: usize,
      y_i: usize,
    }

    x.push(x_i);
    y.push(y_i);
  }

  let mut tmp = 0.0;

  for i in 0..n {
    for j in i+1..n {
      let d = distance([x[i], y[i]], [x[j], y[j]]);
      if tmp == 0.0 || d < tmp {
        tmp = d;
      }
    }
  }

  println!("{}", tmp);
}

// ２点間の距離を計算する
fn distance(a: [usize; 2], b: [usize; 2]) -> f64 {
  let x_vec = b[0] as f64 - a[0] as f64;
  let y_vec = b[1] as f64 - a[1] as f64;

  (x_vec * x_vec + y_vec * y_vec).sqrt()
}
