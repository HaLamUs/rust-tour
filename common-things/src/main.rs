fn main() {
  // control flow 
  let number = 6;
  if number < 10 {
      println!("first condition was true");
  } else if number < 22 {
    println!("Second condition was true")
  } else {
    println!("Condition was false")
  }

  let condition = true; 
  let number = if condition { 5 } else { 6 };
}