use std::cmp::Ordering;
use std::io;

use rand::prelude::*;

fn main() {
    const MIN: i32 = 1;
    const MAX: i32 = 10;

    println!("Guess the number between {MIN} and {MAX}...");
    println!("Please input your guess.");

    let secret_number = rand::rng().random_range(MIN..=MAX);
    // println!("secret_number: {secret_number}");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");

        println!("You guessed {guess}!!!");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too low!"),
            Ordering::Greater => println!("too high!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}

