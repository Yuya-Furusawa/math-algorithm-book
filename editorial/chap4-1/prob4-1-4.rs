// 問題ID:036
use proconio::input;
use std::f64::consts::PI;

fn main() {
  input! {
    a: f64,
    b: f64,
    h: f64,
    m: f64,
  }

  // 分針の角度を計算
  let a_rad = 2.0 * PI * m / 60.0;
  let a_pos = [a * a_rad.cos(), a * a_rad.sin()];

  // 時針の角度を計算
  let b_rad = 2.0 * PI * (h / 12.0) + (2.0 * PI / 12.0) * (m / 60.0);
  let b_pos = [b * b_rad.cos(), b * b_rad.sin()];

  let ans = length([b_pos[0] - a_pos[0], b_pos[1] - a_pos[1]]);

  println!("{}", ans);
}

// ベクトルの長さを計算
fn length(x: [f64; 2]) -> f64 {
  (x[0] * x[0] + x[1] * x[1]).sqrt()
}
