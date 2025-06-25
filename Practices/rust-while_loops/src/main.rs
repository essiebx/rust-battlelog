/* while loops execute code as long as a specified condition is true.
// They are useful when you want to repeat a block of code until a certain condition is met
// or when you don't know how many times you need to repeat the code.
// The syntax for a while loop is:
// while condition {
//     // code to be executed
// }


*/

fn main() {
    // Example of a while loop that runs as long as a condition is true
    let mut count = 1;
    while count <= 3 {
        println!("Hello!");
        count += 1; // Increment count by 1
    }
    /*False Condition
    The while loop checks the condition before each loop, so if the condition is false at the start, the loop will not run at all:
     */

    let count = 10;

    while count <= 5 {
        println!("This won't be printed.");
    }
    // Example of a while loop that runs until a condition is false
    let mut number = 5; // Initialize a mutable variable number to 5
    while number > 0 {
        println!("Number is: {}", number);
        number -= 1; // Decrement number by 1
    }

    // Example of a while loop that checks a condition before executing the code
    let mut i = 0; // Initialize a mutable variable i to 0
    while i < 3 {
        println!("i is: {}", i);
        i += 1; // Increment i by 1
    }

    /*Stop a While Loop
    You can stop a while loop when you want by using break: */

    let some_mut_variable = 10; // Initialize a mutable variable some_mut_variable to 10
    while some_mut_variable > 0 {
        println!("some_mut_variable is: {}", some_mut_variable);
        some_mut_variable -= 1; // Decrement some_mut_variable by 1
        if some_mut_variable == 5 {
            println!("Stopping the loop because some_mut_variable is now 5.");
            break; // This will stop the loop when some_mut_variable reaches 5
        }
    }

    let mut num = 1;

    while num <= 10 {
        if num == 6 {
            break;
        }
        println!("Number: {}", num);
        num += 1;
    }
    // This prints numbers from 1 to 5 (stops the loop when num reaches 6).

    /*
        Skip a Value
    You can skip a value by using the continue statement:
         */
    let mut num = 1;

    while num <= 10 {
        if num == 6 {
            num += 1;
            continue;
        }

        println!("Number: {}", num);
        num += 1;
        // This prints numbers from 1 to 10, except for the number 6.
    }
}
