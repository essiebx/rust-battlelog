/*
for  loops in Rust are used to iterate over a range of values or collections.
When you know exactly how many times you want to loop through a block of code,
use the for loop together with the in keyword, instead of a while loop:
// They are useful when you want to repeat a block of code a specific number of times or over a collection of items.
// The syntax for a for loop is:


for variable in collection {
    // code to be executed
}

//         println!("Number: {}", num);
//         num += 1; // Increment num by 1
//     }

*/
fn main() {
    for i in 1..6 {
        println!("i is: {}", i);
    }
    /*this prints numbers from 1 to 5.

    Note: 1..6 means from 1 up to (but not including) 6.

    Note: Rust handles the counter variable (i) automatically,
    unlike many other programming languages. You don't need to declare or increment it manually.
    */
    //Inclusive Range
    //If you want to include the last number, use ..= (two dots and an equals sign)
    for anything in 1..=6 {
        println!("anything is: {}", anything);
    }
    /*This prints numbers from 1 to 6.
    Note: 1..=6 means from 1 to 6, including both endpoints.
    Note: Rust handles the counter variable (anything) automatically,
    */

    // Break and Continue
    /*you can use break to stop the loop and continue to skip a value */
    for number in 1..=10 {
        if number == 5 {
            println!("Skipping number 5");
            continue; // Skip the rest of the loop when number is 5
        }
        if number == 8 {
            println!("Breaking at number 8");
            break; // Stop the loop when number is 8
        }
        println!("Number: {}", number);
    }
    for i in 1..=10 {
        if i == 3 {
            continue; // skip 3
        }
        if i == 5 {
            break; // stop before printing 5
        }
        println!("i is: {}", i);
    }
    //     /*This prints numbers from 1 to 10, skipping 3 and stopping before 5.
    //     Note: continue skips the rest of the loop for that iteration, while break stops
    //     the loop entirely.

    /*Rust Loops Summary
    Rust has three types of loops that let you run code over and over again. Each one is used in different situations:

    1. loop
    The simplest kind of loop. It runs forever unless you stop it with break */
    loop {
        // do something
        if condition {
            break;
        }
    }
    // 2. while
    // Runs as long as a condition is true. It checks the condition before each iteration.
    while condition {
        // do something
    }
    while count <= 5 {
        println!("{}", count);
        count += 1;
    }
    // Use while when you want to repeat code until something happens.
    // 3. for
    // Used to loop through a range of numbers or items in a collection.
    for item in collection {
        // do something with item
    }
    for i in 1..=5 {
        println!("{}", i);
    }
    // Use for when you know how many times you want to repeat code or when you want to go through a list of items.
    /*Extra Keywords
    You can use these in any loop:

    break - stop the loop
    continue - skip a value in the loop
     */
}
