# Concurrency in Rust - Creating Threads

[⬅ Back](../../README.md)

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

## Thread 
2 kinds:

- one-to-one threads aka os threads, native threads, system threads etc 
- green threads aka user threads, program thread  etc 

Many operating system provide an api to create new threads one-to-one threads, which mean when you create a thread in your program it maps to an operating system thread ---> one-to-one mapping 

Many programming languages provide their own special implementation of threads which we call green thread. There is no 1-to-1 mapping, 20 green threads that map to only 5 OS threads --> m-to-n 

Each model has its advantages and trade-offs 

The most important trade-off is runtime support, runtime which mean the code that's included by the programming language in every single binary 

If you use green-thread your binary size will increase 


## Example 
By default every program has one main thread so in this case we are letting the main thread sleep for 

```Rust
use std::{thread, time::Duration};

fn main() {
  thread::spawn(||{
    for i in 1..10 {
      println!("hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }
}

```

Notice: the spawn thread didnt finish printing all of its, but the main did 
So the main thread ends the spawn thread is stopped no matter

So we will wait 

`handle.join()` will block the current thread which is main thread for now 
until the spawn thread terminates blocking a thread meaning prevented from doing any further 

```Rust
use std::{thread, time::Duration};

fn main() {
  let handle = thread::spawn(||{
    for i in 1..10 {
      println!("hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  handle.join().unwrap(); // 2

  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }

  handle.join().unwrap(); // 1
}

```

Depend ons (1) or (2) the output will vary 

## Capture var 

Capture variable in thread's closure 
We cant use reference because we dont know how long spawn thread will run 
So we have to move the ownership to the closure 

After `move` we cant use `v` in main thread 

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>