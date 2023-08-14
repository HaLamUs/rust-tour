use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secretNumber = rand::thread_rng().gen_range(1, 191);
    println!("The secret number: {}", secretNumber);

    loop {
      println!("Please input your guess.");
      let mut guess = String::new();

      io::stdin().read_line(&mut guess).expect("Failed to readline!");
  
      let guess: u32 = match guess.trim().parse() {
          Ok(num) => num,
          Err(_) => continue,
      }; 
  
      println!("Your guess: {}", guess);
  
      match guess.cmp(&secretNumber) {
          Ordering::Less => println!("{}", "Too small!!".red()),
          Ordering::Greater => println!("{}","Too big!!".red()),
          Ordering::Equal => {
            println!("{}", "You win!".green());
            break;
          }
      }    
    }
}
