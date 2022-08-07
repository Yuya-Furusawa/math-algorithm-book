use proconio::input;

fn main() {
  input! {
    N: u32,
  }

  let mut A = 0;

  for i in 0..N {
    input! {
      a : u32,
    }

    A += a;
  }

  println!("{}", A % 100);
}
