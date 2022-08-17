use proconio::input;

fn main() {
  input! {
    // arrayのslice indexはusizeじゃなくてはいけない
    // u32とかにしてしますと2..nの部分でjがu32型になりsliceできない
    n: usize,
  }

  let mut v = Vec::new();

  for _i in 0..n {
    input! {
      a: u64,
    }
    v.push(a);
  }

  let mut x = gcd(v[0], v[1]);
  for j in 2..n {
    x = gcd(x, v[j]);
  }

  println!("{}", x);

}

// 最大公約数を算出する関数
fn gcd(a: u64, b: u64) -> u64 {
  let mut x = a;
  let mut y = b;
  while x >= 1 && y >= 1 {
    if x > y {
      x = x % y;
    } else {
      y = y % x;
    }
  }

  if x >= 1 {
    return x;
  }
  return y;
}

