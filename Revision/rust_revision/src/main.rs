use rand::Rng; //Rng trait defines methods that random number generators implement
use std::cmp::Ordering;
use std::io;

fn main() {
    println!(
        "\x1b[31mThis is an interactive guessing game where youll guess some numbers and the system random matches your guess to the actuall value.\x1b[0m"
    );
    let secret_number = rand::thread_rng().gen_range(1..=100); /*the rand::thread_rng
    function that gives us the particular random number generator we’re going to use: one that
    is local to the current thread of execution and is seeded by the operating system
    gen_range method takes a range expression(1 to 100=> takes the form
    start..=end i.e 1..=100) as an argument and generates a random
    number in the range.
     */
    loop {
        //create a loop instead of continously asking a user to enter a number
        println!("Please input your guess.");
        let mut guess = String::new(); // guesses get stored here (guess variable) nb:the guesses here are in string type
        io::stdin() //read user input
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            // through shadowing convert the guess usinf the parse method  from string to int type of u32(postive no of 32 bit size)
            Ok(num) => num, /* If parse is able to successfully turn the string into a number, it will return an Ok value that
            contains the resultant number. */
            Err(_) => continue, /* If parse is not able to turn the string into a number, it will return an Err value that contains
                                 more information about the error. The Err value does not match the Ok(num) pattern in
                                the first match arm, but it does match the Err(_) pattern in the second arm. The
                                 underscore, _ , is a catchall value; in this example, we’re saying we want to match all Err
                                 values, no matter what information they have inside them.  */
        };
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
