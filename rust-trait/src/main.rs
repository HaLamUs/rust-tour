
pub struct NewArticle {
  pub author: String,
  pub headline: String,
  pub content: String
}

impl Summary for NewArticle {

  fn summarize_author(&self) -> String {
      format!("author {}", self.author)
  }

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
  fn summarize_author(&self) -> String {
    format!("username {}", self.username)
  }

//     fn summarize(&self) -> String {
//       format!("{}, by {}", self.content, self.username)
//     }
}

pub trait Summary {
  fn summarize_author(&self) -> String;


  fn summarize(&self) -> String {
    format!("Read more from {} ", self.summarize_author())
  }
}

pub fn notify2(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}

pub fn notify<T: Summary>(item: &T) {
  println!("Breaking news! {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
      username: String::from("@abc"),
      content: String::from("Bonjour State"),
      reply: false,
      retweet: false,
    };

    let  article = NewArticle {
      author: String::from("Uncle Sam"),
      headline: String::from("Openheimer"),
      content: String::from("Delayed by 1 month to a world")
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());

    notify(&article);
}
