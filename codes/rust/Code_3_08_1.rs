// 問題ID:032のコード
use proconio::input;

fn main() {
  input! {
    n: usize,
    x: usize,
    mut a: [usize; n],
  }

  // ソート必要
  // ソートするためにaをmutにしている
  a.sort();

  let mut left = 1;
  let mut right = n;

  let mut ans = String::from("No");

  while left <= right {
    let mid = (left + right) / 2;

    if a[mid - 1] == x {
      ans = String::from("Yes");
      break;
    }

    if a[mid - 1] > x {
      right = mid - 1;
    }
    if a[mid - 1] < x {
      left = mid + 1;
    }
  }

  println!("{}", ans)
}
