
struct Point<T> {
  x: T,
  y: T,
}

// just a syntax 
// impl<T> Point<T>{
impl<U> Point<U>{
    fn x(&self) -> &U {
      &self.x
    }
}

impl Point<f64> {
  fn y(&self) -> &f64 {
    &self.y
  }
    
}

fn main() {
  let p1 = Point{x: 1, y: 1};
  p1.x();
  p1.y(); // error 
  let p2 = Point{x: 1.0, y: 1.0};
  p2.x();
  p2.y(); // ok 
}

