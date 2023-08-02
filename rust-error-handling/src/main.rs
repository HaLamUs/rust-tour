use std::fs::{self, File};
use std::io::{Error, ErrorKind, Read};

fn read_username_from_file() -> Result<String, Error> {
  let f = File::create("hellp.txt");
  let mut f = match f {
      Ok(file) => file,
      Err(e) => return Err(e),
  };

  let mut s = String::new();
  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e)
  }
}

fn read_username_from_file2() -> Result<String, Error> {
  let mut f = File::create("hellp.txt")?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}

fn read_username_from_file3() -> Result<String, Error> {
  let mut s = String::new();
  File::create("hellp.txt")?.read_to_string(&mut s)?;
  Ok(s)
}

fn read_username_from_file4() -> Result<String, Error> {
  fs::read_to_string("hello.txt")
}


fn main() {

}