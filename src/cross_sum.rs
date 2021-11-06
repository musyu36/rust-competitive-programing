use proconio::input;

pub fn run() {
  input! {
      h: usize,
      w: usize,
      n3: [[i32; w]; h]
  }

  let mut v: Vec<Vec<i32>> = vec![vec![Default::default(); w]; h];
  let mut v_w: Vec<i32> = vec![Default::default(); w];
  let mut v_h: Vec<i32> = vec![Default::default(); h];

  for i in 0..h {
    for j in 0..w {
      v_w[j] += n3[i][j];
      v_h[i] += n3[i][j];
    }
  }
  for i in 0..h {
    for j in 0..w {
      v[i][j] += v_w[j];
      v[i][j] += v_h[i];
      v[i][j] -= n3[i][j];
    }
  }

  let dst: Vec<Vec<String>> = v
    .iter()
    .map(|x| x.iter().map(|y| y.to_string()).collect())
    .collect();

  let dst_iter = dst.iter();
  for dst_val in dst_iter {
    println!("{}", dst_val.join(" "));
  }
}
