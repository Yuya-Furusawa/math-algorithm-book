use proconio::input;

fn main() {
  input! {
    n: u64,
  }

  let mut v = Vec::new();
  let mut prev = n;

  loop {
    let firstPrime = firstPrime(prev);
    v.push(firstPrime);
    if prev == firstPrime {
      break;
    } else {
      prev /= firstPrime;
    }
  }

  for x in v {
    println!("{}", x);
  }
}

// 最小の素因数を返す
fn firstPrime(num: u64) -> u64 {
  let mut ans = num;
  let mut i: u64 = 2;
  while i * i <= num {
    if num % i == 0 {
      ans = i;
      break;
    }
    i += 1;
  }
  return ans;
}
