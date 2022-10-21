// 問題ID: 056のコード
use proconio::input;
use std::ops::{Mul, Rem};

fn main() {
  input! {
    n: usize,
  }

  let a = Matrix3x3::new(1, 1, 1, 1, 0, 0, 0, 1, 0);
  let b = power(a, n-1, 1000000007);

  let ans = (2 * b.third.left + b.third.mid + b.third.right) % 1000000007;

  println!("{}", ans);
}

// matrixのn乗を各要素modをとったやつを返す
fn power(matrix: Matrix3x3, n: usize, m: usize) -> Matrix3x3 {
  let mut p = matrix;
  let mut q = Matrix3x3::new(1, 0, 0, 0, 1, 0, 0, 0, 1); // 単位行列

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

// 3x3の行列を定義
struct Matrix3x3 {
  first: MatrixRow,
  second: MatrixRow,
  third: MatrixRow,
}

struct MatrixRow {
  left: usize,
  mid: usize,
  right: usize,
}

impl Matrix3x3 {
  fn new(
    top_left: usize,
    top_mid: usize,
    top_right: usize,
    mid_left: usize,
    mid_mid: usize,
    mid_right: usize,
    bottom_left: usize,
    bottom_mid: usize,
    bottom_right: usize,
  ) -> Self {
    let first = MatrixRow {
      left: top_left,
      mid: top_mid,
      right: top_right,
    };
    let second = MatrixRow {
      left: mid_left,
      mid: mid_mid,
      right: mid_right,
    };
    let third = MatrixRow {
      left: bottom_left,
      mid: bottom_mid,
      right: bottom_right,
    };

    Self {
      first, second, third
    }
  }
}

// 行列（の実態）同士の掛け算をimpl
impl Mul for Matrix3x3 {
  type Output = Matrix3x3;

  fn mul(self, other: Matrix3x3) -> Matrix3x3 {
    let top_left = self.first.left * other.first.left + self.first.mid * other.second.left + self.first.right * other.third.left;
    let top_mid = self.first.left * other.first.mid + self.first.mid * other.second.mid + self.first.right * other.third.mid;
    let top_right = self.first.left * other.first.right + self.first.mid * other.second.right + self.first.right * other.third.right;
    let mid_left = self.second.left * other.first.left + self.second.mid * other.second.left + self.second.right * other.third.left;
    let mid_mid = self.second.left * other.first.mid + self.second.mid * other.second.mid + self.second.right * other.third.mid;
    let mid_right = self.second.left * other.first.right + self.second.mid * other.second.right + self.second.right * other.third.right;
    let bottom_left = self.third.left * other.first.left + self.third.mid * other.second.left + self.third.right * other.third.left;
    let bottom_mid = self.third.left * other.first.mid + self.third.mid * other.second.mid + self.third.right * other.third.mid;
    let bottom_right = self.third.left * other.first.right + self.third.mid * other.second.right + self.third.right * other.third.right;

    Matrix3x3::new(top_left, top_mid, top_right, mid_left, mid_mid, mid_right, bottom_left, bottom_mid, bottom_right)
  }
}

// 行列の参照同士の掛け算をimpl
impl Mul<&Matrix3x3> for &Matrix3x3 {
  type Output = Matrix3x3;

  fn mul(self, other: &Matrix3x3) -> Matrix3x3 {
    let top_left = (*self).first.left * (*other).first.left + (*self).first.mid * (*other).second.left + (*self).first.right * (*other).third.left;
    let top_mid = (*self).first.left * (*other).first.mid + (*self).first.mid * (*other).second.mid + (*self).first.right * (*other).third.mid;
    let top_right = (*self).first.left * (*other).first.right + (*self).first.mid * (*other).second.right + (*self).first.right * (*other).third.right;
    let mid_left = (*self).second.left * (*other).first.left + (*self).second.mid * (*other).second.left + (*self).second.right * (*other).third.left;
    let mid_mid = (*self).second.left * (*other).first.mid + (*self).second.mid * (*other).second.mid + (*self).second.right * (*other).third.mid;
    let mid_right = (*self).second.left * (*other).first.right + (*self).second.mid * (*other).second.right + (*self).second.right * (*other).third.right;
    let bottom_left = (*self).third.left * (*other).first.left + (*self).third.mid * (*other).second.left + (*self).third.right * (*other).third.left;
    let bottom_mid = (*self).third.left * (*other).first.mid + (*self).third.mid * (*other).second.mid + (*self).third.right * (*other).third.mid;
    let bottom_right = (*self).third.left * (*other).first.right + (*self).third.mid * (*other).second.right + (*self).third.right * (*other).third.right;

    Matrix3x3::new(top_left, top_mid, top_right, mid_left, mid_mid, mid_right, bottom_left, bottom_mid, bottom_right)
  }
}

// 行列の参照と行列の実態の掛け算をimpl
impl Mul<&Matrix3x3> for Matrix3x3 {
  type Output = Matrix3x3;

  fn mul(self, other: &Matrix3x3) -> Matrix3x3 {
    let top_left = self.first.left * (*other).first.left + self.first.mid * (*other).second.left + self.first.right * (*other).third.left;
    let top_mid = self.first.left * (*other).first.mid + self.first.mid * (*other).second.mid + self.first.right * (*other).third.mid;
    let top_right = self.first.left * (*other).first.right + self.first.mid * (*other).second.right + self.first.right * (*other).third.right;
    let mid_left = self.second.left * (*other).first.left + self.second.mid * (*other).second.left + self.second.right * (*other).third.left;
    let mid_mid = self.second.left * (*other).first.mid + self.second.mid * (*other).second.mid + self.second.right * (*other).third.mid;
    let mid_right = self.second.left * (*other).first.right + self.second.mid * (*other).second.right + self.second.right * (*other).third.right;
    let bottom_left = self.third.left * (*other).first.left + self.third.mid * (*other).second.left + self.third.right * (*other).third.left;
    let bottom_mid = self.third.left * (*other).first.mid + self.third.mid * (*other).second.mid + self.third.right * (*other).third.mid;
    let bottom_right = self.third.left * (*other).first.right + self.third.mid * (*other).second.right + self.third.right * (*other).third.right;

    Matrix3x3::new(top_left, top_mid, top_right, mid_left, mid_mid, mid_right, bottom_left, bottom_mid, bottom_right)
  }
}

// 行列各要素の剰余をimpl
impl Rem<usize> for Matrix3x3 {
  type Output = Matrix3x3;

  fn rem(self, modulus: usize) -> Matrix3x3 {
    let top_left = self.first.left % modulus;
    let top_mid = self.first.mid % modulus;
    let top_right = self.first.right % modulus;
    let mid_left = self.second.left % modulus;
    let mid_mid = self.second.mid % modulus;
    let mid_right = self.second.right % modulus;
    let bottom_left = self.third.left % modulus;
    let bottom_mid = self.third.mid % modulus;
    let bottom_right = self.third.right % modulus;

    Matrix3x3::new(top_left, top_mid, top_right, mid_left, mid_mid, mid_right, bottom_left, bottom_mid, bottom_right)
  }
}

// 行列の参照の各要素の剰余をimpl
impl Rem<usize> for &Matrix3x3 {
  type Output = Matrix3x3;

  fn rem(self, modulus: usize) -> Matrix3x3 {
    let top_left = (*self).first.left % modulus;
    let top_mid = (*self).first.mid % modulus;
    let top_right = (*self).first.right % modulus;
    let mid_left = (*self).second.left % modulus;
    let mid_mid = (*self).second.mid % modulus;
    let mid_right = (*self).second.right % modulus;
    let bottom_left = (*self).third.left % modulus;
    let bottom_mid = (*self).third.mid % modulus;
    let bottom_right = (*self).third.right % modulus;

    Matrix3x3::new(top_left, top_mid, top_right, mid_left, mid_mid, mid_right, bottom_left, bottom_mid, bottom_right)
  }
}
