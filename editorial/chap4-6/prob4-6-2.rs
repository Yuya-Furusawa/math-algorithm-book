// 問題ID: 052のコード
use proconio::input;

// 操作A: (i+1,j+2)に移動
// 操作B: (i+2,j+1)に移動
// 操作Aを行う回数をa回、操作Bを行う回数をb回とすると、
// 連立方程式より、a = (2Y-X)/3, b=(2X-Y)/3となる
// つまり合計(X+Y)/3回操作をすることになる

fn main() {
  input! {
    x: usize,
    y: usize,
  }

  let mut ans = 0;
  let mut fact = [1; 2000009];

  if (x + y) % 3 == 0 && x <= 2 * y && y <= 2 * x {
    let a = (2 * y - x) / 3;
    let b = (2 * x - y) / 3;

    fact[1] = 1;
    for i in 2..=a+b {
      fact[i] = (fact[i-1] * i) % 1000000007;
    }

    // 繰り返し二乗法
    let mut tmp = 1;
    let mut p = (fact[a] * fact[b]) % 1000000007;

    for i in 0..30 {
      if 1000000005 & (1 << i) != 0 {
        tmp *= p;
        tmp %= 1000000007;
      }
      p *= p;
      p %= 1000000007;
    }

    ans = (fact[a+b] * tmp) % 1000000007;
  }

  print!("{}", ans);
}