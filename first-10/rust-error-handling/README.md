
# Rust Error handling

[⬅ Back](../../README.md)

## Intro 
Video

<div>
  <a href="https://www.youtube.com/watch?v=wM6o70NAWUI"><img src="https://img.youtube.com/vi/wM6o70NAWUI/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>


## Panic

You want crash the program.

```Rust
panic!("Crash and burn")
```

```Bash
RUST_BACKTRACE=1 cargo run
```

## Result enum 
You dont want to crash ur program.


```Rust
use std::fs::File;
use std::io::{Error, ErrorKind};
fn main() {
  let f = File::open("hello.txt");
  let f = File::open("hello.txt").unwrap();
  let f = File::open("hello.txt").expect("Failed");

  let f = match f {
      Ok(file) => file,
      Err(error) => match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
          Ok(fc) => fc,
          Err(e) => panic!("Problem creating the file: {:?}", e),
        },
        other_error  => {
          panic!("Problem creating the file: {:?}", other_error);
        }        
      }
  };
}
```

## Error propagation 
We want child func return ther error back to the caller 

`?` use as optional will auto return Error 

```Rust
fn read_username_from_file2() -> Result<String, Error> {
  let mut f = File::create("hellp.txt")?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}
```

## Best practice 

Use Result enum and error propagation

Use `unWrap` when 100% sure 

## Extend 🆒

We can create a new type just for validate the input value --> Cool 


<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>
