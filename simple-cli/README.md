# Mini grep

[⬅ Back](../README.md)

## Intro 
Video 1

<div>
  <a href="https://www.youtube.com/watch?v=XYkiwsplDTg"><img src="https://img.youtube.com/vi/XYkiwsplDTg/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>

Video 2

<div>
  <a href="https://www.youtube.com/watch?v=AABHxixn6Cw"><img src="https://img.youtube.com/vi/AABHxixn6Cw/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>

## Program 
We implement mini `grep`

```Rust
let args: Vec<String> = env::args().collect();
```

This will convert list of passing args to collection of String.

We need to define type `Vec<String>` because collection requires a type

## Refactor 
- Ideally a fn should have one responsibility 
- query and filename need connect in some ways ❓
- Centrelize place to handle errors 

If the main function gets too large same the binary crate  

Fix: Split the lib, then the binary crate will call fns in lib 


### Struct can store a ref var 
But we have to take care the lifetime 

```Rust
fn parse_config(args: &[String]) -> Config {
  let query = args[1].clone();
  let filename = args[2].clone();
  Config { query, filename }
}
```

Use `clone()` we don't want to take the ownership 

Problem: `parse_config` is very closely tied to `Config` type. We want decouple this.

Solve: We the public fn into `Config` struct

### Return result type 

```Rust
impl Config {
  fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }
    let query = args[1].clone();
    let filename = args[2].clone();
    Ok(Config { query, filename })
  }    
}

```

## question mark 

```Rust
let contents = fs::read_to_string(config.filename)?;
```

Add question mark at the end if `read_to_string` returns an error type that error type will automatically be returned   


## lib.rs 

Move struct `Config` and `run` fn to lib.rs using `pub` keyword to make these available.

Import `lib.rs` using `use simple_cli::Config;`

Where `simple_cli` is the folder's project 

## Test driven 

We write our test first. then red --> green --> refactor --> loop again

`&str` string slice

```Rust
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let content = "\
    Rust:
    safe, fast, productive.
    Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search(query, content));
  }

}

```

<h1>REMEMBER <h1>
Everytime we return a ref from the function we have to tie the lifetime of that ref to the lifetime of one of input vars 


The return value need lifetime same as `content` because the result in the `content`

```Rust
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
```

Assign var for both if and else 

```Rust
let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };
```

`let case_sensitive = env::var("CASE_INSENSITIVE").is_err();`

`is_err()` will return false if there is an error 

## Set env 

```Bash
export CASE_INSENSITIVE=true
unset CASE_INSENSITIVE
```

## Output to file 

```Bash
cargo run > output.txt 
```



<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>