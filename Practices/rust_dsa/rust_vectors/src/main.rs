/*Vectors
A vector is a resizable array. Unlike regular arrays, vectors can grow or shrink in size.

Creating a Vector
To create a vector, use the vec! macro


*/
fn main() {
    //syntax
    // Creating a vector using the vec! macro
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Vector: {:?}", numbers);

    // Creating a vector with initial values;
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("Initial vector: {:?}", numbers);

    // Accessing vector elements
    println!("First element: {}", numbers[0]);
    let fruits = vec!["apple", "banana", "orange"];
    println!("First fruit: {}", fruits[0]);

    // Looping through a vector
    for number in numbers.iter() {
        println!("Number: {}", number);
    }

    /*Change Vector Values
    To change a value in the vector, refer to the index number and assign a new value.

    Remember to make the vector mutable (using the mut keyword): */

    let mut fruits = vec!["apple", "banana", "orange"];
    fruits[0] = "grape";
    println!("New first fruit: {}", fruits[0]);

    /*Add Elements to a Vector
    You can add a new element to the end of a vector using the push() method: */
    let mut numbers = vec![1, 2, 3, 4, 5];
    numbers.push(6);
    println!("Updated vector: {:?}", numbers);

    let sol_accounts = vec!["Alice", "Bob", "Charlie"];
    println!("Solana accounts: {:?}", sol_accounts);
    // You can also add multiple elements at once using the extend() method
    let mut solana_ecosystem_list = vec![
        "jupiter", "solana", "saber", "celo", "raydium", "orca", "bonfida",
    ];
    solana_ecosystem_list.extend(vec!["mango", "serum", "step finance", "saber", "wormhole"]);
    println!("Solana ecosystem list: {:?}", solana_ecosystem_list);
    // Remove Elements from a Vector
    //To remove the last element from a vector, use pop():
    let mut fruits = vec!["apple", "banana", "cherry"];
    fruits.pop();
    println!("{:?}", fruits); // ["apple", "banana"]

    /*Add or Remove Elements at a Specified Index
    Rust vectors are designed to grow and shrink at the end, but you can also add or remove elements at the beginning or at a specified index.

    Use insert() to add an item at a specified index: */

    let mut fruits = vec!["banana", "orange"];
    fruits.insert(0, "apple");
    println!("{:?}", fruits); // ["apple", "banana", "orange"]

    let mut fruits = vec!["banana", "orange"];
    fruits.insert(1, "apple");
    println!("{:?}", fruits); // ["banana", "apple", "orange"]

    /*Remove the First Item
    Use remove() to remove an element from a specified index: */

    let mut fruits = vec!["apple", "banana", "orange"];
    fruits.remove(0);
    println!("{:?}", fruits); // ["banana", "orange"]

    /*Vector Length
    You can find out how many elements there are in a vector using the .len() method: */
    let numbers = vec![1, 2, 3, 4, 5];
    println!("The length of the vector is: {}", numbers.len());

    /*Loop Through a Vector
    Just like arrays, you can use a for loop to go through all the values in a vector: */
    for number in numbers.iter() {
        println!("Number: {}", number);
    }
    /*Note: Use &fruits to borrow the vector instead of moving it.

    In Rust, borrowing means using a reference to a value instead of taking ownership of it. When you loop through a vector without &, the values are moved out, and you can no longer use the vector.
     But when you borrow the vector using &, you can still use it later in your program. */
    for fruit in &fruits {
        println!("Fruit: {}", fruit);
    }
}
