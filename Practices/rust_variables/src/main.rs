/* Variables are storage mechanisms in all languages.
Rust has two types of variables: immutable and mutable.
By default, variables in Rust are immutable(cant be changed).
The `mut` keyword allows a variable's value to be modified.
Note: `mut` affects the value, not the data type.
In Rust, the type of a variable is decided by the value you give it.
 Rust looks at the value and automatically chooses the right type:
let my_num = 5;         // integer
let my_double = 5.99;   // float
let my_letter = 'D';    // character
let my_bool = true;     // boolean
// let my_text = "Hello";  // string
you can explicitly specify the type of a variable using a colon `:` followed by the type name.
For example, to declare a variable as an integer, you can write:
let my_num: i32 = 5; // explicitly typed as i32
 Basic data types in Rust are divided into different groups:

Numbers - Whole numbers and decimal numbers (i32, f64)
Characters - Single letters or symbols (char)
Strings - Text, a sequence of characters (&str)
Booleans - True or false values (bool)
Numbers
Number types are divided into two groups: integer types and floating point types.

Integer (i32)
The i32 type is used to store whole numbers, positive or negative (such as 123 or -456), without decimals:

Example
let age: i32 = 25;
println!("Age is: {}", age);
Floating Point (f64)
The f64 type is used to store numbers containing one or more decimals:

Example
let price: f64 = 19.99;
println!("Price is: ${}", price);
Characters (char)
The char type is used to store a single character. A char value must be surrounded by single quotes, like 'A' or 'c':

Example
let myGrade: char = 'B';
println!("{}", myGrade);

Strings (&str)
The &str type is used to store a sequence of characters (text). String values must be surrounded by double quotes:

Example
let name: &str = "John";
println!("Hello, {}!", name);
Booleans (bool)
The bool type can only take the values true or false:

Example
let is_logged_in: bool = true;
println!("User logged in? {}", is_logged_in);

Constants vs Variables
Here's a quick comparison:

Feature	        Constant (const)	Variable (let)
Can change?	    No               	Yes, if mut is used
Type required?	Yes	                 No (optional)


*/

fn main() {
    // ✅ Mutable variable
    let mut x = 2;
    println!("Initial value of x is : {}", x);
    // {} is a placeholder for the value of x to show variable value.values are passed in order of appearance

    x = 5; // Modify the value
    println!("Updated value of x: {}", x);

    // ❌ Immutable variable (uncomment to see compile error)
    let y = 3;
    println!("y = {}", y);
    // y = 4;
    // println!("Updated y = {}", y);

    // ✅ Shadowing = redeclaring a variable with `let` (new memory allocation)
    let meri_5x = 4;
    let meri_5x = meri_5x + 5;
    println!("The new shadowed value of meri_5x is: {}", meri_5x);

    // 🧪 Call test function with assert_eq!()
    assert_eq!(meri_5x, 9);
}

// 🧪 Separate function for assertion practice
fn test_shadowing_vs_mutability() {
    // 🟢 Mutability
    let mut a = 10;
    a = a + 5;
    assert_eq!(a, 15);

    // 🔁 Shadowing
    let b = 20;
    let b = b + 1;
    assert_eq!(b, 21);

    // 🔀 Shadowing can change type
    let spaces = "   ";
    let spaces = spaces.len(); // now usize
    assert_eq!(spaces, 3);

    println!("All assertions passed. ✅");

    // Key Takeaway:
    // Use mut when you want to change a value in-place.
    // Use shadowing (let) when you want to rebind or transform the value (e.g., from a String to its .len()).
}
