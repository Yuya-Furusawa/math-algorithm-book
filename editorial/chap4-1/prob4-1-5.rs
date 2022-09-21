// 問題ID:037
// TODO: FIX
use proconio::input;

fn main() {
  input! {
    one: [f64; 2],
    two: [f64; 2],
    three: [f64; 2],
    four: [f64; 2]
  }

  let mut ans = String::from("No");

  // 両線分が垂直
  if one[0] == two[0] && three[0] == four[0] {
    if one[0] == three[0] {
      let max_one_two = if one[1] > two[1] { one[1] } else { two[1] };
      let min_one_two = if one[1] < two[1] { one[1] } else { two[1] };
      let max_three_four = if three[1] > four[1] { three[1] } else { four[1] };
      let min_three_four = if three[1] < four[1] { three[1] } else { four[1] };

      // 垂直な２つの線分が重なる場合
      if max_one_two <= min_three_four || max_three_four <= min_one_two {
        ans = String::from("Yes");
      } else {
        ans = String::from("No");
      }
    } else {
      ans = String::from("No");
    }
  }
  // 一方の線分だけが垂直
  else if one[0] == two[0] {
    let max_x = if three[0] > four[0] { three[0] } else { four[0] };
    let min_x = if three[0] < four[0] { three[0] } else { four[0] };

    // 線分が直線をまたぐかどうか
    if min_x <= one[0] && one[0] <= max_x {
      let y = ((four[1] - three[1]) / (four[0] - three[0])) * one[0] + ((three[1] * four[0] - three[0] * four[1]) / (four[0] - three[0]));
      let max = if one[1] > two[1] { one[1] } else { two[1] };
      let min = if one[1] < two[1] { one[1] } else { two[1] };

      if y < min || max < y {
        ans = String::from("No");
      } else {
        ans = String::from("Yes");
      }
    } else {
      ans = String::from("No");
    }
  }
  else if three[0] == four[0] {
    let max_x = if one[0] > two[0] { one[0] } else { two[0] };
    let min_x = if one[0] < two[0] { one[0] } else { two[0] };

    // 線分が直線をまたぐかどうか
    if min_x <= three[0] && three[0] <= max_x {
      let y = ((two[1] - one[1]) / (two[0] - one[0])) * three[0] + ((one[1] * two[0] - one[0] * two[1]) / (two[0] - one[0]));
      let max = if three[1] > four[1] { three[1] } else { four[1] };
      let min = if three[1] < four[1] { three[1] } else { four[1] };

      if y < min || max < y {
        ans = String::from("No");
      } else {
        ans = String::from("Yes");
      }
    } else {
      ans = String::from("No");
    }
  }
  // ２つの線分が並行の場合
  else if (two[1] - one[1])/(two[0] - one[0]) == (four[1] - three[1])/(four[0] - three[0]) {
    // 直線のy切片が同じ場合
    if ((one[1] * two[0] - one[0] * two[1]) / (two[0] - one[0])) == ((three[1] * four[0] - three[0] * four[1]) / (four[0] - three[0])) {
      let max_one_two = if one[0] > two[0] { one[0] } else { two[0] };
      let min_one_two = if one[0] < two[0] { one[0] } else { two[0] };
      let max_three_four = if three[0] > four[0] { three[0] } else { four[0] };
      let min_three_four = if three[0] < four[0] { three[0] } else { four[0] };

      // 並行な２つの線分が重なるかどうかで場合分け
      if min_three_four <= max_one_two && min_one_two <= max_three_four {
        ans = String::from("Yes");
      } else {
        ans = String::from("No");
      }
    }
    // y切片が異なる場合は重ならない
    else {
      ans = String::from("No");
    }
  }
  // ２つの線分が垂直でも並行でもない場合
  else {
    let x = (((one[1] * two[0] - one[0] * two[1]) / (two[0] - one[0])) - ((three[1] * four[0] - three[0] * four[1]) / (four[0] - three[0]))) / (((four[1] - three[1]) / (four[0] - three[0])) - ((two[1] - one[1]) / (two[0] - one[0])));
    let max_one_two = if one[0] > two[0] { one[0] } else { two[0] };
    let min_one_two = if one[0] < two[0] { one[0] } else { two[0] };
    let max_three_four = if three[0] > four[0] { three[0] } else { four[0] };
    let min_three_four = if three[0] < four[0] { three[0] } else { four[0] };

    let min_max_x = if max_one_two < max_three_four { max_one_two } else { max_three_four };
    let max_min_x = if min_one_two > min_three_four { min_one_two } else { min_three_four };

    if max_min_x <= x && x <= min_max_x {
      ans = String::from("Yes");
    } else {
      ans = String::from("No");
    }
  }

  print!("{}", ans);
}