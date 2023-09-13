fn main() {
  let a = 5;
  let b = &mut a; // error 

  let mut c = 10;
  let d = &c;
  *d = 20; // error 
}
