# Closures in Rust

[⬅ Back](../README.md)

## Intro 
Video 

<div>
  <a href="https://www.youtube.com/watch?v=kZXJvLfjUS4"><img src="https://img.youtube.com/vi/kZXJvLfjUS4/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>

## Closure 
Closure is anonymous function.

No name, store as variable and pass around, capture variable in their scope  

### Problem

```Rust
fn generate_workout(intensity: u32, random_number: u32) {
  if intensity < 25 {
    println!(
      "Today, do {} pushups!",
      simulated_expensive_calculation(intensity)
    );
    println!(
      "Next, do {} situps!",
      simulated_expensive_calculation(intensity)
    );
  } else {
    if random_number == 3 {
      println!("Take a break today! Remember to stay hydrated!")
    } else {
      println!(
        "Today, run for {} minutes!",
        simulated_expensive_calculation(intensity)
      );
    }
  }
```

Too much duplicate code 




<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 


## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>