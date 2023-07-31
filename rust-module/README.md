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
  mod hosting {
    fn add_to_waitlist() {}

    fn seat_at_table() {}
  }

}

pub fn eat_at_restaurant() {
  // absolute path 
  crate::front_of_house::hosting::add_to_waitlist();

  // relative path
  front_of_house::hosting::add_to_waitlist();   
}
```


<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>
