// N個の整数の最小公倍数を計算するアルゴリズム
use proconio::input;

fn main() {
  input! {
    n: usize,
  }

  let mut v = Vec::new();

  for _i in 0..n {
    input! {
      a: u64,
    }
    v.push(a);
  }

  let mut x = lcm(v[0], v[1]);
  for j in 2..n {
    x = lcm(x, v[j]);
  }

  println!("{}", x);
}

// 最小公倍数を求める関数
fn lcm(a: u64, b: u64) -> u64 {
  let mut x = a;
  let mut y = b;
  let mut gcm = 0;

  while x >= 1 && y >= 1 {
    if x > y {
      x = x % y;
    } else {
      y = y %x;
    }
  }

  // まずは最大公約数を求める
  if x >= 1 {
    gcm = x;
  } else {
    gcm = y;
  }

  // 最大公約数を使って最小公倍数を求める
  return a / gcm * b;
}
