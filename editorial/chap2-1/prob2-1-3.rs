use proconio::input;

fn main() {
  input! {
    A1: u32,
    A2: u32,
    A3: u32,
  }

  let A = A1 * A2 * A3;

  println!("{}", A)
}
