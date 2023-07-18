fn main() {
    my_function(11);
}

fn my_function(x: i32) -> i32 {
  println!("The mine function.");
  println!("The value of x is: {}", x);
  let sum = x + 122;
  sum
  // return sum;
}
