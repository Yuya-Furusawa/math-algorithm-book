// 問題ID: 065のコード
use proconio::input;

fn main() {
  input! {
    h: usize,
    w: usize,
  }

  let mut ans = 0;

  // w=1 or h=1のときは特殊ケース
  if w == 1 || h == 1 {
    println!("1");
  } else {
    // wの偶奇で場合分け
    if w % 2 == 0 {
      ans = (w / 2) * h;
      println!("{}", ans);
    } else {
      // hの偶奇で場合分け
      if h % 2 == 0 {
        ans = w * (h / 2);
        println!("{}", ans);
      } else {
        ans = w * (h / 2) + ((w + 1) / 2);
        println!("{}", ans);
      }
    }
  }
}
