use rand::Rng;
use std::cmp::Ordering;
use std::io; // io is not a part of the prelude.

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin() // std::io::stdin() without using use std::io.
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}

// Rust variables are immutable by default.
// let apples = 5; // immutable
// let mut mangoes = 5; // mutable
