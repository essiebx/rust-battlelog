/*Borrowing and References


Sometimes you want to use a value without taking ownership of it.

Rust lets you do this using a reference - this is called borrowing:

What is a Reference?
A reference lets you look at a value without owning it. You create a reference using the & symbol:

*/
fn main() {
    let a = "hey whats up !".to_string();
    let b = &a;
    println!("a = {}", a);
    println!("b = {}", b);
    // Since b is only borrowing the value, a still owns it.

    // Mutable References
    // If you want to change a value through a reference, you need to make the reference mut:

    let mut name = String::from("John");
    let name_ref = &mut name;
    name_ref.push_str(" Doe");

    println!("{}", name_ref); // John Doe
    /*Note: You can only have one mutable reference to a value at a time!

Why Borrowing is Important
Borrowing helps you reuse values safely, without giving them away.

It lets you use values without taking ownership
It avoids cloning, which can be slow for large data
It makes your programs safer and faster
 */
}
