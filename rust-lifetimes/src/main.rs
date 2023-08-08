struct ImportantExcerpt<'a> {
  part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self, announcement: &str) -> &str {
      println!("Attention please: {}", announcement);
      self.part
    }
}

fn main() {
  let novel = String::from("Call me maybe. 2 hours ago");
  // first_sentence is the ref to a string slice of the 1st sentence of the novel 
  let first_sentence = novel.split(',').next().expect("Could not find");
  let i = ImportantExcerpt {
    part: first_sentence,
  };
  // if first_sentence gone the i too 
}
