# Common Programming Concepts

[⬅ Back](../README.md)

## Intro 
Video

<div>
  <a href="https://www.youtube.com/watch?v=2V0JaMVjzws"><img src="https://img.youtube.com/vi/2V0JaMVjzws/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>

## Variables 
Vars are immutable by default 

``` rust 
// will throw error 
let x = 5;
println!("This is the value of {}", x);
x = 6; // <-- 🐞

```

## Constants 

``` rust 
const MY_BANK_NUMBER: u32 = 100_000;
```
const you have to define the type, can't change it

## Shadowing 

```rust
let x = 5;
println!("This is the value of {}", x);
let x = 6.2;
print!("This is the value of {}", x);

```

## Scalar data types 
- Integers 
- Floating-point numbers 
- Booleans 
- Character

### Overflow 

```rust
let f: u8 = 256;
```

## Compound data types
- Tuples
- Array 

## Return 
Replace `return` by just var name 

```rust
sum
// return sum;
```


<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License