use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the number guessing game!");
    let mut secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter your guess.");

        // Create a mutable variable to store the user's guess
        let mut guess = String::new();

        // Read the user's input and store it in the guess variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();

        if guess == "quit" {
            println!("Quitting game");
            break;
        }
        
        if guess == "restart" {
            println!("The secret number was {secret_number}");
            println!("Restarting game...");
            secret_number = rand::thread_rng().gen_range(1..=100);
            continue;
        }

        // Convert the user's input to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please only enter a number!");
                continue;
            },
        };

        // Check if value is less than or greater than 100
        // Question: What is the benefit of using a match statement here?
        // Or is it just a preference?
        // Answer: If statements require a Boolean condition, whereas match
        // statements can be used to match against any type and enum.
        if guess > 100 || guess < 1 {
            println!("Please enter a number >= 1 and <= 100");
            continue;
        }

        // Wrap variables in brackets to print them
        println!("You guessed: {guess}");

        // Compare the user's guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        };
    }
}
