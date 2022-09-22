// 問題ID:039のコード
use proconio::input;

fn main() {
  input! {
    n: usize,
    q: usize,
  }

  let mut l = Vec::new();
  let mut r = Vec::new();
  let mut x = Vec::new();

  for _i in 0..q {
    input! {
      l_i: usize,
      r_i: usize,
      x_i: usize,
    }

    l.push(l_i);
    r.push(r_i);
    x.push(x_i);
  }

  let mut a = Vec::new();
  for _i in 0..n {
    a.push(0.0);
  }

  // 計算量を減らすために「差」だけに注目する
  for i in 0..q {
    a[l[i] - 1] += x[i] as f64;
    if r[i] < n {
      a[r[i]] -= x[i] as f64;
    }
  }

  for i in 1..n {
    if a[i] < 0.0 {
      // 区画iのほうが積もっているので>と出力
      print!("{}", ">".to_string());
    } else if a[i] > 0.0 {
      // 区画i+1のほうが積もっているので<と出力
      print!("{}", "<".to_string());
    } else {
      print!("{}", "=".to_string());
    }
  }
}
