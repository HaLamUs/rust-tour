# Smart Pointers in Rust - Interior Mutability

[⬅ Back](../../README.md)

## Intro 
Video 

<div>
  <a href="https://www.youtube.com/watch?v=77aRH6YBKyY"><img src="https://img.youtube.com/vi/77aRH6YBKyY/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>


## Interior Mutability 
IM is design pattern in Rust that allows you to mutate data even when there are immutable references to that data which is typically disallowed by the borrowing rules to mutate data.

This pattern uses unsafe code inside a data structure to bypass the typical rules around mutation and borrowing 

Unsafe code is code that is not checked at compile time for memory safety 

Let think that we have a data structure that stores some value and inside that data structure the value is mutable but when we get a reference to that data structure the reference itself is immutable code outside of the data structure would not be able to mutate the data stored within the data structure directly but you can imagine we could call some methods that would mutate the inner value --> IM 

Same idea for RefCell sp but the method will return mutable or immutable reference to the data. RefCell check the reference are valid at runtime 


## RefCell smart pointer 
RC sp represents single ownership over the data it holds much like the box smart pointer.

The difference is the box sp enforces the borrowing rules at compile time whereas the ref cell sp enforces borrowing rules at RUNTIME this means if you break the borrowing rules at runtime your program will panic 

RC sp runs in single thread 

## Halting problem 
In computability theory, the halting problem is the problem of determining, from a description of an arbitrary computer program and an input, whether the program will finish running, or continue to run forever.

UNDECIDABLE, meaning that no general algorithm exists that solves the halting problem for all possible program–input pairs.

THAT is why we have unsafe code 


## Smart pointers 

- Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
- Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at complie time; RefCell<T> allows immutable or mutable borrows checked at runtime.
- Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.


Mutating a value inside an immutable value is called the interior mutability pattern (change inside)


## Example 

`*d = 20`, `*` is reference operator 

```Rust
fn main() {
  let a = 5;
  let b = &mut a; // error 

  let mut c = 10;
  let d = &c;
  *d = 20; // error 
}

```

`'a` lifetime annotation 

```Rust

pub trait Messenger {
  fn send(&self, msg: &str);
}

mod tests {
  use super::*;
  use std::cell::RefCell;


  struct MockMessenger {
    sent_messages: Vec<String>,
  }

  impl MockMessenger {
    fn new() -> MockMessenger {
      MockMessenger {
        sent_messages: vec![],
      }
    }
  }

impl Messenger for MockMessenger {
    fn send(&send, message: &str) {
      self.sent_messages.push(String::from(message)); // error becoz &send is immutable reference 
    }
  }

// Fix 



impl Messenger for MockMessenger {
    fn send(&mut send, message: &str) {
      self.sent_messages.push(String::from(message)); // still error cause this is NOT trait Messenger
    }
  }

```
Still error, so we find ourselves in a predicament we need a mutable so we can modify the send messages field on our mock messenger struct HOWEVER we cant get immutable reference in this function because that would break the function signature of send which is defined in our messenger trait 

Use Interior Mutability pattern 

Fix wrap the vec in RefCell 

```Rust
struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
  }
```

Then borrow the mutable reference to use 

```Rust 
self.sent_messages.borrow_mut().push(String::from(message));
```

This is immutable reference 

```
assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
```

## Final 
```Rust
pub trait Messenger {
  fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
  messenger: &'a T,
  value: usize,
  max: usize,
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger {
  pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
    LimitTracker {
      messenger,
      value: 0,
      max
    }
  } 

  pub fn set_value(&mut self, value: usize) {
    self.value = value;

    let percentage_of_max = self.value as f64 / self.max as f64;

    if percentage_of_max >= 1.0 {
      self.messenger.send("Error: You are over your quota!");
    } else if percentage_of_max >= 0.9 {
      self.messenger.send("Urgent warning: You 've used up over 90% of your quota!");
    } else if percentage_of_max >= 0.75 {
      self.messenger.send("Warning: You 've used up over 75% of your quota!");
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::cell::RefCell;

  struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
  }

  impl MockMessenger {
    fn new() -> MockMessenger {
      MockMessenger {
        sent_messages: RefCell::new(vec![]),
      }
    }
  }

  impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
      let mut one_borrow = self.sent_messages.borrow_mut();
      let mut two_borrow = self.sent_messages.borrow_mut();

      one_borrow.push(String::from(message));
      two_borrow.push(String::from(message));
    }
  }

  #[test]
  fn it_sends_an_over_75_percent_warning_message() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);
    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
  }

}
```

## Notice 
RefCell sp checks the borrowing rules at runtime and one of the borrowing rules is that we cant have 2 mutable references at the same time 

Example 

```Rust
impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
      let mut one_borrow = self.sent_messages.borrow_mut();
      let mut two_borrow = self.sent_messages.borrow_mut();

      one_borrow.push(String::from(message));
      two_borrow.push(String::from(message));
    }
  }
```

If run `cargo test`, test will failed 

## Multiple owners of mutable data 
Combining the reference counting sp with refCell sp to get multiple owners of mutable data 

`enum List {` we use rc because we wanted to have 2 list that both share a third list 
RC only store immutable values in this version we want change the value After created 

`Cons(Rc<RefCell<i32>>, Rc<List>),` wraps in RC because we want them to have multiple owners 
Our i32 wrap in RefCell because we want it mutable 


```Rust
#[derive(Debug)]

enum List {
  Cons(Rc<RefCell<i32>>, Rc<List>),
  Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::RC;

fn main() {
  let value = Rc::new(RefCell::new(5));

  let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

  let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
  let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

  *value.borrow_mut() += 10;
  println!("a after = {:?}", a)
  println!("b after = {:?}", b)
  println!("c after = {:?}", c)
}
```


`let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));` uses clone because we want `value` variable and `a` list both own the value 


<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>