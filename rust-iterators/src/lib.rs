use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };

  for line in results {
    println!("{}", line);
  }

  Ok(())
}

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(mut args: &[String]) -> Result<Config, &`static str> {
    args.next();// this is a path to our cli

    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Didnt get a query string"),
    }
    
    let filename = match args.next() {
      Some(arg) => arg, // it return a string then query (outside) take the ownership
      None => return Err("Didnt get a file name"),
    }

    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    // config take the ownership of string 
    Ok(Config { query, filename, case_sensitive })
  }    
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      results.push(line)
    }
  }
  results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line)
    }
  }
  results
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, content));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let content = "\
Rust:
safe, fast, productive.
Trust me.";

    assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, content));
  }

}

