use std::io; // io is not a part of the prelude.

fn main() {
   println!("Guess the number");

   println!("Please input your guess.");

   let mut guess = String::new();

   io::stdin()  // std::io::stdin() without using use std::io.
       .read_line(&mut guess)
       .expect("Failed to read line");
       
   println!("You guessed: {guess}");
}

// Rust variables are immutable by default.
// let apples = 5; // immutable
// let mut mangoes = 5; // mutable

