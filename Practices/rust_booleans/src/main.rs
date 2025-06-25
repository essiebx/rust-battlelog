/*
Very often, in programming, you will need a data type that can only have one of two values, like:

YES / NO
ON / OFF
TRUE / FALSE
For this, Rust has a bool data type, which is known as booleans.

Booleans represent values that are either true or false.
Booleans are often used in conditional statements, loops, and logical operations.
Creating Boolean Variables
You can store a boolean value in a variable using the bool type:

*/
fn main() {
    let my_bool: bool = true; // A boolean variable
    println!("My boolean value is: {}", my_bool);
    let another_bool: bool = false; // Another boolean variable
    println!("Another boolean value is: {}", another_bool);
    // Boolean expressions
    let is_greater = 5 > 3; // This will be true
    let is_equal = 5 == 5; // This will also be true
    println!("Is 5 greater than 3? {}", is_greater);
    println!("Is 5 equal to 5? {}", is_equal);
    // Using booleans in conditions
    if my_bool {
        println!("The boolean is true, so we execute this block.");
    } else {
        println!("The boolean is false, so we execute the else block.");
    }
    // Combining booleans with logical operators
    let a = true;
    let b = false;
    let and_result = a && b; // false
    let or_result = a || b; // true
    let not_result = !a; // false
    println!("AND result: {}", and_result);
    println!("OR result: {}", or_result);
    println!("NOT result: {}", not_result);

    let is_programming_fun: bool = true;
    let is_fish_tasty: bool = false;

    println!("Is Programming Fun? {}", is_programming_fun);
    println!("Is Fish Tasty? {}", is_fish_tasty);
    // Rust is smart enough to understand that true and false values are boolean values,
    //  meaning that you don't have to specify the bool keyword:

    /*Boolean from Comparison
    Most of the time, there is no need to type true or false yourself.
    Instead, boolean values come from comparing values using operators like == or >:
     */
    let is_greater_than = 10 > 5; // This will be true
    let is_equal_to = 10 == 10; // This will also be true
    println!("Is 10 greater than 5? {}", is_greater_than);
    let age = 20;
    let can_vote = age >= 18;

    println!("Can vote? {}", can_vote);

    // Booleans in if Statements
    // Boolean values are often used in if statements to decide what code should run
    if can_vote {
        println!("You can vote!");
    } else {
        println!("You cannot vote yet.");
    }

    let is_logged_in = true;

    if is_logged_in {
        println!("Welcome back!");
    } else {
        println!("Please log in.");
    }

    // Boolean expressions in conditions
    if is_greater && is_equal {
        println!("Both conditions are true, so we execute this block.");
    } else {
        println!("At least one condition is false, so we execute the else block.");
    }
    // Using booleans in loops
    let mut count = 0;
    while count < 5 {
        println!("Count is: {}", count);
        count += 1;
    }
    // Using booleans in match statements

    let status = if my_bool { "Active" } else { "Inactive" };
    match status {
        "Active" => println!("The status is active."),
        "Inactive" => println!("The status is inactive."),
        _ => println!("Unknown status."),
    }
}
