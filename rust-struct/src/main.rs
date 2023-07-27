struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

fn main() {
  let mut user1 = User {
    email: String::from("lam"),
    username: String::from("nitendo"),
    active: true,
    sign_in_count: 1
  };
  let name = user1.username;
  user1.username = String::from("ha");

  let user2 = build_user(
    String::from("keoly@gmail.com"), 
  String::from("sony")
  );
  
  let user3 = User { 
    email: String::from("chanhxa@gmail.com"), 
    username: String::from("fox"),
    ..user2
  };
}


fn build_user(email: String, username: String) -> User {
  User { username, email, sign_in_count: 1, active: true }
}