# Concurrency in Rust - Message Passing

[⬅ Back](../README.md)

## Intro 
Video 

<div>
  <a href="https://www.youtube.com/watch?v=FE1BkKqYCGU"><img src="https://img.youtube.com/vi/FE1BkKqYCGU/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>

## Message passing 
Using messages to pass data b/w threads. This ensures safe concurrency  

Go' slogan: Do not communicate by sharing memory instead share memory by communicating 

## Channel 
Using channel for passing messages 

Include 2 parts: transmitter and receiver 

Think about river analogy, the transmitter is the upstream location where you place the rubber duck and the receiver is the downstream location where the rubber duck will END up.

One of your code is calling the function transmitter and another part of your code is listening to the receiver 

## Example 

`use std::sync::mpsc;`

Multi-producer, single-consumer 

`let received = rx.recv().unwrap()`

the `recv` will block the main threads excution while it waits for a message or a value to be sent down the channel 

`let received = rx.try_recv().unwrap()`

the `try_recv` will not block the mrain thread's execution 

Usecase like server listening for upcomming package, every loop we `try_recv` then let the current thread does other work 

```Rust
use std::sync::mpsc;
use std::thread;

fn main() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let msg = String::from("hi");
    tx.send(msg).unwrap();
  });

  let received = rx.recv().unwrap();
  println!("Got: {}", received);
}

```


## Ownership rules 
Ownership rules and how they relate to concurrent code 

```Rust

fn main() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let msg = String::from("hi");
    tx.send(msg).unwrap();
    println!("msg is {}", msg); // ERROR 
  });

  let received = rx.recv().unwrap();
  println!("Got: {}", received);
}

```

We get error because we would send `msg` to another thread and then afterwards we could potentially modify or drop the variable 

`tx.send(msg).unwrap();` will the ownership of `msg`

```Rust
fn main() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];
    
    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  for received in rx {
    println!("Got: {}", received);
  }
}
```

## Multiple producer 

Multiple producer to send messages 




## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>