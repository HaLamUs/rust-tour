# Cargo Workspace

[⬅ Back](../README.md)

## Intro 
Video 

<div>
  <a href="https://www.youtube.com/watch?v=70_9IIsQfjs"><img src="https://img.youtube.com/vi/70_9IIsQfjs/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>

## Cargo workspace
Simple CLI program that had a binary crate and a library crate 
### Problem 
Project growing we have mul lib crates 

Cargo workspace helps you organize your project workspace.

Workspaces help you manage mul related packages that are developed in tandem.

Packages in a workspace share common dependency, share one output directory 

## Scenario
One binary that depends on two libs 

## Structure 
- `add-one` is a library that has `fun add_one`

- `adder` is a binary that uses `fun add_one`


1. Add workspace by define Cargo.toml 

2. Create binary adder 
  ``` bash
  cargo new adder
  cargo build 
  ```

3. Create lib add-one
  ```bash
  cargo new add-one --lib
  ```

4. Add dependency `adder` - Cargo.toml

```
[dependencies]
add-one = { path = "../add-one" }
```

build `adder` and run 

```bash
cargo build

cargo run -p adder // p: package

```



<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>