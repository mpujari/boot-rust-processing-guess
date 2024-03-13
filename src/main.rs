use rand::Rng;
use std::io;

// Learning Rust from https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    println!("The secrete number is: {secret_number}");

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed: {guess}");
}
