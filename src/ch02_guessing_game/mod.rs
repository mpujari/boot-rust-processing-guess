// Learning Rust from https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
pub mod ch02_guessing_game_mod {

    use rand::Rng;
    use std::{cmp::Ordering, io};

    pub fn guess_game() {
        println!("Guess the number!");
        let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
        // println!("The secrete number is: {secret_number}");
        loop {
            println!("Please input your guess");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line.");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("You guessed: {guess}");

            match guess.cmp(&secret_number) {
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
                Ordering::Greater => println!("Too big!"),
                Ordering::Less => println!("Too small!"),
            }
        }
    }
}
