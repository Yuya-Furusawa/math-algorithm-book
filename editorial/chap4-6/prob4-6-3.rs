// 問題ID: 053のコード
use proconio::input;

fn main() {
  input! {
    n: u64,
  }

  print!("{}", comp_bn_sum(n));
}

// a^b(mod m)を返す関数
// オーバーフローを防ぐためにu64で型指定する
fn modpow(a: u64, b: u64, m: u64) -> u64 {
  let mut ans = 1;
  let mut p = a;

  // 2^60くらいまで計算しておく
  for i in 0..60 {
    if b & (1 << i) != 0 {
      ans *= p;
      ans %= m;
    }
    p *= p;
    p %= m;
  }

  ans
}

// 和の計算を二分探索的・再帰的に行っていく関数
// 4^0 + a^1 + ... + 4^lを計算する
fn comp_bn_sum(l: u64) -> u64 {
  if  l == 1 {
    return 1 + 4;
  }
  else if l == 2 {
    return 1 + 4 + 16;
  }
  else {
    // ちょうど半分に割れるとき
    if (l + 1) % 2 == 0 {
      let d = (l + 1) / 2;
      return (comp_bn_sum(d-1) + modpow(4, d, 1000000007) * comp_bn_sum(d-1)) % 1000000007;
    }
    // ちょうど半分に割れないときは真ん中の項が残る
    else {
      let d = l / 2;
      return (comp_bn_sum(d-1) + modpow(4, d, 1000000007) + modpow(4, d+1, 1000000007) * comp_bn_sum(d-1)) % 1000000007;
    }
  }
}
