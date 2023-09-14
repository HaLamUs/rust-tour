# Smart Pointers in Rust - Reference Cycles

[⬅ Back](../README.md)

## Intro 
Video 

<div>
  <a href="https://www.youtube.com/watch?v=pIVZRDFAUyc"><img src="https://img.youtube.com/vi/pIVZRDFAUyc/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>


## Memory leak 
Rust guarantee that you CANT have data races but memory leaks.

You can create a memory that is never cleaned up by rc smart pointer, reference counting sp, refcell sp.

Using these sp create references where item reference each other each other in a cycle which will create a memory leak 






<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>