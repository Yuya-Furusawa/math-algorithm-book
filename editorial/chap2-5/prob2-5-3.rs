use proconio::input;

fn main() {
  input! {
    // 64-bitじゃないと通らない
    n: u64,
  }

  let mut fact = 1;

  for i in 1..n+1 {
    fact *= i;
  }

  println!("{}", fact);
}
