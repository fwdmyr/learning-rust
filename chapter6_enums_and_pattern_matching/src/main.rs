// Basic enum type.
enum Color {
    Red,
    Green,
    Blue,
}

// The enum variants can have arbitrary data associated with them.
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },    // Anonymous struct
    Write(String),              // String type
    ChangeColor(i32, i32, i32), // Three i32 values
}

// Enums can have methods and associated functions that operate on them.
impl Message {
    fn call(&self) {
        // Function body goes here...
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // More complex code arm should be scoped.
        // When match is found, state binds to the coin's internal state value.
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        }
    }
}

// Note: Popular Rust idiom to match an enum, bind a variable to the data inside and execute code
// based on that.
fn increment(value: Option<i32>) -> Option<i32> {
    match value {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let c = Color::Red;

    let msg = Message::Write(String::from("Payload"));
    msg.call();

    // Option<T> is a generic enum for all types T comparable to std::optional<T>.
    // It is implemented like this enum Option<T> {Some(T), None,} and gets brought into scope by
    // default.
    let some_number = Some(5);
    let no_number: Option<i32> = None;

    // The match operator compares a value against a series of patterns and then executes code
    // based on which pattern matches. Patterns can be made up of literal values, variable names,
    // wildcards, enums, and many more.
    let alaska_quarter = Coin::Quarter(UsState::Alaska);
    println!("Value of a quarter is {}", value_in_cents(alaska_quarter));

    let five = Some(5);
    let six = increment(five);
    let none = increment(None);

    let some_value: u32 = 5;
    // Match statements need to be exhaustive.
    match some_value {
        0 => println!("Zero"),
        1 => println!("One"),
        // Match-all placeholder with unit value arm. Would not compile without this.
        _ => (),
    }

    // Use if let to ignore all but a single pattern in a less verbose way.
    // Use like this: if let <PATTERN> = <EXPRESSION> {<IF ARM>} else {<ELSE ARM>}
    let alabama_quarter = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = alabama_quarter {
        println!("Quarter from {:?}", state);
    } else {
        println!("No quarter");
    }
}
