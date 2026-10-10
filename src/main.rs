use std::io;

use rand::prelude::*;

fn main() {
    const MIN: i32 = 1;
    const MAX: i32 = 10;

    println!("Guess the number between {MIN} and {MAX}...");
    println!("Please input your guess.");

    let secret_number = rand::rng().random_range(MIN..=MAX);

    let mut done = 0;

    // println!("secret_number: {secret_number}");

    let mut guess = String::new();
    let mut users_guess: Result<i32, _>;

    while done == 0 {
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");

        println!("You guessed {guess}!!!");

        users_guess = guess.trim().parse();

        match users_guess {
            Ok(num) => {
                if num < MIN || num > MAX {
                    println!("Make sure to choose a number between {MIN} and {MAX}!!!");
                }

                if num == secret_number {
                    done = 1;
                    println!("CORRECT! Goodbye for now dear.");
                } else {
                    println!("WRONG! Keep guessing.");
                    guess.clear();
                }
            },
            Err(e) => {
                println!("Failed: {}: ", e);
            }
        }
    }
}
