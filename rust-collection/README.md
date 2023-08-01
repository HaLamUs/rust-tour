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

Borrow and change value 

```Rust
et mut v = vec![1, 2, 3, 4, 5];

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


<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>
