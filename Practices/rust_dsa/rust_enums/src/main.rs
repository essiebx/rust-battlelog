/*
in programing enums are a way to define a type that can be one of several different variants.
Enums are useful when you want to represent a value that can take on multiple forms, like a
status code that can be success, error, or pending.
Enums can also have data associated with each variant, allowing you to store additional information
for each variant.
Enums are defined using the enum keyword, followed by the name of the enum and its variants.
Enums can also have methods associated with them, allowing you to define behavior for each variant.
Enums are often used in Rust to represent a fixed set of options or states, making your code
more type-safe and expressive.

Enums
An enum (short for "enumeration") is a way to define a type that can be one of a few different values.

Each value in the enum is called a variant.

Enums are useful when you want to represent a value that can only be one of a set of options - like days of the week, directions, or results like success and error.

Create an Enum
To create an enum, use the enum keyword and add a set of named values (variants) separated by commas:

*/

fn main() {
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    // Create a variable of type Direction
    let move_direction = Direction::Up;
    // Use a match statement to handle different variants
    match move_direction {
        Direction::Up => println!("Moving up!"),
        Direction::Down => println!("Moving down!"),
        Direction::Left => println!("Moving left!"),
        Direction::Right => println!("Moving right!"),
    }
    /*Match on Enum Values
    Enums work great with the match statement. You can run different code depending on which variant is used: */
    enum Status {
        Success,
        Error(String),
        Pending,
    }
    /*Enums with Data
    Enum variants can also hold data. This is useful when each variant needs to store extra information: */
    enum LoginStatus {
        Success(String),
        Error(String),
    }

    fn main() {
        let result1 = LoginStatus::Success(String::from("Welcome, John!"));
        let result2 = LoginStatus::Error(String::from("Incorrect password"));

        match result1 {
            LoginStatus::Success(message) => println!("Success: {}", message),
            LoginStatus::Error(message) => println!("Error: {}", message),
        }
    }
    /*Why Use Enums?
    To group related values into one type
    To make your code more readable and safe
    To handle different cases with match */
}
