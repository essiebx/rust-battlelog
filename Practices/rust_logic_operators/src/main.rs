/*
Logical Operators
Logical operators are used to work with boolean values:

Operator	Name	   Description
&&	        AND	      true if both values are true
||	        OR	      true if at least one is true
!	        NOT  	inverts the boolean value
*/
fn main() {
    let a = true;
    let b = false;

    // AND operator
    let and_result = a && b; // false
    println!("AND result: {}", and_result);

    // OR operator
    let or_result = a || b; // true
    println!("OR result: {}", or_result);

    // NOT operator
    let not_result = !a; // false
    println!("NOT result: {}", not_result);

    let logged_in = true;
    let is_admin = false;

    println!("Is regular user: {}", logged_in && !is_admin);
    println!("Has any access: {}", logged_in || is_admin);
    println!("Not logged in: {}", !logged_in);
}
