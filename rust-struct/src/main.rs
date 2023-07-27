
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  // 🟡 In Swift doesnt need to pass &self
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }

}

fn main() {
  let rect = Rectangle {
    width: 30,
    height: 50,
  };

  let rect2 = Rectangle { width: 20, height: 20 };
  let rect3 = Rectangle{ width: 60, height: 60 };
  println!("rect can hold rect1: {}", rect.can_hold(&rect2));
  println!("rect can hold rect2: {}", rect.can_hold(&rect3));

  println!("Rect {:#?}", rect);

  println!(
    "The are of the rectangle is {} square pixels.",
    rect.area()
  );
}