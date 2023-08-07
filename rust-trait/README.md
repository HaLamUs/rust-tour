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



<p><img type="separator" height=8px width="100%" src="https://github.com/HaLamUs/nft-drop/blob/main/assets/aqua.png"></p>

## Author

This repo was developed by [@lamha](https://github.com/HaLamUs). 
Follow or connect with me on [my LinkedIn](https://www.linkedin.com/in/lamhacs). 

## License
The source code for the site is licensed under the MIT license, [MIT](https://opensource.org/license/mit/)

 <a href="#top">Back to top</a>
