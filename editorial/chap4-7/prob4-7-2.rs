// 問題ID: 055のコード
use proconio::input;
use std::ops::{Mul, Rem};

fn main() {
  input! {
    n: usize,
  }

  let a = Matrix2x2::new(2, 1, 1, 0);
  let b = power(a, n-1, 1000000007);

  let ans = (b.bottom_left + b.bottom_right) % 1000000007;

  println!("{}", ans);
}

// matrixのn乗を各要素modをとったやつを返す
fn power(matrix: Matrix2x2, n: usize, m: usize) -> Matrix2x2 {
  let mut p = matrix;
  let mut q = Matrix2x2::new(1, 0, 0, 1); // 単位行列

  for i in 0..60 {
    if n & (1 << i) != 0 {
      q = &p * &q;
      q = &q % m;
    }
    p = &p * &p;
    p = &p % m;
  }

  q
}

// 2x2の行列を定義
struct Matrix2x2 {
  top_left: usize,
  top_right: usize,
  bottom_left: usize,
  bottom_right: usize,
}

impl Matrix2x2 {
  fn new(
    top_left: usize,
    top_right: usize,
    bottom_left: usize,
    bottom_right: usize,
  ) -> Self {
    Self {
      top_left, top_right, bottom_left, bottom_right
    }
  }
}

// 行列（の実態）同士の掛け算をimpl
impl Mul for Matrix2x2 {
  type Output = Matrix2x2;

  fn mul(self, other: Matrix2x2) -> Matrix2x2 {
    let top_left = self.top_left * other.top_left + self.top_right * other.bottom_left;
    let top_right = self.top_left * other.top_right + self.top_right * other.bottom_right;
    let bottom_left = self.bottom_left * other.top_left + self.bottom_right * other.bottom_left;
    let bottom_right = self.bottom_left * other.top_right + self.bottom_right * other.bottom_right;

    Matrix2x2::new(top_left, top_right, bottom_left, bottom_right)
  }
}

// 行列の参照同士の掛け算をimpl
impl Mul<&Matrix2x2> for &Matrix2x2 {
  type Output = Matrix2x2;

  fn mul(self, other: &Matrix2x2) -> Matrix2x2 {
    let top_left = (*self).top_left * (*other).top_left + (*self).top_right * (*other).bottom_left;
    let top_right = (*self).top_left * (*other).top_right + (*self).top_right * (*other).bottom_right;
    let bottom_left = (*self).bottom_left * (*other).top_left + (*self).bottom_right * (*other).bottom_left;
    let bottom_right = (*self).bottom_left * (*other).top_right + (*self).bottom_right * (*other).bottom_right;

    Matrix2x2::new(top_left, top_right, bottom_left, bottom_right)
  }
}

// 行列の参照と行列の実態の掛け算をimpl
impl Mul<&Matrix2x2> for Matrix2x2 {
  type Output = Matrix2x2;

  fn mul(self, other: &Matrix2x2) -> Matrix2x2 {
    let top_left = self.top_left * (*other).top_left + self.top_right * (*other).bottom_left;
    let top_right = self.top_left * (*other).top_right + self.top_right * (*other).bottom_right;
    let bottom_left = self.bottom_left * (*other).top_left + self.bottom_right * (*other).bottom_left;
    let bottom_right = self.bottom_left * (*other).top_right + self.bottom_right * (*other).bottom_right;

    Matrix2x2::new(top_left, top_right, bottom_left, bottom_right)
  }
}

// 行列各要素の剰余をimpl
impl Rem<usize> for Matrix2x2 {
  type Output = Matrix2x2;

  fn rem(self, modulus: usize) -> Matrix2x2 {
    let top_left = self.top_left % modulus;
    let top_right = self.top_right % modulus;
    let bottom_left = self.bottom_left % modulus;
    let bottom_right = self.bottom_right % modulus;

    Matrix2x2::new(top_left, top_right, bottom_left, bottom_right)
  }
}

// 行列の参照の各要素の剰余をimpl
impl Rem<usize> for &Matrix2x2 {
  type Output = Matrix2x2;

  fn rem(self, modulus: usize) -> Matrix2x2 {
    let top_left = (*self).top_left % modulus;
    let top_right = (*self).top_right % modulus;
    let bottom_left = (*self).bottom_left % modulus;
    let bottom_right = (*self).bottom_right % modulus;

    Matrix2x2::new(top_left, top_right, bottom_left, bottom_right)
  }
}
