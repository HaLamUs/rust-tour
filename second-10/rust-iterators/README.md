# Iterators in Rust

[⬅ Back](../../README.md)

## Intro 
Video 1

<div>
  <a href="https://www.youtube.com/watch?v=4GcKrj4By8k"><img src="https://img.youtube.com/vi/4GcKrj4By8k/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>

Video 2

<div>
  <a href="https://www.youtube.com/watch?v=rb63xJEjaZU"><img src="https://img.youtube.com/vi/rb63xJEjaZU/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>

## Iterator pattern
Iterate over a sequence of elements regardless how elements are stored (Ex: array, custom data ...)
Iterator encapsulates the logic for iterating over different data structures in a uniform way 

```Rust
let v1_iter = v1.iter(); // this is lazy 
```

By implement iterator trait. Any data structure can iterate

```Rust
pub trait Iterator {
  type Item;

  fn next(&mut self) -> Option<Self::Item>;
}
```

Iterator trait provides some default method 

```Rust
let total: i32 = v1_iter.sum(); // total needs exciplit type
```

## Map 
Map which takes in a closure and create an iterator which calls the closure over each element in the sequence 

```Rust
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
```

## Example

```Rust
#[derive(PartialEq, Debug)]
struct Shoe {
  size: u32,
  style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
  shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
```

## Custom iterator 

```Rust
struct Counter {
  count: u32,
}

impl Counter {
  fn new() -> Counter {
    Counter { count: 0 }
  }
}

impl Iterator for Counter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.count < 5 {
      self.count += 1;
      Some(self.count)
    } else {
      None
    }
  }  
}

```

```Rust

#[test]
fn using_other_iterator_trait_methods() {
  let sum: u32 = Counter::new()
      .zip(Counter::new().skip(1))
      .map(|(a, b)| a * b)
      .filter(|x| x % 3 == 0)
      .sum()
  assert_eq!(18, sum);  
}

```

🟡 Return 'None' break the loop 🤔

zip method will take 2 iterators and zip them up into one iterator containing pairs of value

skip will create an iterator that skips the first n elements 

map takes a closure which it will call for each item in the iterator in this case each item has a pair of values because we just call the zip method 


## Problem 

```Rust

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }
    
    let query = args[1].clone(); //Clone is  Not efficient
    let filename = args[2].clone(); //Clone is  Not efficient

    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config { query, filename, case_sensitive })
  }    
}

```

## Solution

```Rust
impl Config {
  pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
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


```

`pub fn new(mut args: env::Args)` need `mut` coz iterate 

`Result<Config, &'static str> ` uses static because Args is a custom type and string slice we yield error string `Didnt get a query string` is last long as the program does 

## Conclusion 
Using loops or using iterators has the same performance - same speed 




<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>