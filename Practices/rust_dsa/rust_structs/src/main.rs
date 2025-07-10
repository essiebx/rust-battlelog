/*Structs
A struct (short for "structure") is a custom data structure that lets you group related values together.

You can think of a struct like a mini-database for one thing, like a person with a name and age.

Create a Struct
You define a struct using the struct keyword and place the fields (variables) inside: */
fn main() {
    // Define a struct called Person

    struct Person {
        name: String,
        age: u32,
        can_vote: bool,
    }
    // Create an instance of the Person struct
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
        can_vote: true,
    };
    // Accessing struct fields
    println!(
        "Name: {}, Age: {}, Can Vote: {}",
        person1.name, person1.age, person1.can_vote
    );
    // You can also create a mutable struct
    let mut person2 = Person {
        name: String::from("Bob"),
        age: 25,
        can_vote: false,
    };
    // Modify a field
    person2.age = 26;
    println!("Updated Age: {}", person2.age);
    // Structs can also have methods
    impl Person {
        fn is_adult(&self) -> bool {
            self.age >= 18
        }
    }

    // Call the method
    if person1.is_adult() {
        println!("{} is an adult.", person1.name);
    } else {
        println!("{} is not an adult.", person1.name);
    }

    // Structs can also have associated functions
    impl Person {
        fn new(name: &str, age: u32, can_vote: bool) -> Person {
            Person {
                name: String::from(name),
                age,
                can_vote,
            }
        }
    }

    // Create a new person using the associated function
    let person3 = Person::new("Charlie", 20, true);
    println!(
        "Created new person: {}, Age: {}, Can Vote: {}",
        person3.name, person3.age, person3.can_vote
    );

    //Change a Field
    // To change a value inside a struct, you must make the struct object mutable by using mut
    struct Person {
        name: String,
        age: u32,
    }

    let mut user = Person {
        name: String::from("John"),
        age: 35,
    };

    user.age = 36; // Change value of age
    println!("Name: {}", user.name);
    println!("Updated age: {}", user.age);
    /* structs methods:
    Methods are functions that are associated with a struct. They allow you to define behavior for your struct.
    You define methods using the impl keyword followed by the struct name. Inside the impl block,
    you can define methods that take &self as the first parameter to access the struct's fields
    and perform operations on them.


     */

    /*Why Use Structs?
    To group related data in a clean way
    To make your code easier to read and maintain
    To create real-world examples, like users, books, cars, etc. */
}
