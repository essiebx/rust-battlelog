use rand::Rng;
use std::io;

fn main() {
    println!("🎮 Welcome to the Guessing Game!");

    println!("What is your best name?");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");
    let name = name.trim(); // shadow to make it a clean &str

    // Generate the secret number
    let secret_number = rand::thread_rng().gen_range(1..=100); // 1 to 100 inclusive
    println!("(Your secret number is: {})", secret_number); // Just for debug

    println!("{name}, take a good guess (number between 1 and 10):");

    let mut guess_string = String::new();
    io::stdin()
        .read_line(&mut guess_string)
        .expect("Failed to read guess");

    let guess: i32 = guess_string
        .trim()
        .parse()
        .expect("Please enter a valid number");

    println!("Hey {name}, your guess is: {guess}");
}
