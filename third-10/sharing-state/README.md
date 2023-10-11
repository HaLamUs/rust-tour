# Concurrency in Rust - Message Passing

[⬅ Back](../../README.md)

## Intro 
Video 

<div>
  <a href="https://www.youtube.com/watch?v=mupwF9jbVZ4"><img src="https://img.youtube.com/vi/mupwF9jbVZ4/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>


## Sharing state 
Transferring data using shared state. You can think it as one-way data flow, one thread passed a message to another thread and then that receiving thread now owns the data ???

Shared state, we have some piece of data in memory the multiple threads can read and write 

## Mutex 
Mutex aka mutual exclusion which means you have some piece of data and only ONE thread could access that piece of data 

To do this, mutex uses a locking system when a thread wants to access to a piece of data behind a mutex it will signal that it wants to access that data and acquire the mutex's lock. 

The lock is a data structure that keeps track of which thread has exclusive access to a piece of data once a thread has acquired a lock on a particular piece of data NO other thread can access that data, once the thread is done with that data it can unlock the data 

Mutex is hard to manage, 

2 rules: 
1. acquire a lock the data before you have access to data  
2. have to release that lock when you are done 

## Mutex example 

`release` will be handled by Rust automatically









## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>