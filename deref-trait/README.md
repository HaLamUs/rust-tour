# Smart Pointers in Rust - The Deref Trait

[⬅ Back](../README.md)

## Intro 
Video 

<div>
  <a href="https://www.youtube.com/watch?v=dYEC6NElVOg"><img src="https://img.youtube.com/vi/dYEC6NElVOg/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>

## Deref trait 
Deref trait allows you to customize the behavior dereference operator `*`


```Rust
  let y = &x; // 1
  let y = Box::new(x); // 2

```

`1` y is reference to x, is a memory address, or a poitner points to the location where `5` is stored

🔴 `2` y is pointing to a copy of `5` because primitives type (integer) get passed to a function the value is copied instead of ownership being transfer 


### Explain 

Box is SP that implement deref trait which allows dereference operator to work the same as if it were a reference. 😲






<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>