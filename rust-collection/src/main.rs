fn main() {
  let mut v = vec![1, 2, 3, 4, 5];

  for i in &mut v {
    *i += 50;
    // println!("{}", i);
  }
  for i in &v {
    println!("{}", i);
  }

}
