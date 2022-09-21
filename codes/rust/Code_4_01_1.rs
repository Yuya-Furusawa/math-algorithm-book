// 問題ID:033のコード
use proconio::input;

fn main() {
  input! {
    a: [isize; 2],
    b: [isize; 2],
    c: [isize; 2],
  };

  let mut ans: f64 = 0.0;

  let vec_ba = [a[0] - b[0], a[1] - b[1]];
  let vec_bc = [c[0] - b[0], c[1] - b[1]];

  let vec_ca = [a[0] - c[0], a[1] - c[1]];
  let vec_cb = [b[0] - c[0], b[1] - c[1]];

  let ip_ba_bc = inner_product(vec_ba, vec_bc);
  let ip_ca_cb = inner_product(vec_ca, vec_cb);

  if ip_ba_bc < 0 {
    // 最短距離は線分ABの長さ
    ans = length(vec_ba);
  } else if ip_ca_cb < 0 {
    // 最短距離は線分ACの長さ
    ans = length(vec_ca);
  } else {
    // 最短距離はAから線分BCに下ろした垂線の長さ
    ans = outer_product(vec_ba, vec_bc) as f64 / length(vec_bc);
  }

  println!("{}", ans);
}

// ２つのベクトルの内積を計算する
fn inner_product(x: [isize; 2], y: [isize; 2]) -> isize {
  x[0] * y[0] + x[1] * y[1]
}

// ２つのベクトルの外戚を計算する
fn outer_product(x: [isize; 2], y: [isize; 2]) -> isize {
  (x[0] * y[1] - x[1] * y[0]).abs()
}

// 線分の長さを計算する
fn length(x: [isize; 2]) -> f64 {
  let fx = [x[0] as f64, x[1] as f64];
  (fx[0] * fx[0] + fx[1] * fx[1]).sqrt()
}
