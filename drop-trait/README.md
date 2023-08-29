# Smart Pointers in Rust - The Drop Trait

[⬅ Back](../README.md)

## Intro 
Video 

<div>
  <a href="https://www.youtube.com/watch?v=RPWZcTYBS4k"><img src="https://img.youtube.com/vi/RPWZcTYBS4k/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>

## Drop trait 
The drop trait could be implemented on any type and it allows you to customize what happens when a value goes out of scope.

Example: 
- Box SP, we want deallocated memory on heap when out of scope 
- SP uses `lock` we want to release it when it out of scope 

## Drop method 

```Rust
c.drop(); // error 
drop(c); // supported by Rust 
```

## Summary
the drop trait combined with Rust ownership meaning we dont need to take the cleaning up memory. Rust will do that 




<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>