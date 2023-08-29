# Smart Pointers in Rust - The Deref Trait

[⬅ Back](../README.md)

## Intro 
Video 

<div>
  <a href="https://www.youtube.com/watch?v=dYEC6NElVOg"><img src="https://img.youtube.com/vi/dYEC6NElVOg/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>

## Deref trait 
Deref trait allows you to customize the behavior dereference operator `*`


```Rust
  let y = &x; // 1
  let y = Box::new(x); // 2

```

`1` y is reference to x, is a memory address, or a poitner points to the location where `5` is stored

🔴 `2` y is pointing to a copy of `5` because primitives type (integer) get passed to a function the value is copied instead of ownership being transfer 

```Rust
  let y = Box::new(x);
  assert_eq!(5, *y); // no error 
```

### Explain 

Box is SP that implement deref trait which allows dereference operator to work the same as if it were a reference. 😲

## Tuple struct 

```Rust
struct MyBox<T>(T);

```

Using Tuple Structs Without Named Fields to Create Different Types

https://doc.rust-lang.org/book/ch05-01-defining-structs.html

```Rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```


### Problem 


```Rust
  let y = MyBox::new(x);
  assert_eq!(5, *y); // ERROR: MyBox cant be dereferenced 
```

### Solved 

```Rust
impl<T> Deref for MyBox<T> {
  type Target = T;

  // fn deref(&sefl) -> &Self::Target {
  fn deref(&sefl) -> &T {
    &self.0 // first item in MyBox tuple 
  }
}
```

when dereference operator is used we want to return a reference to the item stored in our tuple struct 

### Question 

Why deref dont return the value directly because Rust's ownership system if dref did Rust would move the ownership of the value 


The actual code 

```Rust
assert_eq!(5, *(y.deref()));
```

## Implicit deref coercion 
// cast ngầm 

https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/deref-coercions.html

It's just a convenience feature that Rust that automatically for type that implement the `deref`. Deref coercion will convert a reference of one type to a reference of different type



```Rust

let m = MyBox::new(String::from("Hello Lam!"));
hello(&m); // m is MyBox type but has no problem when hello is need a ref to string slice

// &MyBox<String> --> &String --> &str

fn hello(name: &str) {
  println!("Hello, {}!", name);
}

```

`// &MyBox<String> --> &String --> &str` 
Call deref on `m` will return `&string`, call deref on string will return a string slice

WTHeck, Rust see the type being passed to `hello` is different than the type expected it will auto perform chained deref call at compile time to get the correct type 

Without deref cocercion 
`hello(&(*m)[..])`

First, dereference (*) m to a string then taking a reference to a string slice - the full range of the string 

## Borrowing rules 
Rust will perform the deref cocercion in these cases 

1. From immutable reference to another immutable ref
2. From mutable reference to another immutable ref
3. From mutable reference to another immutable ref 
CAN NOT convert from immutable to mutable because borrowing rule 

Borrow rule states that you can only have one mutable reference to a specific piece of data within a specific scope 


Convert a immutable ref to a mutable ref would require that the intial immutable ref is only immutable ref to that piece of data in that scope --> this breaking the rule 
BECAUSE We can have MANY immutable 

Rules:
1. one or more references (&T) to a resource - 1 - many 
2. exactly one mutable reference (&mut T).




<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>