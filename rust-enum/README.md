# Rust enum

[⬅ Back](../README.md)

## Intro 
Video

<div>
  <a href="https://www.youtube.com/watch?v=DSZqIJhkNCM"><img src="https://img.youtube.com/vi/DSZqIJhkNCM/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>


## Enum 
Enum allows enumerate variants

```Rust
enum IpAddrKind {
  V4, 
  V6,
}

```

Store data inside enum

```Rust
enum IpAddrKind {
  V4(String), 
  V6(String),
}

enum IpAddrKind {
  V4(u8, u8, u8, u8), 
  V6(String),
}


```

## Enum vs Struct 

```Rust
enum Message {
  Quit,
  Move { x: i32, y: i32}, // struct 
  Write(String),
  ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct 
struct MoveMessage {
  x: i32,
  y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

```

Using struct to define data type we need 4 data type within Enum we can grouped under `Message` type


## Implement Enum

```Rust
impl Message {
  fn some_log() {
    println!("Hi Ken!");
  }
}
```


<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>
