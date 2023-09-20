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





## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>