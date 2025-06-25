/*Constants
Constant variables are used to store values that never change.

Unlike regular variables, constants must be defined with a type (e.g. i32 or char).
To create a constant, use the const keyword, followed by the name, type, and value:
const year_of_birth: i32 = 1960; ->this was actually wrong approach of writing the name of the constant
println!("Year of birth: {}", year_of_birth);
Constants Must Have a Type
Constants in Rust must always have a type annotation, unlike regular variables which can be inferred
names should lwayas be in upper case and use underscores to separate words
Constants vs Variables
Here's a quick comparison:

Feature	        Constant (const)	Variable (let)
Can change?	    No               	Yes, if mut is used
Type required?	Yes	                 No (optional)

 */

fn main() {
    const YEAR_OF_BIRTH: i32 = 1960; // Constants must be in uppercase
    println!("your Year of birth is: {}", YEAR_OF_BIRTH);
    // Constants must have a type annotation-> type here is i32(32-bit signed integer)
    const PI: f64 = 3.14159; // Example of a constant with a floating-point type
    println!("Value of PI: {}", PI);
    const GRADE: char = 'A'; // Example of a constant with a character type
    println!("My grade is: {}", GRADE);
    const GREETING: &str = "Hello, Rust!"; // Example of a constant with a string slice type
    // String slices (&str) are used for string constants- slices are immutable views into a string
    let slice = &"Greeting"[1..3]; // Example of a string slice. [1..3] means characters from index 1 to 2 (exclusive of 3)
    println!("Slice of the string: {}", slice);
    println!("{}", GREETING);
}
