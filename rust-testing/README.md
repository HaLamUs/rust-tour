# Testing in Rust

[⬅ Back](../README.md)

## Intro 
Video

<div>
  <a href="https://www.youtube.com/watch?v=18-7NoNPO30"><img src="https://img.youtube.com/vi/18-7NoNPO30/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>

# Testing 
Rust have bring the value of type system such as borrow checker, make sure we are passing the right type and not mismanaging our memory 

But I cant check the business logic, the intent of our code  

```Bash
cargo new adder --lib
cargo  test

```

```Rust
    #[test]
    fn failing_test() {
      panic!("Make this test fail");
    }

  assert_eq
  asser_ne

  assert!(
    result.contains("Carol"),
    "Greeting did not contain name, value was {}", result
  )

  #[test]
  #[should_panic]
  fn greater_than_100() {
    Guess::new(200);
  }

```

`#[should_panic]` got Green 

`#[should_panic(expected = "Guess value must be less than or equal to 100")]`

we expect substring.

# Thread 
By default, each test gets its own thread 

If we want to run test serially we can set thread test = 1

``` Bash
cargo test -- --test-threads=1
```

This will run test binary with extra dash dash 



<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License