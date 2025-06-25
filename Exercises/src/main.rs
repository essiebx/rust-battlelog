fn main() {
    /* ✅ Challenge 1: Integer Arithmetic
       Simple addition with immutable integers
    */
    let a = 7;
    let b = 5;
    let c = a + b;
    assert_eq!(c, 12); // 7 + 5 = 12

    /* ✅ Challenge 2: Boolean AND Logic
       Using `&&` and `!` to evaluate a logical expression
    */
    let is_rust_fun = true;
    let is_boring = false;
    assert_eq!(is_rust_fun && !is_boring, true); // true && true

    /* ✅ Challenge 3: Shadowing and String Length
       Shadowing a variable and changing its type
    */
    let word = "Rustacean";
    let word = word.len(); // `word` is now a number: 9
    assert_eq!(word, 9); // "Rustacean" has 9 letters

    /* ✅ Challenge 4: Floating Point Division
       Division between two f32 values
    */
    let a: f32 = 20.0;
    let b: f32 = 4.0;
    let result = a / b;
    assert_eq!(result, 5.0); // 20.0 / 4.0

    /* ✅ Challenge 5: Unicode Character Size
       A Rust `char` always occupies 4 bytes
    */
    let letter = '🦀';
    assert_eq!(std::mem::size_of_val(&letter), 4); // fixed 4-byte char

    /* ✅ Challenge 6: String Composition
       Using `format!()` to safely combine &str values
    */
    let greeting = "hey";
    let name = "esther";
    let full = format!("{} {}", name, greeting);
    assert_eq!(full, "esther hey");
    println!("{}", full); // esther hey

    /* 🆕 Challenge 7: Boolean OR Logic
       Using `||` (logical OR)
    */
    let a = false;
    let b = true;
    let result = a || b; // false OR true = true
    assert_eq!(result, true);

    /* 🆕 Challenge 8: Integer Overflow Awareness
       Get the max i8 value and add 1 using i16
       to prevent overflow in smaller type
    */
    let max = std::i8::MAX; // i8 max = 127
    let new = max as i16 + 1;
    assert_eq!(new, 128); // 127 + 1

    /* 🆕 Challenge 9: Match Expression
       Rust's match is like switch in other languages
    */
    let num = 2;
    let desc = match num {
        1 => "one",
        2 => "two",
        _ => "other",
    };
    assert_eq!(desc, "two");

    /* 🆕 Challenge 10: Tuple Destructuring
       Splitting tuple elements into variables
    */
    let coords = (3, 7);
    let (x, y) = coords;
    assert_eq!(x + y, 10); // 3 + 7

    //  Concepts to Learn from Variables in Rust

    // 1. **Shadowing**: Re-declaring a variable with the same name allows changing its type
    let shadowed = 5;
    let shadowed = shadowed + 1; // now shadowed is 6
    assert_eq!(shadowed, 6); //assert helps ensure correctness byasserting that the value is as expected
    // 2. **Immutability**: Variables are immutable by default, meaning their value cannot change after assignment
    let immutable = 10;
    // immutable = 20; // This would cause a compile error
    assert_eq!(immutable, 10); // still 10
    // by default you dont have to use  immutability keyword, variables are immutable by default
    let x = 5;
    assert_eq!(x, 5); // x is still 5
    // 3. **Type Inference**: Rust can often infer the type of a variable based on its value
    let inferred = 3.14; // Rust infers this as f64
    assert_eq!(inferred, 3.14); // inferred is 3.14
    // 4. **Type Annotations**: You can explicitly specify a variable's type
    let annotated: i32 = 42; // explicitly typed as i32
    assert_eq!(annotated, 42); // annotated is 42
    // 5. **Constants**: Use `const` for values that should never change
    // Constants are always immutable and must have a type annotation
    const PI: f64 = 3.14159; // constant value
    const max_value: i32 = 20;
    assert_eq!(PI, 3.14159); // PI is 3.141
    const MAX_POINTS: u32 = 100_000; //notice the use of underscore for readability
    assert_eq!(MAX_POINTS, 100_000); // MAX_POINTS is 100,000
    // 6. **Mutable Variables**: Use `mut` to allow a variable's value to change
    let mut number_is_mutable = 10; // mutable variable
    number_is_mutable += 5; // now it's 15

    // Scoping + Shadowing in Blocks
    // Test variable visibility and shadowing inside scopes:
    {
        let inner = 20; // inner scope variable
        assert_eq!(inner, 20); // inner is 20
        let number_is_mutable = 15; // shadowing the outer variable
        assert_eq!(number_is_mutable, 15); // inner shadowed variable is
        // 15, not the outer one
        // The outer `number_is_mutable` is still 10 here
    }
    // The outer `number_is_mutable` is still 15 here
    let outer = 10; // outer scope variable
    {
        // Inner scope
        let inner = 5; // shadowing outer variable
        assert_eq!(inner, 5); // inner is 5
        assert_eq!(outer, 10); // outer is still 10
    }
    // lesson learned: Variables in Rust are immutable by default, and you can use shadowing to change their type or value within the same name scope. Scopes allow you to create variables that are only visible within a specific block, and shadowing lets you redefine a variable with the same name in an inner scope.
    // can we access the inner variable outside its scope?
    // No, the inner variable is not accessible outside its block
    // println!("{}", inner); // This would cause a compile error
    // The outer variable is still accessible

    // structs
    // 7. **Structs**: Custom data types that group related data

    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 10, y: 20 }; //unpacked struct initialization
    assert_eq!(p.x, 10); // p.x is 10
    struct current_balance {
        amount: f64,
        currency: String,
    }
    let balance = current_balance {
        amount: 100.50,
        currency: String::from("USD"),
    };
    assert_eq!(balance.amount, 100.50); // balance amount is 100
    struct wallet_type {
        balance: f64,
        currency: String,
        name: String,
        type_of_wallet: String,
    }
    let get_wallet_info = wallet_type {
        balance: 250.75,
        currency: String::from("EUR"), // using String::from to create a String
        name: String::from("My Wallet"),
        type_of_wallet: String::from("Digital"),
    };
    //  imports an input out library for such
    use std::io;
    {
        println("guess the number");
        println!("also include your name");
        // variable to store user input, we lso need to import or create  the data type since its not in the st::io library
        let mut guess = String::new();
        let name = String::new();
    }
}
