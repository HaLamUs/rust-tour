# Trait

[⬅ Back](../README.md)

## Intro 
Video

<div>
  <a href="https://www.youtube.com/watch?v=T0Xfltu4h3A"><img src="https://img.youtube.com/vi/T0Xfltu4h3A/0.jpg" alt="IMAGE ALT TEXT"></a>
</div>


# Trait 
In case, you want to share a set of methods that are avaiable different types 
 

 ```Rust
 pub struct NewArticle {
  pub author: String,
  pub headline: String,
  pub content: String
}

impl Summary for NewArticle {
  fn summarize(&self) -> String {
    format!("{}, by {}", self.headline, self.author)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
      format!("{}, by {}", self.content, self.username)
    }
}

pub trait Summary {
  fn summarize(&self) -> String;
}

 ```

 # Default implementation 

 ```Rust
 pub trait Summary {
  fn summarize(&self) -> String {
    String::from("Read more ...")
  }
}

 ```

# Trait and generic 

```Rust
  pub fn notify2(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
  }

  pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
  }

  pub fn notify(item1: &(impl Summary + Display), item2: &impl Summary) {
    ...
  }

  pub fn notify<T: Summary + Display>(item1: &T, item2: &T) { ... }

  fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32

  fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug 

```

# Examples 

```Rust
fn returns_summarizable(switch: bool) -> impl Summary {
  if switch {
    NewArticle {
      headline: String::from("headline"),
      author: String::from("author"),
      content: String::from("content"),
    }
  } else {
    Tweet {
      username: String::from("username"),
      content: String::from("content"),
      reply: false,
      retweet: false,
    }
  }
}


struct Pair<T> {
  x: T, 
  y: T
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self {x, y}
  }
}

imple<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largest member is x = {}", self.x);
    } else {
      println!("The largest member is y = {}", self.y);
    }
  }
}

```


<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>
