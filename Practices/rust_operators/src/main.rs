/*Operators are used to perform operations on values and variables.

Rust supports many common operators, like:

Arithmetic Operators
Assignment Operators
Comparison Operators
Logical Operators
Arithmetic Operators
Arithmetic operators are used to do basic math:

Operator	   Name        	Example      	Result
+	           Addition	     5 + 3	         8
-	          Subtraction	   5 - 3	      2
*	          Multiplication	5 * 3	      15
/	          Division	         10 / 2	      5
%	         Remainder (modulus)  	10 % 3	   1
*/

fn main() {
    let add: i32 = 5 + 3; // Addition
    let sub = 10 - 4;
    let mul = 6 * 7; // Multiplication
    let div = 20 / 4; // Division
    let rem = 10 % 3; // Remainder (modulus)
    println!("Addition: {}", add);
    println!("Subtraction: {}", sub);
    println!("Multiplication: {}", mul);
    println!("Division: {}", div);
    println!("Remainder: {}", rem);

    // Assignment Operators
    /*Assignment Operators
    Assignment operators are used to assign and update values:

    Operator	Example     	Same As
    =	        x = 5	         Assign 5 to x
    +=	        x += 3	               x = x + 3
    -=	      x -= 2	               x = x - 2
    *=    	x *= 4               	x = x * 4
    /=	     x /= 2	                x = x / 2
    %=	   x %= 2	               x = x % 2

    */

    let mut x = 10;
    println!("Start: {}", x);

    x += 5;
    println!("After += 5: {}", x);

    x -= 2;
    println!("After -= 2: {}", x);

    x *= 2;
    println!("After *= 2: {}", x);

    x /= 3;
    println!("After /= 3: {}", x);

    x %= 4;
    println!("After %= 4: {}", x);

    // Comparison Operators
    /*
        Comparison operators compare values and return true or false:
        Operator	      Meaning	   Example
    ==	                 Equal to 	   5 == 5 is true
    !=	                 Not equal to	5 != 3 is true
    >	                 Greater than	7 > 3 is true
    <	                 Less than	2 < 5 is true
    >=	                 Greater than or equal to	5 >= 5 is true
    <=	                  Less than or equal to	3 <= 4 is true
         */
    let a = 5;
    let b = 3;
    println!("Is a equal to b? {}", a == b); // false
    let ba = 5;
    let bu = 10;

    println!("5 == 10: {}", a == b);
    println!("5 != 10: {}", a != b);
    println!("5 < 10: {}", a < b);
    println!("5 >= 10: {}", a >= b);
}
