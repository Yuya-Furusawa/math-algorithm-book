use proconio::input;

fn main() {
  input! {
    n: i32,
  }

  let mut v = Vec::new();

  for i in 2..n+1 {
    if isPrime(i) == true {
      v.push(i);
    }
  }

  for x in v {
  	println!("{}", x);
  }
}

// 素数かどうかを判定する関数
fn isPrime(i: i32) -> bool {
  for j in 1..i+1 {
    // 割り切れてかつ、1でもその数字自身でもない
  	if i % j == 0 && j != 1 && j != i {
      return false;
    }
  }
  return true;
}
