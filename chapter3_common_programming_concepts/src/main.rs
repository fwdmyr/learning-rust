fn main() {
    // Primitive types

    let mut lvalue = 1;
    lvalue = 2;

    let const_lvalue = "this is a string";
    // Create numeric variable that shadows the String variable.
    let const_lvalue = const_lvalue.len();

    // Type annotation required in case of ambiguities
    let num: u32 = "69".parse().expect("NaN");

    // Const values always require a type annotation.
    const CONSTEXPR_LVALUE: u32 = 10_000;

    // Compound types

    // Like std::tuple
    let tup: (i32, f64, u8) = (-7, 3.4, 16);
    let tup = (-7, 3.4, 16);
    let (x, y, z) = tup;

    println!("The value of y is {} which is also {}", y, tup.1);

    // Like std::array
    let arr = [1, 2, 3, 4, 5];
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr = [0; 5]; // equal to let arr = [0, 0, 0, 0, 0]

    // Note: Out-of-bounds access results in runtime error.
    println!("The value of the first element is {}", arr[0]);

    // Statements and expressions

    // Statements do not return a value
    let statement = 5;

    // Expressions return something
    // The scope itself is an expression. Think IIFE.
    let expression = {
        let x = 1; // Statement
        x + 1 // Expressions do not end with a semicolon!
    };

    // Functions

    print_values(1, 2);

    expression_definition(true);

    println!("{}, {}", five(), five_or_six(false));

    // Loops

    // Infinite loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break 2 * counter;
        }
    };

    println!("Exited infinite loop. Result is {}", result);

    // Conditional loop
    counter = 3;
    while counter != 0 {
        counter -= 1;
    }

    // For loop
    // Range-based
    for element in arr.iter() {
        println!("Element is {}", element);
    }
    // Prints integers in [0, 5)
    for number in 0..5 {
        println!("{}", number);
    }
}

// Functions with return values implicitly return the last expression.
fn five() -> i32 {
    5
}

// Returning early is possible by using the return keyword.
fn five_or_six(b: bool) -> i32 {
    if b {
        return 5;
    }
    6
}

fn expression_definition(b: bool) {
    // Let statement that uses an expression to define a value.
    let x = if b { 1 } else { 2 };
    println!("x = {}", x);
}

// Functions require type annotation and may be declared below the call-site in a translation unit.
fn print_values(x: i32, y: i32) {
    println!("x = {}, y = {}", x, y);
}
