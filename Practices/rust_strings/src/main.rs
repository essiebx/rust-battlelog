/*
Strings
Strings are used to store text.
you can use &str for string slices or String for owned strings.
&str is a reference to a string slice, while String is an owned string type that can
be modified.
This is important for memory management and avoiding bugs.
//     Avoid naming conflicts by ensuring variables have unique names in their scopes
//     Prevent errors by understanding where variables can be accessed
//     Use variables in nested blocks to keep them local to specific parts of your program
//     Use variable shadowing to change the type or value of a variable without creating a new
//     name
//     Use variables in nested blocks to keep them local to specific parts of your program
let greeting: &str = "Hello";
println!("{}", greeting);


hat strings are surrounded by double quotes ("Hello").

There are two main types of strings in Rust:

&str - is called "string slices", and is used for fixed text like "Hello"
String - used when you need a string that can change

*/

fn main() {
    let greeting: &str = "Hello";
    println!("{}", greeting);

    /*Create a String
    You can create a String from a string literal using the
     to_string() method or
     the String::from() function:

     It is up to you which one to choose - both to_string() and String::from() are very common in Rust.


     */
    let mut text1 = "Hello World".to_string();
    println!("text1: {}", text1);
    let mut text2 = String::from("Hello Rust"); // Create a mutable String
    // Change a String
    /*Strings are mutable, so you can change them if they are declared with mut.

    Use push_str() to add text to a string: */
    let mut greeting = String::from("Hello");
    greeting.push_str(" World");
    println!("{}", greeting); // Hello World

    text1.push_str(" - Welcome!"); // Append a string slice to the String
    println!("text1 after push_str: {}", text1);

    // Use push() to add a single character to a string:
    text2.push(' '); // Append a space character to the String
    text2.push('!'); // Append a character to the String.

    //Concatenate Strings
    //You can combine strings using the format! macro:
    let combined = format!("{} {}", text1, text2);
    println!("Combined: {}", combined);
    //You can also use the + operator to concatenate two strings, but the first string must be a String type:
    let text3 = text1 + " - " + &text2; // Note:
    // text1 is moved here, so it cannot be used again
    println!("text3: {}", text3);
    let s1 = String::from("Hello");
    let s2 = String::from("World!");
    let s3 = String::from("What a beautiful day!");
    let result = format!("{} {} {}", s1, s2, s3);
    println!("{}", result);

    //You can only add a &str to a String with +. That is why &s2 and &s3 (a string slice) is used here
    //to avoid moving the ownership of s2 and s3.

    //nb:format! is often the preferred choice than using + for combining strings.

    //String Length
    //You can get the length of a string using the len() method:
    let length = text1.len();
    println!("Length of text1: {}", length);

    //String Slices
    //You can create a slice of a string using the range syntax:
    let slice = &text1[0..5]; // Get the first 5 characters
    println!("Slice of text1: {}", slice);

    //Iterating Over Characters
    //You can iterate over the characters in a string using a for loop:
    for c in text2.chars() {
        println!("{}", c);
    }

    // Convert to uppercase
    let upper = text2.to_uppercase(); // "HELLO RUST !"

    // Convert to lowercase
    let lower = text2.to_lowercase(); // "hello rust !"

    // Replace a substring
    let replaced = text2.replace("Rust", "World"); // "Hello World !"

    // Check if it contains a substring
    let contains_rust = text2.contains("Rust"); // true

    // Split a string by whitespace or pattern
    for word in text2.split_whitespace() {
        println!("{}", word); // "Hello", "Rust", "!"
    }

    // Trim whitespace from ends
    let with_spaces = String::from("  padded string  ");
    let trimmed = with_spaces.trim(); // "padded string"

    // Remove characters using retain
    let mut filtered = String::from("R2D2");
    filtered.retain(|c| c.is_alphabetic()); // "RD"

    // Reverse a string
    let reversed: String = text2.chars().rev().collect();
    /*Shared Between &str and String (Both Can Use)
    .len() – returns the number of bytes.

    .to_uppercase() / .to_lowercase() – returns a new String.

    .contains("text") – checks for a substring.

    .replace("a", "b") – returns a new String.

    .split(), .split_whitespace(), .chars() – used for iteration or tokenizing.

    You can call these on either type because String implements Deref to &str, which allows borrowing its content as a string slice.

    🛠️ Only for String (Because They Modify)
    .push_str("text") – appends a &str to a String.

    .push('x') – appends a single char.

    .clear() – empties the string.

    .truncate(n) – shortens the string.

    .retain() – filters characters based on a predicate.

    .capacity() – returns the memory capacity (since it’s heap-backed).

    🔄 Converting Between the Two
    &String can be used as &str automatically, thanks to Deref.

    To turn a &str into a String, use .to_string() or String::from(...). */
}
