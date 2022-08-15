use proconio::input;

fn main() {
  input! {
    N: u32,
    S: u32,
  }

  let mut cnt = 0;

  for i in 1..N+1 {
    for j in 1..N+1 {
      if i + j <= S {
        cnt += 1;
      }
    }
  }

  println!("{}", cnt);
}
