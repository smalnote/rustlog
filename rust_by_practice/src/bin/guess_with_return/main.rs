use rand::Rng;
use std::cmp::Ordering;
use std::error::Error;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let secret_number = rand::rng().random_range(1..=100);
    println!("Guess the number!");
    loop {
        print!("Please input your guess:");
        io::stdout().flush()?;
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)?;
        let guess: u32 = guess.trim().parse()?;
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Try again."),
            Ordering::Greater => println!("Too big! Try again."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("The secret number is: {}", secret_number);
    Ok(())
}
