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


```Rust

use std::sync::Mutex;

fn main() {
  let m = Mutex::new(5);

  {
    let mut num = m.lock().unwrap();
    *num = 6;
  }
  println!("m = {:?}", m);
}

```

`move` use to move `counter` variable into the thread 


```Rust
  let counter = Mutex::new(0)
  for _ in 0..10 {
    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap();
      *num += 1;
    });

    handles.push(handle);
  }
```

Error at `move`, because it's already moved in previous iteration 

To allow `counter` to have multiple owners, use Reference counting 


```Rust
  let counter = Rc::new(Mutex::new(0));
  let mut handles = vec![]!

  for _ in 0..10 {
    let counter = Rc::clone(&counter);
    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap();
      *num += 1;
    });

    handles.push(handle);
  }
```

Here we have an other issue, this reference counting is not a thread safe 

```Rust
fn main() {
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap();
      *num += 1;
    });

    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("Result: {}", *counter.lock().unwrap());
}

```

We use atomic reference counting 

Despite `counter` is immutable but we are able to get a mutable reference to the value that it holds, and that's because mutex uses interios mutability in the same way that the ref cell smart pointer allows you to mutate a value that's inside an rc smart pointer 




## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>