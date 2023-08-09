
pub fn greeting(name: &str) -> String {
  // return format!("Hello {}!", name);
  return format!("Hello");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn greeting_contains_name() {
    let result = greeting("Carol");
    // assert!(result.contains("Carol"));
    assert!(
      result.contains("Carol"),
      "Greeting did not contain name, value was {}", result
    )
  }

}
