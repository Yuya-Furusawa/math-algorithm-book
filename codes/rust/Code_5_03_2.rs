// 問題ID: 064のコード
use proconio::input;

fn main() {
  input! {
    n: usize,
    k: usize,
  }

  let mut a = [0; 59];
  let mut sum = 0;
  for i in 1..=n {
    input! {
      a_i: usize,
    }
    a[i] = a_i;
    sum += a_i;
  }

  // kとsumの偶奇が一致しない場合は絶対ムリ
  if k % 2 != sum % 2 {
    println!("No");
  } else {
    // sum > k ならどんなに頑張ってもムリ
    if sum > k {
      println!("No")
    } else {
      println!("Yes")
    }
  }
}
