use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    let f = File::open("test.txt");

    let f = match f {
        Ok(file) => file,
        // Match on value of error enum.
        Err(error) => match error.kind() {
            // Try to create a new file.
            ErrorKind::NotFound => match File::create("test.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem creating the file: {:?}", other_error),
        },
    };

    // A more concise and idiomatic way to achieve the same thing can be implemented like so:
    let f = File::open("test.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("test.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Unwrap and expect can be used instead of the verbose match syntax.

    // Unwrap returns the value inside Ok or panics on Err.
    let f = File::open("test.txt").unwrap();
    // Expect is similar to unwrap but takes an argument that will be used as the panic error message.
    let f = File::open("test.txt").expect("Failed to open test.txt");

    // Propagating errors.
    let res = read_username_from_file("test.txt");

    // Prefer Result return types instead of panicking as the default.
    // When failure is expected atleast some of the time, returning a Result is more appropriate.
    // When a contract is violated or the program unexpectedly enters a bad state, panicking is
    // appropriate.

    // The type system can be used to enforce contracts and validate inputs by creating custom
    // types.
    // Contract fulfilled. Ok.
    let g1 = Guess::new(5);
    // Contract violated. Panics.
    let g2 = Guess::new(101);
}

fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
    // The ? operator can be used on Result enums. If Result is Ok, it returns the value inside Ok.
    // Else, it returns the Err from the caller early. The ? operator can only be applied to error
    // types that implement the From trait. The From trait defines a conversion between the current
    // error type and the error type specified in the return type of the calling function.
    // Here the associated function open and method read_to_string both return result types.
    let mut f = File::open(&filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

pub struct Guess {
    // Implicitly declared private.
    val: i32,
}

impl Guess {
    // Only way to construct a Guess.
    pub fn new(val: i32) -> Guess {
        // Contract checking.
        if val < 1 || val > 100 {
            panic!("Guess val must be between 1 and 100, got {}", val);
        }
        Guess { val }
    }

    // Required to access private field.
    pub fn value(&self) -> i32 {
        self.val
    }
}
