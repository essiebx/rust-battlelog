/*
Tuples
A tuple is a group of values of different types, stored in a single variable.

Tuples are useful when you want to return or work with multiple values together.

Create a Tuple
Tuples are written using parentheses (), with values separated by commas

*/

fn main() {
    let person: (&str, i32) = ("Alice", 30);
    println!("Name: {}, Age: {}", person.0, person.1);
    let person = ("John", 30, true);
    // Accessing tuple elements
    //You can access tuple values by using a dot . followed by the index:
    println!(
        "Name: {}, Age: {}, Is Student: {}",
        person.0, person.1, person.2
    );

    let (name, age) = person;
    println!("Name: {}, Age: {}", name, age);
    //Unpack a Tuple
    //When we create a tuple, we normally assign values to it. This is called "packing" a tuple:
    let person = ("Alice", 30, true);
    // Unpacking a tuple means extracting the values from it:
    //But, in Rust, we are also allowed to extract the values back into variables. This is called "unpacking":
    let (name, age, is_student) = person;
    println!("Name: {}, Age: {}, Is Student: {}", name, age, is_student);

    // Nested Tuples
    let nested_tuple = (1, 2.5, ("Hello", "World"));
    println!("Nested Tuple: {:?}", nested_tuple);
    println!("First Element: {}", nested_tuple.0);
    println!("Second Element: {}", nested_tuple.1);
    println!("Third Element: {}, {}", nested_tuple.2.0, nested_tuple.2.1);

    let person = ("Jenny", 45, false);
    let (name, age, active) = person;

    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Active: {}", active);

    //Return a Tuple from a Function
    //Tuples are often used to return multiple values from a function:
    fn get_person_info() -> (&'static str, i32, bool) {
        ("Alice", 30, true)
    }

    let person_info = get_person_info();
    println!(
        "Name: {}, Age: {}, Is Student: {}",
        person_info.0, person_info.1, person_info.2
    );

    // Tuples as Function Parameters
    //You can also use tuples as function parameters:
    fn print_person_info(person: (&str, i32)) {
        println!("Name: {}, Age: {}", person.0, person.1);
    }

    let person_tuple = ("Bob", 25);
    print_person_info(person_tuple);
    // Tuples with Different Types
    //Tuples can hold values of different types, making them versatile for various use cases:
    let mixed_tuple: (i32, f64, &str) = (42, 3.14, "Hello");
    println!(
        "Mixed Tuple: {}, {}, {}",
        mixed_tuple.0, mixed_tuple.1, mixed_tuple.2
    );
    /*tuples methods:
    Tuples in Rust do not have methods like vectors or strings, but you can use pattern matching to destructure them and access their elements.
    You can also use the .0, .1, etc. syntax to access elements by
    index, as shown in the examples above.



     */
}
