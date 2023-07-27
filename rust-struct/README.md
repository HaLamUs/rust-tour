# Rust struct

[⬅ Back](../README.md)

## Intro 
Video

<div>
  <a href="https://www.youtube.com/watch?v=n3bPhdiJm9I"><img src="https://img.youtube.com/vi/n3bPhdiJm9I/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>


## Struct 
The way you define a new data type 

```Rust
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}
```

## Tuple struct 
the struct without name fields

```Rust
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);
```

Problem

```Rust
fn main() {
  let width1 = 30;
  let height1 = 50;

  println!(
    "The are of the rectangle is {} square pixels.",
    area(width1, height1)
  );
}

fn area(width: u32, height: u32) -> u32 {
  width * height
}ions.0 * dimensions.1
}
```

We want to make (w, h) is more related together

Fix by tuple 
```Rust
fn main() {
  let rect = (30, 50)

  println!(
    "The are of the rectangle is {} square pixels.",
    area(rect)
  );
}

fn area(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}
```

Fix by struct 
```Rust
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  let rect = Rectangle {
    width: 30,
    height: 50,
  };

  println!(
    "The are of the rectangle is {} square pixels.",
    area(&rect)
  );
  // let sss = rect.height; // error: without ref we already passed the value
}

// ref without taking the ownership
fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}

```

## Implement struct 

```Rust
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  // 🟡 In Swift doesnt need to pass &self
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }

}

```


<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License