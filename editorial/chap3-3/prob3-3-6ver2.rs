// 問題ID:022のコード
// サンプルコード通りに実装
// HashMapを使わずvecで実装（こっちの方が簡単）
use proconio::input;

fn main() {
  input! {
    n: usize,
  }

  let mut cards = Vec::new();

  for _i in 0..n {
    input! {
      a: usize,
    }
    cards.push(a);
  }

  // ここちゃんと型を付けてあげないとパスしない
  let mut cnt: [usize; 100000] = [0; 100000];
  for j in 0..n {
    cnt[cards[j]] += 1;
  }

  let mut ans = 0;
  for k in 1..50000 {
    ans += cnt[k] * cnt[100000 - k];
  }
  ans += cnt[50000] * (cnt[50000] - 1) / 2;

  println!("{}", ans);
}
