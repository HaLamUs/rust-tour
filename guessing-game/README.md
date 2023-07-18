# Guessing Game

[⬅ Back](../README.md)

## Intro 
Video

<div>
  <a href="https://www.youtube.com/watch?v=H0xBSbnQYds"><img src="https://img.youtube.com/vi/H0xBSbnQYds/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>

### Immutable 

rust var immutable by default 

```rust
let guess = String::new();
```

```rust
let mut guess = String::new();
```

`&mut` mean get reference modiy but without taking the ownership

```rust
io::stdin().read_line(&mut guess);
```

This is just read the reference.
```rust
guess.cmp(&secretNumber);
```

### Add dependency

in `Cargo.toml`
```rust 
[dependencies]
rand = "0.5.5"
```

then `cargo build`

### Naming 
You can define the same name in the same scope. 

What's crazzy is that! 🤪
It's called shadowing.

Shadowing meaning convert one type to another type but we keep the name

### match 
  just like if,else, switch case 


### Add .zshrc 

https://stackoverflow.com/a/50711056

```bash
# Add Rust to $PATH
export PATH="$HOME/.cargo/bin:$PATH"
```

<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License