/*you can use logic conditions to control the flow of your program.
Rust has the following conditional statements:

Use if to specify a block of code to be executed, if a specified condition is true
Use else to specify a block of code to be executed, if the same condition is false
Use else if to specify a new condition to test, if the first condition is false
Use switch to specify many alternative blocks of code to be executed

*/

fn main() {
    if 7 > 5 {
        println!("7 is greater than 5.");
    } else if 7 < 5 {
        println!("7 is less than 5.");
    } else {
        println!("7 is equal to 5.");
    }

    // if...else
    // If the condition is not true, you can use else to run different code:
    // Example of using if-else statements
    let age = 16;

    if age >= 18 {
        println!("You can vote.");
    } else {
        println!("You are too young to vote.");
    }

    // else if// You can also use else if to specify a new condition to test, if the first condition is false:
    // Example of using else if

    let score = 85;

    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else if score >= 70 {
        println!("Grade: C");
    } else {
        println!("Grade: F");
    }

    let number = 10;

    if number < 0 {
        println!("The number is negative.");
    } else if number == 0 {
        println!("The number is zero.");
    } else {
        println!("The number is positive.");
    }

    let age = 20;

    if age < 18 {
        println!("You are a minor.");
    } else if age >= 18 && age < 65 {
        println!("You are an adult.");
    } else {
        println!("You are a senior citizen.");
    }
    /*
        Using if as an Expression
    In Rust, if...else can also be used as an expression.

    This means you can assign the result of an if to a variable
         */
    let time = 20;
    let greeting = if time < 18 {
        "Good day."
    } else {
        "Good evening."
    };
    println!("{}", greeting);
    // nb:When using if as an expression, you must include else . This ensures the result always has a value

    let time = 20;
    let greeting = if time < 18 {
        "Good day."
    } else {
        "Good evening."
    };
    println!("{}", greeting);
}
