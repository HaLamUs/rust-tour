# Smart Pointers in Rust - Interior Mutability

[⬅ Back](../README.md)

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



<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>