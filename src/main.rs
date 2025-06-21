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
}
