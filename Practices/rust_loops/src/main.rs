/*
Loops can execute a block of code as long as a specified condition is reached.

Loops are handy because they save time, reduce errors, and they make code more readable. For example, instead of writing the same line 10 times to print some text, you can use a loop to repeat it for you.

Rust has three types of loops: loop, while, and for
loop
loop is the simplest of Rust's three loop types.

It will run forever unless you tell it to stop
// by using the break keyword.
It can be used to create infinite loops or to repeat a block of code until a certain condition
is met
To stop a loop, use the break keyword:
*/
fn main() {
    //syntax:
    // loop {
    //     // code to be executed
    // }
    // Example of a loop that runs forever
    loop {
        println!("This will repeat forever!");
    }
    // Example of a loop that runs forever but breaks after one iteration
    loop {
        println!("This will run forever unless we break out of it!");
        break; // This will stop the loop after one iteration
    }

    let mut count = 0; // Initialize a mutable variable count to 0
    loop {
        //opening a loop
        count += 1; // Increment count by 1
        println!("Count is: {}", count);
        if count == 5 {
            // Check if count has reached 5
            println!("Count reached 5, breaking the loop.");
            break; // This will stop the loop when count reaches 5
        }
        /*This prints "Hello World!" 3 times.
        It uses a counter to keep track of how many times it has looped.
        The counter starts at 1 (let mut count = 1;).
        Each time the loop runs, the counter goes up by 1: (count += 1;).
        When it reaches 3, the loop stops. */
    }

    //Return a Value
    /*You can also return a value from a loop using break with a value.

    This lets you save the result of the loop into a variable */
    let mut sum = 0; // Initialize a mutable variable sum to 0
    let mut i = 1; // Initialize a mutable variable i to 1
    loop {
        sum += i; // Add i to sum
        i += 1; // Increment i by 1
        if i > 10 {
            break sum; // Break the loop and return the value of sum when i is greater than
            // 10
        }
    }
    let mut count = 1;

    let result = loop {
        println!("Hello!");

        if count == 3 {
            break count; // Stop the loop and return the number 3
        }

        count += 1;
    };

    println!("The loop stopped at: {}", result);
}
