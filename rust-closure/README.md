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
      simulated_expensive_calculation(intensity) // 1
    );
    println!(
      "Next, do {} situps!",
      simulated_expensive_calculation(intensity) // 2
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

Normal ppl fix (1, 2) will create a var too keep track the closure's return value 

## Fix 
Memorialization pattern 
By using struct that hold closure and the result of closure 

`Fn(u32) -> u32` this called trait.

```Rust
struct Cacher<T> where T: Fn(u32) -> u32, {
  calculation: T, 
  value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32, {
  fn new(calculation: T) -> Cacher<T> {
    Cacher {
      calculating,
      value: None,
    }
  }

  fn value(&mut self, arg: u32) -> u32 {
    match self.value {
      Some(v) => v,
      None => {
        let v = (self.calculation)(arg);
        self.value = Some(v);
        v
      }
    }
  }
}

fn generate_workout(intensity: u32, random_number: u32) {
  let mut cached_result = Cacher::new (|num| {
    println!("calculating slowly ...");
    thread::sleep(Duration::from_secs(2));
    num
  });

  if intensity < 25 {
    println!(
      "Today, do {} pushups!",
      cached_result.value(intensity)
    );
    println!(
      "Next, do {} situps!",
      cached_result.value(intensity)
    );
  } else {
    if random_number == 3 {
      println!("Take a break today! Remember to stay hydrated!")
    } else {
      println!(
        "Today, run for {} minutes!",
        cached_result.value(intensity)
      );
    }
  }
}

```


## Example closure

```Rust
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5); // <-- error's here: missmatched type

```

Because, first type (string) passed intro the closure will be concrete type of input parameter's closure 





<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 


## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>