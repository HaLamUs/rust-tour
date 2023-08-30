# Smart Pointers in Rust - Reference Counting

[⬅ Back](../README.md)

## Intro 
Video 

<div>
  <a href="https://www.youtube.com/watch?v=M9Owp3iLigg"><img src="https://img.youtube.com/vi/M9Owp3iLigg/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>

## Reference counting 
RC allows us to share ownership of some data 

### Problem 
Most of the cases, ownership is clear, we know extractly which variable owns a specific value. But we have cases where a single value has multiple owners for example 

Graph with multiple edges that point to the same node conceptly that node is owned by all the edges 

## RC smart pointer 
RC SP which will keep track the number of references to a value and when there are no more references the value will get cleaned up 

An analogy: When you watching TV with some friends, you cant turn it off without having ppl watching it

Usecase when we want to allocate a value on the heap and have multip;le parts of our program read that value 
🔴 This only working in single thread programs 



<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>