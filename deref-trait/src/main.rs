
use std::ops::Deref;

struct MyBox<T>(T)

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;

  // fn deref(&sefl) -> &Self::Target {
  fn deref(&sefl) -> &T {
    &self.0 // first item in MyBox tuple 
  }
}

fn main() {
  let x = 5;
  let y = &x;
  let y = MyBox::new(x);

  assert_eq!(5, x);
  assert_eq!(5, *y);
}
