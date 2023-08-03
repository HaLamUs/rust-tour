use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>>{
  let f = File::create("hello.txt");
  Ok(())
}