/*In Rust, data structures are used to store and organize values.

Rust provides many built-in data structures. Each is used to handle data in different ways.

Some of the most common are:

Array
Vector (Vec)
Tuple
HashMap


Arrays
An array in Rust is a fixed-size list of values, all of the same type.

You cannot grow or shrink an array after it's created.

To access an array element, refer to its index number.

Array indexes start with 0: [0] is the first element, [1] is the second element, etc.

Vectors
A vector is a resizable array. Unlike regular arrays, vectors can grow or shrink in size.

Vectors are defined using the Vec type.

Tuples
A tuple can hold multiple values of different types. It is useful when grouping different types together.

You access tuple elements using a dot and an index number, like person.1,
where person is the tuple name and 1 is the index of the element you want to access.


HashMaps
A HashMap stores key-value pairs. It lets you look up a value using a key.

To use HashMap, you must import it from the standard library.
*/
fn main() {
    // Example of an array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", arr);

    let fruits = ["apple", "banana", "orange"];
    println!("Last fruit: {}", fruits[2]);

    // Example of a vector
    let mut vec: Vec<i32> = vec![1, 2, 3];
    vec.push(4);
    println!("Vector: {:?}", vec);

    let mut fruits = vec!["apple", "banana"];
    fruits.push("cherry");

    println!("Last fruit: {}", fruits[2]);

    // Example of a tuple
    let person: (&str, i32) = ("Alice", 30);
    println!("Tuple: Name - {}, Age - {}", person.0, person.1);

    let person = ("John", 30, true);
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is active: {}", person.2);

    // Example of a HashMap
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert("Alice", 10);
    scores.insert("Bob", 20);
    println!("HashMap: {:?}", scores);

    // Import HashMap
    use std::collections::HashMap;

    fn main() {
        let mut capitalCities = HashMap::new();
        capitalCities.insert("France", "Paris");
        capitalCities.insert("Japan", "Tokyo");

        println!("Capital of Japan is {}", capitalCities["Japan"]);
    }

    /*Data Structures Overview
    Data Structure	     Use Case	        Can Grow?
    Array	             Fixed-size list of same-type values	No
    Vector (Vec)	     Growable list of same-type values	Yes
    Tuple	             Group different types together	No
    HashMap	             Key-value lookup	Yes */
}
