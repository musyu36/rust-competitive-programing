use proconio::input;
use std::mem;

fn main() {
  input! {
      a: isize,
      b: isize,
      c: isize,
  }
  let r = gcd(a, gcd(b, c));
  println!("{}", (a / r) + (b / r) + (c / r) - 3);
}

fn gcd(mut a: isize, mut b: isize) -> isize {
  if b > a {
    mem::swap(&mut a, &mut b);
  }
  if a % b == 0 {
    return b;
  } else {
    return gcd(b, a % b) as isize;
  }
}
