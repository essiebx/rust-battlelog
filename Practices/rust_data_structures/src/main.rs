/*In Rust, data structures are used to store and organize values.

Rust provides many built-in data structures. Each is used to handle data in different ways.

Some of the most common are:

Array-fixed size
Vector (Vec)
Tuple
HashMap
//
Arrays
An array in Rust is a fixed-size list of values, all of the same type.

You cannot grow or shrink an array after it's created.

To access an array element, refer to its index number.

Array indexes start with 0: [0] is the first element, [1] is the second element, etc.

let fruits = ["apple", "banana", "orange"];
println!("Last fruit: {}", fruits[2]);

Vectors
A vector is a resizable array. Unlike regular arrays, vectors can grow or shrink in size.
let mut fruits = vec!["apple", "banana"];
fruits.push("cherry");

println!("Last fruit: {}", fruits[2]);

Tuples
A tuple can hold multiple values of different types. It is useful when grouping different types together.

You access tuple elements using a dot and an index number, like person.1, etc:

let person = ("John", 30, true);
println!("Name: {}", person.0);
println!("Age: {}", person.1);
println!("Is active: {}", person.2);


*/

fn main() {
    println!("Hello, world!");
}
