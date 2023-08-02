# Rust Collections

[⬅ Back](../README.md)

## Intro 
Video

<div>
  <a href="https://www.youtube.com/watch?v=Zs-pS-egQSs"><img src="https://img.youtube.com/vi/Zs-pS-egQSs/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>

## Collection 
Collection is unlike Array, Enum. It allocated on Heap, which means it's size can grow or shrink as needed.
Ex: Vector, string, hashmap 

## Vector 

```Rust
let mut v = vec![1, 2, 3, 4, 5];

  let third = &v[2]; 
  match v.get(2) {
      Some(third) => println!("The 3rd element is {}", third),
      None => println!("3rd element 404"),
  }
```

Ampersand 

https://stackoverflow.com/a/43036358

```Rust

  let v2 = vec![1, 2, 3];
  for i in &v2 {}
  for i in &v2 {}

  let v3 = vec![1, 2, 3];
  for i in v3 {}
  for i in v3 {} // <--  Error 


```
## Ampersand in Depth

a new Rust developer randomly inserting ampersands to appease the Rust compiler. 

### Problem:
Imagine we have a simple function that calculates the length of a string. This function needs to look at the string. 

1. But does it need permission to modify it? No. 
2. When the length function is finished, should we drop the string from memory? No. 
3. This means the length function needs only read-access and it only needs a temporary view of the string rather than a permanent version. 

🔑 This is what the `&variable` notation means in Rust. 

https://fiberplane.com/blog/getting-past-ampersand-driven-development-in-rust

So the ampersand using in the video just a randomly use 


Borrow and change value 

```Rust
let mut v = vec![1, 2, 3, 4, 5];

  for i in &mut v { // &mut v mean borrow as mutable
    *i += 50;
    println!("{}", i);
  }
```

```Rust

enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}


let row = vec![
  SpreadsheetCell:: Int(3),
  SpreadsheetCell::Text(String::from("blue")),
  SpreadsheetCell::Float(10.12),
]

match &row[1] {
  SpreadsheetCell::Int(i) => printl!("{}", i),
  _ => println!("Not a integer!")
}

```

## String 
Strings are stored as a collection of UTF-8 encoded bytes 

### Ascii
Ascii is the string ENCODING take ones and zeros turn it into a string or tale a string and turn it into ones and zero.

Each ascii char stored as a byte and ONLY 7 bits of that byte used to represent the character that means ascii can only represent 128 unique char (`2*2*2*2*2*2*2`) 7 positons, each has 2 option (0, 1)

### Unicode 
Unicode solves the Ascii problem (only 128 unique char)

It backwards compatiable with ascii and that's because the first 128 symbols of unique are ascii chars 

### UTF-8
UTF-8 is variable-width char encoding.
Notice the WIDTH (ascii only using 1 byte) BUT UTF-8 can use 1, 2, 4, 8 bytes to represent chars 

## Asterisk 
`*` to modify the value
```Rust
fn main() {
  let mut v = vec![1, 2, 3, 4, 5];
  for i in &mut v {
    *i += 50; //
  }
  for i in &v {
    println!("{}", i);
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
