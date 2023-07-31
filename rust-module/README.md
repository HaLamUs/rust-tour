# Rust module

[⬅ Back](../README.md)

## Intro 
Video

<div>
  <a href="https://www.youtube.com/watch?v=5RPXgDQrjio"><img src="https://img.youtube.com/vi/5RPXgDQrjio/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>


## Module 
The way we organize our project structure.

## Crate
A crate is a compilation unit in Rust. Whenever rustc `some_file.rs` is called, `some_file.rs` is treated as the crate file. 
https://doc.rust-lang.org/rust-by-example/crates.html

## Lib 

```Bash
cargo new --lib restaurant
```

`lib.rs` replaces by `main.rs`

<img src="./assets/structure.png">

Everything is private by default 

```Rust
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }

}

pub fn eat_at_restaurant() {
  // absolute path 
  crate::front_of_house::hosting::add_to_waitlist();

  // relative path
  front_of_house::hosting::add_to_waitlist();   
}
```

Struct is private and its properties too.

```Rust
mod back_of_house {
  struct Breakfast {
    toast: String,
    seasonal_fruit: String,
  }

  impl Breakfast {
    fn summer(toast: &str) -> Breakfast {
      Breakfast { toast: String::from(toast), seasonal_fruit: String::from("peaches") }
    }
  }
}
```

`super` keyword

```Rust

fn serve_order() {}

mod back_of_house {
  fn fix_incorrect_order() {
    cook_order();
    super::serve_order();
  }

  fn cook_order() {}
}
```

Enum

```Rust
mod back_of_house {
  pub enum Appetizer {
    Soup,
    Salad,
  }
}

pub fn eat_at_restaurant() {
  let order1 = back_of_house::Appetizer::Soup;
}
```

## Short

```Rust
// use crate::front_of_house::hosting;
use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
  // front_of_house::hosting::add_to_waitlist();
  hosting::add_to_waitlist();
}

use std::fmt::Result;
use std::io::Result as IOResult;


// Before 
use rand:: Rng;
use rand:: ErrorKind::Transient;

use std::io;
use std::io::Write;

// After 
use rand:: {Rng, ErrorKind::Transient};
use std:: {Self, Write};

```

## Publish fn 

```Rust
pub use crate::front_of_house::hosting;

```


<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>
