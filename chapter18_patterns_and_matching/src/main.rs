// Function parameters also are a pattern.
fn print_coordinates(&(x, y): &(u32, u32)) {
    println!("Coordinates: [{}, {}]", x, y);
}

// Function parameters can be ignored.
fn ignore_first_argument(_: i32, x: i32) {
    println!("First argument doesnt bind to anything and second is {}", x);
}

struct Point {
    x: u32,
    y: u32,
}

struct Triple {
    x: u32,
    y: u32,
    z: u32,
}

enum Color {
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
}

enum Message {
    Quit,
    Move { x: u32, y: u32 },
    Write(String),
    ChangeColor(Color),
}

enum Payload {
    Data { id: u32 },
}

fn destructure_message_enum(msg: Message) {
    match msg {
        Message::Quit => println!("No data to destructure."),
        Message::Move { x, y } => println!("{}, {}", x, y),
        Message::Write(text) => println!("{}", text),
        // We can also destructure nested items.
        Message::ChangeColor(Color::RGB(r, g, b)) => println!("{}, {}, {}", r, g, b),
        Message::ChangeColor(Color::HSV(h, s, v)) => println!("{}, {}, {}", h, s, v),
    }
}

fn main() {
    // Even this simple let statement uses patterns. It matches the pattern (x) to the right-hand
    // side of the equation. As the pattern x matches anything, this always produces a match.
    let x = 5;

    // This statement ...
    if let y = x {
        println!("Matched 5.");
    } else {
        println!("No match.");
    }
    // ... is the same as ...
    match x {
        5 => println!("Matched 5."),
        _ => println!("No match."),
    }

    let mut stack = vec![1, 2, 3];

    // Destructuring in for-loops.
    for (index, value) in stack.iter().enumerate() {
        println!("Value {} at index {}.", value, index);
    }

    // While let conditional loops.
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let point = (1, 2);
    print_coordinates(&point);

    // Irrefutable patterns are those that will match anything.
    // For-loops, functions and let statements require irrefutable patterns.
    let irrefutable_pattern = 5;

    // Refutable patterns are those that might fail to match.
    let refutable_pattern_closure = |option: Option<u32>| match option {
        Some(value) => println!("Some({})", value),
        None => println!("None"),
    };

    // It is possible to match multiple patterns in a single arm.
    let multiple_pattern_closure = |x| match x {
        // Useable for all types and enums.
        1 | 2 => println!("One or two."),
        // Useable for numeric and char types.
        3..=10 => println!("Three to ten."),
        _ => println!("Something else."),
    };

    // Destructuring structs.
    let p = Point { x: 0, y: 2 };
    // Equivalent to: let x = p.x; let y = p.y;
    let Point { x, y } = p;
    assert_eq!(x, 0);
    assert_eq!(y, 2);
    match p {
        Point { x: 0, y } => println!("Point lies on the y-axis."),
        Point { x, y } => println!("Point does not lie on the y-axis."),
    }

    // We can also ignore parts of a value with nested underscores.
    let partial_ignore_closure = |option: Option<u32>| match option {
        Some(_) => println!("Something"),
        _ => println!("Nothing"),
    };
    // Or with two dots.
    let tr = Triple { x: 1, y: 2, z: 3 };
    match tr {
        Triple { x, .. } => println!("Triple's x is {}.", x),
    }
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => println!("First is {}, last is {}.", first, last),
    }

    // Match guards are additional if conditions specified after the pattern that must also match.
    // The enhanced pattern is evaluated as <pattern> && <match_guard>.
    let match_guarded_closure = |option: Option<u32>| match option {
        Some(value) if value < 5 => println!("Less than five."),
        Some(_) => println!("Greater equal than five."),
        None => (),
    };

    // We can use @ bindings to create variables that hold values at the same time we are testing
    // that value for a pattern match.
    let data = Payload::Data { id: 5 };

    match data {
        Payload::Data {
            id: id_variable @ 1..=10,
        } => println!("Found id {} in range [1,10]", id_variable),
        _ => (),
    }
}
