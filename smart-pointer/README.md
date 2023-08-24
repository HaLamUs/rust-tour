# The Box Smart Pointer in Rust

[⬅ Back](../README.md)

## Intro 
Video 

<div>
  <a href="https://www.youtube.com/watch?v=m76sRj2VgGo"><img src="https://img.youtube.com/vi/m76sRj2VgGo/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>

## Pointer 
Pointer is a variable that stores a memory address and that memory address refers to or points to some other data in memory 

Reference is the most common pointer in Rust.

Reference simply borrow the values they point to which means they don't have ownership over the value 

## Smart pointer 
SP has more capabilities, SPs are data structures that act like a pointer but have metadata and extra capabilities 

They have reference counting which keeps tracking the number of owners and cleaning up when there is no owners. Sometime it owns the data NOT just borrow. 

Example: String, Vector 

They own data and allow you to manipulate it. They store extra metadata such as capacity and extra capabilities for example `String` ensures that the data is valid utf-8 

In detail, SP is implemented by struct and traits 





<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>