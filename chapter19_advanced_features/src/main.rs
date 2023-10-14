// Associated types as placeholders.
pub trait Iter {
    // Associated placeholder type. Implementors of this trait will specify the concrete type for
    // Item at a later stage.
    // If we would use generics instead, we would have to annotate the type to indicate which
    // implementation to use as generics would allow us to implement the same trait multiple times.
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

struct Meters(i32);

struct Millimeters(i32);

// Trait that uses a default type parameter. Mirrors std::ops::Add.
trait MyAdd<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

// Using the default, i.e. impl MyAdd<Point> for Point.
impl MyAdd for Point {
    type Output = Point;

    // Matches the declaration where RHS = Self.
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Overriding the default. Used to provide customization for specific cases.
impl MyAdd<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (1000 * other.0))
    }
}

use std::ops::Add;

// All operators in std::ops can be overloaded.
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot for Human");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard for Human");
    }
}

impl Human {
    fn fly(&self) {
        println!("Human");
    }
}

trait Animal {
    fn name();
}

struct Dog;

impl Animal for Dog {
    fn name() {
        println!("Animal for Dog");
    }
}

impl Dog {
    fn name() {
        println!("Dog");
    }
}

// Using Supertraits to require one trait's functionality within another trait.
trait Supertrait {
    fn super_method(&self);
}

// Require Supertrait.
trait Othertrait: Supertrait {
    fn other_method(&self) {
        println!("Other method.");
        self.super_method();
    }
}

// Implement missing definition.
impl Supertrait for Point {
    fn super_method(&self) {
        println!("Super method!");
    }
}

impl Othertrait for Point {}

// Use the newtype pattern to get around the orphan rule, i.e. implementing external traits on
// external types.
// Downside: We introduce new wrapper type that does not implement all the traits of the type it
// wraps. We can fix this by either implementing Deref that returns the wrapped type or implement
// all traits that the wrapped type implements.

use std::fmt;

// Thin wrapper local to our crate around external type.
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Get the first element of the wrapper tuple. This is elided by the compiler and we get a
        // zero overhead abstraction.
        write!(f, "[{}]", self.0.join(", "))
    }
}

// The never type (or empty type) never returns and can be coerced into ANY type.
// This is how we can have match arms that either return a value or panic.
fn infinite_loop() -> ! {
    loop {}
}

// Function pointers
fn add_one(x: i32) -> i32 {
    x + 1
}

// The fn type is a function pointer. Function pointers implement all three of the closure traits
// (Fn, FnMut, FnOnce) so you can always pass a function pointer to a function that accepts a
// closure. Therefore it is best to write functions that should accept a callable using a generic
// type and one of the closure traits so the functions can accept either functions or closures.
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// Return closure by wrapping it in a smart ptr.
fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let lhs = Point { x: 5, y: 6 };
    let rhs = Point { x: 2, y: 4 };

    // Operator overloading in action.
    println!("{:?} + {:?} = {:?}", lhs, rhs, lhs.clone() + rhs.clone());

    // Using fully-qualified syntax for disambiguation:
    // <Type as Trait>::function(receiver_if_method, args...)
    // We can omit parts of this syntax that the compiler can deduce by itself.
    let person = Human;
    // Human
    person.fly();
    // Pilot for Human
    Pilot::fly(&person);
    // Wizard for Human
    Wizard::fly(&person);
    let dog = Dog;
    // Dog
    Dog::name();
    // Animal for Dog
    <Dog as Animal>::name();

    // Supertraits.
    let p = Point { x: 1, y: 2 };
    p.other_method();

    // Rust also has aliases.
    type SignedInteger = i32;
    type IOResult<T> = Result<T, std::io::Error>;

    // Working with function pointers.
    let list_of_numbers = vec![1, 2, 3];
    // This style using a closure is equivalent ...
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    // .. to this style that uses a function pointer ...
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    // We can also use this neat syntax that uses the initialization syntax of tuple structs. The
    // initialization is actually implemented as function calls that return the instance
    // constructed from its arguments. So we can write something like this ...
    let list_of_options: Vec<Option<u32>> = (1..10).map(Option::Some).collect();
    // ... instead of this ...
    let list_of_options: Vec<Option<u32>> = (1..10).map(|i| Option::Some(i)).collect();

    // We can return closures from functions only by wrapping them in a smart ptr.
    let my_closure = return_closure();
}
