# Concurrency in Rust - Creating Threads

[⬅ Back](../README.md)

## Intro 
Video 

<div>
  <a href="https://www.youtube.com/watch?v=06WcsNPUNC8"><img src="https://img.youtube.com/vi/06WcsNPUNC8/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>


## Concurrent
Concurrent programming is when different parts of your program execute independently 

Parallel programming is when different parts of your program execute at the same time  

Rust allows you fearless concurrency 

## Operating system 

In most current operating systems an executed program's code is ran within a PROCESS 
and the operating system manages running multiple processes at once 

Within a program you can have independent parts that run simultaneously and the features that run these independent parts are called THREAD

Splitting your program's computation into multiple threads can improve performance because 
multiple parts of ur program are running at the same time but this also increase complexity
because threads run simultaneously you dont hvae control over the order in which differen parts of your program are excuted which leads some unique challenges 

Such as: 

- race conditions where threads are accessing data or resources in an inconsistent order 

- because execution order is non-deterministic bugs can appear that only happen in certain situations and are hard to reproduce and fix reliably 

- deadlocks where we have 2 threads that are both waiting for a resource that the other thread has making both threads wait

<img src="./imgs/deadlock.png">



## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>