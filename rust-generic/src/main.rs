
struct Point<T, U> {
  x: T,
  y: U,
}

fn main() {
  let p1 = Point{x: 1, y: 1};
  let p2 = Point{x: 1.0, y: 1.0};
  let p3 = Point{x: 1, y: 1.0};
}

