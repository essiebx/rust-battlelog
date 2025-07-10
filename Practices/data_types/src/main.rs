fn main() {
    /* rust has 2 types of data types:
       1. Scalar Types: represent a single value.
       Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
          - Integer: i8, i16, i32, i64, i128, isize
          - float: f32, f64

         bool: true or false
        - Character/char(4 bytes): single Unicode character (e.g., 'a', '🦀')
       2. Compound Types: can group multiple values
          - Tuple: fixed size, can have different types
          - Array: fixed size, same type elements

    for numbers  theres 2 types of integers:
            - Signed: i8, i16, i32, i64, i128,
            isize (can be negative or positive)
            - Unsigned: u8, u16, u32, u64, u128
            usize (only positive)
         Rust also has a special type called `unit` which is represented by `()`
         It is used when there is no meaningful value to return, like in functions that return nothing.


     */

    //  // tuple
    //  let tuple: (i32, f64, char) = (42, 3.14, 'a');
    //  let (x, y, z) = tuple; // destructuring
    //  // above is fixed heterogeneous size(3 elements) heterogeneous data type meaning it can have different types of data in it
    //  assert_eq!(x, 42);

    //  // array
    //  let array: [i32; 3] = [1, 2, 5]; // fixed size (3 elements) homogeneous data type meaning it can only have same types of data in it
    //  let first_element = array[0]; // accessing first element

    //  // char
    //  let letter: char = 'A'; // single Unicode character
    //  assert_eq!(letter, 'A');
    //  // boolean
    //  let is_rust_fun: bool = true; // boolean value
    //  assert!(is_rust_fun); // checking if true
    //  // integer
    //  let age: i32 = 30; // signed integer
    //  assert_eq!(age, 30);
    //  // unsigned integer
    //  let positive_number: u32 = 42; // unsigned integer
    //  assert_eq!(positive_number, 42);
    //  // floating point
    //  let pi: f64 = 3.14159; // double precision floating point
    //  assert_eq!(pi, 3.14159);//checks
    //  // unit type
    //  let unit_value: () = (); // unit type denoted by ()/, represents no value
    //  assert_eq!(unit_value, ()); // checking if unit value is equal to unit type
    //  // string
    //  let greeting: &str = "Hello, Rust!"; // string slice- r
    //  assert_eq!(greeting, "Hello, Rust!"); // checking if string is equal
    //  // string literal-this is a string slice, which is a reference to a string literal
    //  let name: String = String::from("Rustacean"); // owned string(heap allocated-meaning it can grow or shrink in size)
    //  assert_eq!(name, "Rustacean"); // checking if string is equal
    //  // rust has how to handle strings in a safe way, it has a String type which is an owned string, and a &str type which is a string slice, which is a reference to a string literal.
    //  // lesson learned :
    let my_string: String = String::from("bonnita");
    let my_int: i32 = my_string.parse().expect("failed");

    println!(
        "the data type changed from string to int now it is: {}",
        my_int,
    );
}
