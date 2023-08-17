
pub trait Iterator {
  type Item;

  fn next(&mut self) -> Option<Self::Item>;
}

#[test]
fn iterator_demonstration() {
  let v1 = vec![1, 2, 3];

  let mut v1_iter = v1.into_iter();

  assert_eq!(v1_iter.next(), Some(1)); // &1 is the number in v1 
  assert_eq!(v1_iter.next(), Some(2));
  assert_eq!(v1_iter.next(), Some(3));
  assert_eq!(v1_iter.next(), None);

}




fn main() {
  let v1 = vec![1, 2, 3];
  let v1_iter = v1.iter();

  for value in v1_iter {
    println!("Got: {}", value);
  }
}
