# Mini grep

[⬅ Back](../README.md)

## Intro 
Video 1

<div>
  <a href="https://www.youtube.com/watch?v=XYkiwsplDTg"><img src="https://img.youtube.com/vi/XYkiwsplDTg/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>

Video 2

<div>
  <a href="https://www.youtube.com/watch?v=-L4nKAlmH3M"><img src="https://img.youtube.com/vi/-L4nKAlmH3M/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>

## Program 
We implement mini `grep`

```Rust
let args: Vec<String> = env::args().collect();
```

This will convert list of passing args to collection of String.

We need to define type `Vec<String>` because collection requires a type

## Refactor 
- Ideally a fn should have one responsibility 
- query and filename need connect in some ways ⁉️
- Centrelize place to handle errors 

If the main function gets too large same the binary crate  

Fix: Split the lib, then the binary crate will call fns in lib 


### Struct can store a ref 
But we have to take care the lifetime 

```Rust
fn parse_config(args: &[String]) -> Config {
  let query = args[1].clone();
  let filename = args[2].clone();
  Config { query, filename }
}
```

Use `clone()` we don't want to take the ownership 

Problem: `parse_config` is very closely tied to `Config` type. We want decouple this.

Solve: We the public fn into `Config` struct


<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License