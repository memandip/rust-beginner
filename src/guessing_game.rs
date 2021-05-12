use rand;
use std::io;

fn main() {
  println!("Guess the number!");

  println!("Please input your guess:");

  let secret_number = rand::Rng::gen_range(1..101);

  // variables in rust are immutable by default
  let mut guess = String::new(); // mutable

  io::stdin().read_line(&mut guess).expect("Failed to read line.");

  println!("You guessed: {}", guess);
}
