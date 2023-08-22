# Rust Crate

[⬅ Back](../README.md)

## Intro 
Video 

<div>
  <a href="https://www.youtube.com/watch?v=4TI153PIEDQ"><img src="https://img.youtube.com/vi/4TI153PIEDQ/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>

## Crate 
A crate is a compilation unit in Rust. Whenever rustc some_file.rs is called, some_file.rs is treated as the crate file

Upload your own code to crates.io 

## Profile 

```bash
// dev 
cargo build
```

```bash
// release
cargo build --release
```

### Cargo.toml

```bash
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

The trade-off 

For dev, We will have a fast compile time a slower run time 

Release mode, slower compile time for a faster runtime 




<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>