/*
scope is how variables are accessed in Rust in different parts of the code./function
A variable only lives inside the block where it was created. A block is anything inside curly braces { }.
Variable Inside a Function
A variable created inside a function only exists inside that function:


*/
fn my_function() {
    let x = 10; // This variable is only accessible inside this function
    println!("x inside my_function: {}", x);
}

fn main() {
    my_function(); // This will print: x inside my_function: 10

    // Uncommenting the next line will cause a compile-time error because x is not accessible here
    // println!("x outside my_function: {}", x);

    // Variable Inside a Block
    /*Variable Inside a Block
    You can also create blocks inside other code, like in if statements or loops.
    Variables created in these blocks are only valid inside them */
    {
        let y = 20; // This variable is only accessible inside this block
        println!("y inside block: {}", y);
    }

    // Uncommenting the next line will cause a compile-time error because y is not accessible here
    // println!("y outside block: {}", y);
    let score = 80;

    if score > 50 {
        let result = "Pass";
        println!("Result: {}", result);
    }

    println!("Result: {}", result); // Error: result is out of scope here
    // Uncommenting the next line will cause a compile-time error because result is not accessible here
    // println!("Result: {}", result);

    // Variable Shadowing
    /*Variable Shadowing
    In Rust, you can create a new variable with the same name as an existing one.
    This is called variable shadowing. The new variable "shadows" the old one,
    meaning the old one is no longer accessible in that scope. This can be useful for changing

    the type or value of a variable without needing to create a new name.*/
    let z = 30; // Original variable
    println!("z before shadowing: {}", z);
    {
        let z = 40; // New variable with the same name
        println!("z inside block: {}", z); // This will print the new value
    }
    println!("z after block: {}", z); // This will print the original value
    // Variable Shadowing with Different Types
    /*Variable Shadowing with Different Types
    You can also change the type of a variable by shadowing it. This is useful
    when you want to transform a value without needing to create a new variable name.*/
    let a = "Hello"; // Original variable is a string
    println!("a before shadowing: {}", a);
    {
        let a = 42; // New variable with the same name, but now it's an
        println!("a inside block: {}", a); // This will print the new value
    }
    // Variables in the Same Scope
    // Two variables cannot have the same name in the same scope.
    // Uncommenting the next line will cause a compile-time error because a is already defined in this scope
    // let a = "World"; // Error: a is already defined in this scope

    // Using Variables Outside Their Scope
    /*Using Variables Outside Their Scope
    If you try to use a variable outside the block where it was created, Rust will give you an error.
    This helps prevent bugs by ensuring you only use variables where they are valid.*/
    // Uncommenting the next line will cause a compile-time error because x is not accessible here
    // println!("x outside my_function: {}", x);
    // Using Variables in Nested Blocks
    /*Using Variables in Nested Blocks
    You can create variables in nested blocks, and they will only be accessible within those blocks.
    This is useful for organizing code and keeping variables local to specific parts of your program.*/
    {
        let nested_var = "I'm inside a nested block";
        println!("nested_var: {}", nested_var);
    }

    /*Why Scope Matters
    Understanding scope helps you:

    Know where a variable can be used
    Prevent naming conflicts
    Avoid errors when working with functions, loops, and conditionals */
}
