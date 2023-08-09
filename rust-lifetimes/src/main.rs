use std::fmt::Display;

fn longest_with_an_announcemeny<'a, T>(
  x: &'a str,
  y: &'a str,
  ann: T,
) -> &'a str 
where T: Display,
{
  println!("Announcement! {}", ann);
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

fn main() {
  let s: &'static str = "I have a static lifetime";
}
