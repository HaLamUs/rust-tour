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
