/*
Match
When you have many choices, using match is easier than writing lots of if...else.

match is used to select one of many code blocks to be executed

*/
fn main() {
    let day = 4;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day."),
    }

    /*the match variable (day) is evaluated once.
    The value of the day variable is compared with the values of each "branch"
    Each branch starts with a value, followed by => and a result
    If there is a match, the associated block of code is executed
    _ is used to specify some code to run if there is no match (like default in other languages).
    In the example above, the value of day is 4, meaning "Thursday" will be printed
    */

    /*
    Multiple Matches
    You can match multiple values at once using the | operator (OR)
    */
    let solana_account = "solana";
    match solana_account {
        "solana" | "SOL" => println!("This is a Solana account."),
        "ethereum" | "ETH" => println!("This is an Ethereum account."),
        "bitcoin" | "BTC" => println!("This is a Bitcoin account."),
        _ => println!("Unknown cryptocurrency account."),
    }

    fn main() {
        let day = 6;

        match day {
            1 | 2 | 3 | 4 | 5 => println!("Weekday"),
            6 | 7 => println!("Weekend"),
            _ => println!("Invalid day"),
        }
    }

    // match with a Return Value
    // Just like if, match can also return a value:

    // Each part of the match branches must be the same type - just like with if...else.

    // This means you can save the result of a match into a variable
    let number = 5;
    let result = match number {
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        _ => "Unknown number",
    };
    println!("The number is: {}", result);
    // Match with a Range
    let txn_amount = 1500;
    let txn_type = match txn_amount {
        0..=1000 => "Low Value Transaction",
        1001..=5000 => "Medium Value Transaction",
        5001..=10000 => "High Value Transaction",
        _ => "Very High Value Transaction",
    };
    println!("Transaction Type: {}", txn_type);
    // Match with a Struct
    #[derive(Debug)]
    struct User {
        username: String,
        age: u32,
    }
    let user = User {
        username: String::from("alice"),
        age: 30,
    };
}
