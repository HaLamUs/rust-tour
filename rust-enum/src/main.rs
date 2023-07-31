fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
      // None => None,
      _ => None,
      Some(i) => Some(i +1), // need wrap return in Some 
  }
}

fn main() {
  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);
}