// 問題ID:042
use proconio::input;

fn main() {
  input! {
    n: usize,
  }

  let mut ans = 0;
  for i in 1..=n {
    ans += i * num_divisor(i);
  }

  print!("{}", ans);
}

fn num_divisor(num: usize) -> usize {
  let mut v = Vec::new();
  let mut i = 1;

  while (i * i <= num) {
    if (num % i == 0) {
      v.push(i);
      if (i != num / i) {
        v.push(num / i);
      }
    }
    i += 1;
  }

  return v.len();
}
