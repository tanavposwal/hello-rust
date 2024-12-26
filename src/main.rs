// guessing game
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    // random number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // can remove this
    println!("The secret number is: ***");

    loop {
        println!("Please input your guess.");
        
        // mutable variable
        let mut guess = String::new();

        // inputs
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // will ignore non number input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // compare
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! the secret number is {secret_number}");
                break;
            }
        }
    }
}
