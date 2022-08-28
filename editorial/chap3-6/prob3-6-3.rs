// 問題ID:027のコード
// マージソートアルゴリズムの実装
use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  for i in mergeSort(a){
    print!("{} ", i);
  }
}

fn mergeSort(array: Vec<usize>) -> Vec<usize> {
  // 1つだけならそのまま返す
  if array.len() == 1 {
    return array;
  }

  // 配列を２つに分割し再帰的にソートを行う
  // 返ってきたarrayAとarrayBはすでにソート済みである
  let m = array.len() / 2;
  let arrayA = mergeSort((&array[0..m]).to_vec());
  let arrayB = mergeSort((&array[m..array.len()]).to_vec());

  let mut c1 = 0;
  let mut c2 = 0;
  let mut c = Vec::new();
  while c1 != arrayA.len() || c2 != arrayB.len() {
    // arrayAが空になっている時
    // arrayBの値をpushする
    if c1 == arrayA.len() {
      c.push(arrayB[c2]);
      c2 += 1;
    }
    // arrayBが空になっている時
    // arrayAの値をpushする
    else if c2 == arrayB.len() {
      c.push(arrayA[c1]);
      c1 += 1;
    }
    // arrayAとarrayBのうち一番小さい値をpushする
    else {
      if arrayA[c1] > arrayB[c2] {
        c.push(arrayB[c2]);
        c2 += 1;
      } else {
        c.push(arrayA[c1]);
        c1 += 1;
      }
    }
  }

  return c;
}
