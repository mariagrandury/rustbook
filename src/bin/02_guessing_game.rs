//! This is our first guessing game program!
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    //! This is the main function
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=1_000);

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();
        let guess = match guess.parse::<i32>() {
            Ok(value) => value,
            Err(_) => {
                println!("Come on! \"{} is not a number!", &guess);
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is too low, try again :)"),
            Ordering::Greater => println!("Your guess is too high, try again :)"),
            Ordering::Equal => {
                println!("You won!!!");
                break;
            }
        }
    }
}
