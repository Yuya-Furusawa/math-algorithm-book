// 問題ID:031のコード
use proconio::input;
use std::cmp;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  let mut v = vec![0; n];

  let mut prev: bool = false;

  // 1日目のMaxはもちろん勉強するのみ
  v[0] = a[0];

  // 2日目のMaxを求める
  // 1日目休んで2日目に勉強したほうが良いパターン
  if a[1] > a[0] {
    v[1] = a[1];
    prev = true;
  }
  // 1日目に勉強して2日目に休んだほうが良いパターン
  else {
    v[1] = a[0];
    prev = false;
  }

  for i in 2..n {
    // 前日に勉強している時
    if prev {
      // 前日休んで今日勉強したほうが良い
      if v[i-2] + a[i] > v[i-1] {
        v[i] = v[i-2] + a[i];
        prev = true;
      }
      // 前日勉強して今日休んだほうが良い
      else {
        v[i] = v[i-1];
        prev = false;
      }
    }
    // 前日に勉強していない時
    else {
      // 今日はもちろん勉強した方が良い
      v[i] = cmp::max(v[i-2] + a[i], v[i-1] + a[i]);
      prev = true;
    }
  }

  print!("{}", v[n-1])
}
