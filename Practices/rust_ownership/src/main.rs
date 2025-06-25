/*Rust uses "ownership" to manage memory in a safe way.

Every value in Rust has an owner. The owner is usually a variable.
Ownership Rules
Each value has one owner
When the owner goes out of scope, the value is deleted
You can only have one owner at a time, unless you borrow it
// Borrowing allows you to use a value without taking ownership


*/

fn main() {
    let ma = String::from("Hello");
    let ba = ma;
    // ma is no longer valid after this point because ba now owns the value
    // println!("{}", ma); // Error: ma no longer owns the value
    // In this example, ma owns the string. Then we move it to ba:
    /*When we assign ma to ba, the ownership moves. This means only ba can use the value now, because ma is no longer valid.

    But simple types like numbers, characters and booleans are copied, not moved.

    This means you can still use the original variable after assigning it to another: */
    let a = 5;
    let b = a;
    println!("a = {}", a); // Works
    println!("b = {}", b); // Works
    // above Here, a is copied into b, not moved, so you can still use b.

    // println!("{}", a); Error: a no longer owns the value
    println!("{}", b); // Ok: b now owns the value

    // Clone
    /*use clone method when you want to preserve the data of  the other variable as well as the space bust still makes it to get another through it.
    this is common in strings
     like String, if you really want to keep the original value and also assign it to another variable, you can use the .clone() method, which makes a copy of the data:
     */
    let a = "hellow world".to_string();
    let b = a.clone(); // Clone the string to create a new owner
    let a = String::from("Hello");
    let b = a.clone(); // Now both have the same value

    println!("a = {}", a); // Works
    println!("b = {}", b); // Works
    // However, if you don't need to own the value twice, using a reference (&) is usually better than cloning

    /*Why Ownership Matters
    Rust uses ownership to automatically free memory when it's no longer needed
    It prevents bugs like using memory that's already been deleted
    It is one of the reasons Rust is so safe and fast
    Next: Learn about borrowing - how to let other parts of your program use a value without taking ownership. */
}
