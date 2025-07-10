/*Arrays
Arrays are used to store multiple values in a single variable, instead of declaring separate variables for each value.

Create an Array
You can create an array using square brackets [ ], and separate the values with commas.

Note: Make sure all values are of the same data type (integers in the example below): */

fn main() {
    let numbers = [1, 2, 3, 4, 5]; //This creates an array with five integers
    /*Access Array Elements
To access an array element, refer to its index number.

Array indexes start with 0: [0] is the first element. [1] is the second element, etc.

This statement accesses the value of the first element [0] in numbers */
let numbers = [1, 2, 3, 4, 5];
println!("The first number is: {}", numbers[0]);

    //You can also loop through an array using a for loop
    for number in numbers.iter() {
        println!("Number: {}", number);
    }

    //You can also create an array with a specific size and fill it with the same value
    let zeros = [0; 5]; //This creates an array of five zeros
    println!("Zeros: {:?}", zeros);

    //You can also create an array of strings
    let fruits = ["apple", "banana", "cherry"];
    println!("First fruit: {}", fruits[0]);

    //You can also create an array of mixed types using tuples
    let mixed = [(1, "one"), (2, "two"), (3, "three")];
    println!("First mixed element: {:?}", mixed[0]);


    /*Change Array Values
To change the value of a specified element, refer to the index number and assign a new value.

Remember to make the array mutable (using the mut keyword): */
let mut numbers = [1, 2, 3, 4, 5];
numbers[0] = 10;
println!("The new first number is: {}", numbers[0]);
/*Array Length
You can get the number of elements in an array using the .len() method: */
let length = numbers.len();
println!("The length of the array is: {}", length);
let numbers = [1, 2, 3, 4, 5];
println!("This array has {} elements.", numbers.len());

    //You can also check if an array is empty using the .is_empty() method
    let is_empty = numbers.is_empty();
    println!("Is the array empty? {}", is_empty);

    //You can also find the maximum and minimum values in an array using the .iter().max() and .iter().min() methods
    let max_value = numbers.iter().max();
    let min_value = numbers.iter().min();
    println!("Max value: {:?}", max_value);
    println!("Min value: {:?}", min_value);

    //You can also sort an array using the .sort() method
    let mut unsorted_numbers = [5, 3, 1, 4, 2];
    unsorted_numbers.sort();
    println!("Sorted numbers: {:?}", unsorted_numbers);

    //You can also reverse an array using the .reverse() method
    let mut reversed_numbers = [1, 2, 3, 4,
    5];
    reversed_numbers.reverse();
    println!("Reversed numbers: {:?}", reversed_numbers);
    //You can also find the index of a specific value using the .iter().position() method
    let index = numbers.iter().position(|&x| x == 3);
    match index {
        Some(i) => println!("The index of 3 is: {}", i),
        None => println!("3 is not in the array"),
    }
    /*Print the Entire Array
Note: When printing the whole array, you must use {:?} inside println!: */
    println!("The entire array is: {:?}", numbers);

    //You can also print the entire array using a for loop
    for (index, &value) in numbers.iter().enumerate() {
        println!("Index {}: {}", index, value);
    }

    //You can also use the .join() method to join the elements of an array into a string
    let joined_numbers = numbers.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(", ");
    println!("Joined numbers: {}", joined_numbers);

    /*array methods:
    - .iter() - Returns an iterator over the elements of the array.
    - .len() - Returns the number of elements in the array.
    - .is_empty() - Returns true if the array is empty, false otherwise.
    - .max() - Returns the maximum value in the array.
    - .min() - Returns the minimum value in the array.
    - .sort() - Sorts the elements of the array in ascending order
    - .reverse() - Reverses the order of the elements in the array.
    - .position() - Returns the index of the first occurrence of a specified value in the
    array, or None if the value is not found.
    - .join() - Joins the elements of the array into a string, using a
    specified separator.
    - .map() - Applies a function to each element of the array and returns a new array with the results.
    - .collect() - Collects the elements of an iterator into a new collection, such
    as a vector or a string.
    - .enumerate() - Returns an iterator that yields pairs of the index and the value
    of each element in the array.
    - .to_string() - Converts a value to a string representation.
    - .push() - Adds an element to the end of a vector.
    - .pop() - Removes and returns the last element of a vector.
    - .insert() - Inserts an element at a specified index in a vector.
    - .remove() - Removes and returns the element at a specified index in a vector.
    - .contains() - Returns true if the vector contains a specified value, false otherwise.
    - .clear() - Removes all elements 
    
    
    
    
     */

/*Fixed Size (Arrays) vs. Dynamic Size (Vectors)
You will often hear the terms fixed size and dynamic size when talking about arrays in Rust.

This is because arrays in Rust have a fixed size, meaning you cannot add or remove elements after the array is created: */

}
