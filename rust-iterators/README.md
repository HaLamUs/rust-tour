# Iterators in Rust

[⬅ Back](../README.md)

## Intro 
Video 

<div>
  <a href="https://www.youtube.com/watch?v=4GcKrj4By8k"><img src="https://img.youtube.com/vi/4GcKrj4By8k/0.jpg" alt="IMAGE ALT TEXT"></a>
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



<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>