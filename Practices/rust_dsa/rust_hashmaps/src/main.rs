/*
HashMap
A HashMap is a collection of key/value pairs.

HashMaps are great when you want to store values and find them by a key.

To use HashMap, you must import it from Rust's standard library:

use std::collections::HashMap;


*/
fn main() {
    //syntax of a hashmap

    // Import HashMap
    use std::collections::HashMap;

    fn main() {
        // Create a HashMap called capitalCities
        let mut capitalCities = HashMap::new();

        // Add keys and values (Country, City)
        capitalCities.insert("England", "London");
        capitalCities.insert("Germany", "Berlin");
        capitalCities.insert("Norway", "Oslo");

        println!("{:?}", capitalCities);
    }

    use std::collections::HashMap;
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Alice", 10);
    scores.insert("Bob", 20);
    println!("Scores: {:?}", scores);
    // Accessing values
    /*Access Values
    You can use the .get() method to access a value in a HashMap by its key: */
    let mut capitalCities = HashMap::new();

    capitalCities.insert("England", "London");
    capitalCities.insert("Germany", "Berlin");
    capitalCities.insert("Norway", "Oslo");

    if let Some(city) = capitalCities.get("England") {
        println!("The capital of England is {}.", city);
    } else {
        println!("England is not in the map.");
    }
    if let Some(score) = scores.get("Alice") {
        println!("Alice's score: {}", score);
    } else {
        println!("Alice not found");
    }

    /*Update Values
    If you insert a new value using a key that already exists, the old value is replaced with the new one: */
    capitalCities.insert("England", "Edinburgh");
    println!(
            "Updated capital of England: {:?}",
            capitalCities.get("England")

            let mut capitalCities = HashMap::new();

    capitalCities.insert("England", "London");
    capitalCities.insert("England", "Berlin");

    println!("{:?}", capitalCities);
        );

    /*Remove Values
    To remove a key from a HashMap, use the .remove() method: */
    let mut solana_active_accounts = HashMap::new();
    solana_active_accounts.insert("Alice", 100);
    solana_active_accounts.insert("Bob", 200);
    println!("Before removal: {:?}", solana_active_accounts);
    /*Loop Through a HashMap
    You can use a for loop to go through all key/value pairs: */

    let mut capitalCities = HashMap::new();

    // Add keys and values (Country, City)
    capitalCities.insert("England", "London");
    capitalCities.insert("Germany", "Berlin");
    capitalCities.insert("Norway", "Oslo");

    // Loop through the HashMap
    for (country, city) in &capitalCities {
        println!("The capital of {} is {}.", country, city);
    }
    solana_active_accounts.remove("Alice");
    println!("After removal: {:?}", solana_active_accounts);
    /*tuples methods */
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    /*Why Use HashMaps?
    To store data by key
    To quickly look up values
    To group related data (like names and scores)
    Note: HashMaps require keys to be unique. Inserting the same key again will overwrite the old value. */
}
