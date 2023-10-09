pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

fn private_implementation_detail() -> bool {
    true
}

pub struct Guess {
    val: u32,
}

impl Guess {
    pub fn new(val: u32) -> Guess {
        if val < 1 {
            panic!("Guess less than 1, was {}.", val);
        }
        if val > 100 {
            panic!("Guess larger than 100, was {}.", val);
        }
        Guess { val }
    }
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[cfg(test)]
// Unit tests go here...
mod tests {
    use super::*;

    #[test]
    // It is possible to test private functions in unit tests.
    fn test_implementation_detail() {
        assert!(private_implementation_detail());
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // Do some heavy computations.
        assert!(true);
    }

    #[test]
    // Tests that return a Result enum cannot be annotated with should_panic.
    // Tests that return a Result are convenient as we can directly use operator?.
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            // Test passed.
            Ok(())
        } else {
            // Failure case.
            Err(String::from("two plus two does not equal 4"))
        }
    }

    #[test]
    // Mark death test with should_panic and optional expected error message.
    #[should_panic(expected = "Guess larger than 100, was 101.")]
    fn guess_greater_than_100() {
        let val = 101;
        Guess::new(val);
    }

    #[test]
    fn greeting_contains_name() {
        let name = "Felix";
        let result = greeting(name);
        // Assert macro takes optional arguments that define a custom error message.
        assert!(
            result.contains(name),
            "Greeting did not contain name, value was {}",
            name
        );
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
