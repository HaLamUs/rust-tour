fn main() {
  let r;
  {
    let x = 6;
    r = &x;
  } // x ends here so r is invalid pointer - dangling 

  println!("r: {}", r);
}
