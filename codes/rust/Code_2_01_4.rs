use proconio::input;

fn main() {
  input! {
    A1: u32,
    A2: u32,
    A3: u32,
  }

  let A1_str: String = A1.to_string();
  let A2_str: String = A2.to_string();
  let A3_str: String = A3.to_string();

  let A = A1_str + &A2_str + &A3_str;

  println!("{}", A)
}
