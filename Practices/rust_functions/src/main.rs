/*
functions are a way to group code together in such a way that it can be reused
// without having to rewrite it. Functions can take parameters and return values.
// They are defined using the fn keyword, followed by the function name, parameters, and a
// return type if applicable. The body of the function is enclosed in curly braces.
A function is a block of code that only runs when you call it.

Functions are used to organize your code, avoid repeating yourself, and make your program easier to understand.
// Functions can be called by their name followed by parentheses, and they can be used to
// perform specific tasks or calculations. Functions help in organizing code, making it more readable,
// and allowing for code reuse. They can also help in breaking down complex problems into smaller,
// manageable pieces. In Rust, functions can be defined at the top level or within other functions
// (nested functions). They can also be used as first-class citizens, meaning they can be
// passed as arguments to other functions, returned from functions, and assigned to variables.
// Functions can also be used to create closures, which are functions that capture their environment.
// Functions can also be used to create iterators, which are objects that allow you to iterate
// over a sequence of values. In Rust, functions can also be used to create traits,
// which are a way to define shared behavior across different types. Traits can be used to
// define methods that can be called on types that implement the trait. This allows for polymorphism(it is a
// way to define a common interface for different types, allowing them to be used interchangeably).
// Functions can also be used to create modules, which are a way to organize code into separate
// namespaces. Modules can be used to group related functions, types, and traits together,
// making it easier to manage and understand the codebase. In Rust, functions can also be
// used to create macros, which are a way to define reusable code snippets that can be expanded
// at compile time. Macros can be used to generate code, perform metaprogramming
// tasks, and create domain-specific languages (DSLs). They can also be used to create
// custom syntax extensions, allowing you to define new language constructs that can be used
// in your code. Macros can be defined using the macro_rules! keyword, and they
// can take arguments and produce code based on those arguments. Macros can also be used
// to create procedural macros, which are a way to define custom derive attributes
// that can be applied to types. Procedural macros can be used to generate code at compile


*/

/*Creating a Function
To create a function, use the fn keyword, followed by the function name and a set of parentheses () and curly braces {}: */
fn main() {
    //     fn function_name() {
    //   // code to be executed
    // }

    // Example of a simple function
    fn greet() {
        println!("Hello, world!");
    }

    // Calling the function
    //     Calling a Function
    // Now that you have created a function, you can execute it by calling it.

    // To call a function, write the name of the function followed by two parantheses ().
    greet();
    // Create a function
    fn say_hello() {
        println!("Hello from a function!");
    }

    say_hello(); // Call the function

    // Function with parameters
    // Functions can take parameters, which are values you pass to the function when you call it.

    // You can send information into a function using parameters. Parameters are written inside the parentheses ().
    fn greet_person(name: &str) {
        println!("Hello, {}!", name);
    }

    // Calling the function with an argument
    greet_person("Alice");
    //the function takes a string parameter called name and prints it in the greeting message.

    // Function with return value
    /*Functions with Return Values
    A function can also return a value.

    Use the  -> symbol in the function header to show what type of value will be returned.

    Inside the function, use the return keyword to send the value back */
    fn add(a: i32, b: i32) -> i32 {
        a + b // Implicit return
    }

    // In Rust, you can omit the return keyword. Just write the value on the last line of the function, without a semicolon:
    fn add_explicit(a: i32, b: i32) -> i32 {
        return a + b; // Explicit return
    }

    // Calling the function and storing the result
    let sum = add(5, 3);
    println!("Sum: {}", sum);
}
