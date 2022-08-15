use proconio::input;

fn main() {
  input! {
    N: u32,
    X: u32,
    Y: u32,
  }

  let mut cnt = 0;

  for i in 1..N+1 {
    if i % X == 0 || i % Y == 0 {
      cnt += 1;
    }
  }

  println!("{}", cnt);
}
