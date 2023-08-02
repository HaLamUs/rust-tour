
use unicode_segmentation::UnicodeSegmentation;
fn main() {
  let hello = String::from("Hello");
  // let c = hello[0]; // error 
  for b in hello.bytes() {
    println!("{}", b);
  }

  for b in hello.chars() {
    println!("{}", b);
  }

  for b in hello.graphemes(true) {
    println!("{}", b);
  }
}
