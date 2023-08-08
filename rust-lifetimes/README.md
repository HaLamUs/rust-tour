# Rust Lifetimes

[⬅ Back](../README.md)

## Intro 
Video

<div>
  <a href="https://www.youtube.com/watch?v=juIINGuZyBc"><img src="https://img.youtube.com/vi/juIINGuZyBc/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>

# Dangling reference
It's a reference that points to a invalid data
Rust uses borrow checker to check dangling ref 

```Rust
fn main() {
  let r;
  {
    let x = 6;
    r = &x;
  } // x ends here so r is invalid pointer - dangling 

  println!("r: {}", r);
}



fn main() {
  let x = 6;
  let r = &x;
  println!("r: {}", r);
}

```

# Generic lifetime annotation 
GLA describes the relationship b/w the lifetimes of multiple references and how they relate to each other 

## Problem 

```Rust

fn main() {
  let string1 = String::from("abcd");
  let string2 = String::from("xyz");

  let result = longest(string1.as_str(), string2.as_str());
  println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}


```

`string2.as_str()` is string slice 

`-> &str {` it's ambiguous, because x or y have different lifetimes and the fn can be called in many places so many lifetimes 
To solve we need to know correct lifetime that is returned --> Generic lifetime 

## Fix 

```Rust
fn main() {
  let string1 = String::from("abcd");
  let string2 = String::from("xyz");

  let result = longest(string1.as_str(), string2.as_str());
  println!("The longest string is {}", result); // this case str1's lifetime = str2 
  // borrow checker will check smallest life time b/w x and y if it's still valid at here --> valid
}

// &i32           // a reference 
// & 'a i32       // a reference with an explicit lifetimes 
// & `a mut i32   // a mutable reference with an explicit lifetimes 

fn longest<'a>(x: &'a str, y : &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

```

## Exmaples

```Rust
fn main() {
  let string1 = String::from("abcd");
  {
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result); // string2's lifetime is smallest but the result still valid at this point 
  }
}

fn main() {
  let string1 = String::from("abcd");
  let result;
  {
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str()); // error here 
  }
  println!("The longest string is {}", result); 
}


```

`<'a>` just a syntax 

`fn longest<'a>(x: &'a str, y : &'a str) -> &'a str {`

The relationshop b/w x, y and returned ref is this the lifetime of the returned ref will be the same as smallest lifetime of arguments 

So if `x` has a smaller lifetime than `y` then the lifetime of returned ref will be the same as x and otherwise 


## Note 
``` Rust
// No error 
fn main() {
  let string1 = String::from("abcd");
  let result;
  {
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str()); // error here 
  }
  println!("The longest string is {}", result); 
}

fn longest<'a>(x: &'a str, y : & str) -> &'a str {
  x
}

```

The lifetime of returned value always has to be tied the lifetime of one of our params. Why? Because we return a ref so it has to be a ref to someting that is passed in 

We CANT return a ref to st created inside the fn 

```Rust 
// err 
fn longest<'a>(x: & str, y : & str) -> &'a str {
  let result = String::from("really long string");
  result.as_str()
}

// ok 
fn longest<'a>(x: & str, y : & str) -> String {
  let result = String::from("really long string");
  result // this is OK cause we are passimg the ownership 
}


```


<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License