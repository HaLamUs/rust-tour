# Concurrency in Rust - Message Passing

[⬅ Back](../README.md)

## Intro 
Video 

<div>
  <a href="https://www.youtube.com/watch?v=UGDa0P2PXrY"><img src="https://img.youtube.com/vi/UGDa0P2PXrY/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>

## Object oriented
3 characteristics: 
- objects 
- encapsulation
- inheritance 

1. Objects 
Objects are made out of data and methos that operate on that data but not struct or enum 

2. Encapsulation 
which means that implementation details of an object are hidden from the code USING that object 

Instead of changing the internals directly code OUTSIDE of the object is limited to interacting with the object through its public api 

## Pub keyword 
In rust everything is private by default 

## Inheritance 
is the ability for an object to inherit from another object's definition gaining the data and behavior of that other object without having to define the data and behavior itself
NO IN RUST 

There are 2 main reasons for using inheritance
1. code sharing 
2. polymorphism
  allows you to substitute multiple objects for each other at runtime if they share certain characteristics in classical inheritance 

  Example: 
    Vehicle
      Truck, motorcycle, bike  
    
    ```
    fn takeObject(obj: Vehicle) {}

    fn main() {
      let truck = Truck()
      takeObject(truck)
      let car = Car()
      takeObject(car)
    }
    ```
    
Rust uses generics to abstract away concrete types and use straight bounds to restrict the characteristics of those types 




## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>