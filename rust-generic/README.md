# Rust Generic

[⬅ Back](../README.md)

## Intro 
Video

<div>
  <a href="https://www.youtube.com/watch?v=6rcTSxPJ6Bw"><img src="https://img.youtube.com/vi/6rcTSxPJ6Bw/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>

# Generic 
Generics, traits and lifetime are all ways to reduce code duplicate 

Multiple generic types 

```Rust
fn get_largest<T, G, H>(number_list: Vec<i32>) -> i32 {

```

# Trait 

Trait at the moment it's protocol in Swift

## Problem 
Our `find_largest` fn need to hanlde number and char input 

```Rust
fn main() {

  let number_list = vec![34, 50, 25, 100, 65];
    let largest = get_largest(number_list);
    println!("The largest number is {}", largest);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest = get_largest_char(char_list);
    println!("The largest number is {}", largest);
}

fn get_largest(number_list: Vec<i32>) -> i32 {
    let mut largest = number_list[0];
    for number in number_list {
      if number > largest {
        largest = number
      }
    }
    largest
}

fn get_largest_char(number_list: Vec<char>) -> char {
  let mut largest = number_list[0];
  for number in number_list {
    if number > largest {
      largest = number
    }
  }
  largest
}

```

Use generic 

```Rust
fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
  let mut largest = list[0];
  for number in list {
    if number > largest {
      largest = number
    }
  }
  largest
}
```

## U and T could be a same type 

```Rust
struct Point<T, U> {
  x: T,
  y: U,
}

fn main() {
  let p1 = Point{x: 1, y: 1};
  let p2 = Point{x: 1.0, y: 1.0};
  let p3 = Point{x: 1, y: 1.0};
}

```




<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>
