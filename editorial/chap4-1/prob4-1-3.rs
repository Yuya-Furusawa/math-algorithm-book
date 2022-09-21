// 問題ID:035のコード
use proconio::input;
use std::cmp;

fn main() {
  input! {
    one: [f64; 3],
    two: [f64; 3],
  }

  // ２円の中心の距離を計算する
  let d = distance([one[0], one[1]], [two[0], two[1]]);

  let mut ans = 0;

  if d > one[2] + two[2] {
    ans = 5;
  } else if d == one[2] + two[2] {
    ans = 4;
  } else {
    let small_r = if one[2] < two[2] { one[2] } else { two[2] };
    let large_r = if one[2] >= two[2] { one[2] } else { two[2] };
    if d + small_r == large_r {
      ans = 2;
    } else if d + small_r < large_r {
      ans = 1;
    } else {
      ans = 3;
    }
  }

  println!("{}", ans);
}

// ２点間の距離を計算する
fn distance(a:[f64; 2], b:[f64; 2]) -> f64 {
  let x_vec = b[0] - a[0];
  let y_vec = b[1] - a[1];

  (x_vec * x_vec + y_vec * y_vec).sqrt()
}
