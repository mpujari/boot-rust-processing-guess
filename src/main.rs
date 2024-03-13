use rand::Rng;
use std::{cmp::Ordering, io};

// Learning Rust from https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    println!("The secrete number is: {secret_number}");

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Equal => println!("You win!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Less => println!("Too small!")
    }
}
