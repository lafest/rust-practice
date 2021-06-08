extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("guess the number");
  let secret_number = rand::thread_rng().gen_range(0..101);
  println!("secret number {}", secret_number);
  println!("input your guess");
  let mut guess = String::new();
  io::stdin().read_line(&mut guess).expect("failed to read line");
  let guess: u32 = guess.trim().parse().expect("type only number");

  println!("you guessed: {}", guess);

  match guess.cmp(&secret_number) {
    Ordering::Less => println!("Less"),
    Ordering::Greater => println!("Greater"),
    Ordering::Equal => println!("Equal"),
  }
}