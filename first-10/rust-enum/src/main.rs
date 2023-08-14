
fn main() {
  let some_value = Some(3);
  match some_value {
    Some(3) => println!("Three"),
    _ => ()
  }

  if let Some(3) = some_value {
    println!("Three");
  }
}