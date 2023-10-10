// Generic data types

// This function finds the largest element in a slice of a contiguous sequence of type T.
use std::cmp::PartialOrd;
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    // Destructuring by pattern matching. Usually, we loop over &T. But by specifying &element,
    // element is actually deduced to be T only. Think of template parameter type deduction.
    for &element in list.iter() {
        if element > largest {
            largest = element;
        }
    }
    largest
}

fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for element in list.iter() {
        if *element > *largest {
            largest = element;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x_ref(&self) -> &T {
        &self.x
    }
}

impl<T: Copy> Point<T> {
    fn x_copy(&self) -> T {
        self.x
    }
}

impl Point<bool> {
    // Overloading for method x() where T = bool is not possible.
    fn x(&self) -> &bool {
        &self.x
    }
}

struct Pair<T, U> {
    first: T,
    second: U,
}

// Generic parameters <T, U> relevant for impl scope.
impl<T: Clone, U> Pair<T, U> {
    // Generic parameters <V, W> relevant for function scope.
    fn mix<V, W: Clone>(&self, other: &Pair<V, W>) -> Pair<T, W> {
        Pair {
            first: self.first.clone(),
            second: other.second.clone(),
        }
    }
}

enum RecreatedResult<T, E> {
    Ok(T),
    Err(E),
}

// Traits

// Defines the interface that types that implement this trait must implement.
// These are like pure virtual functions in interfaces that concrete types have to implement when
// inheriting from the abstract interface.
// Needs to be pub so external structs may implement this trait.
pub trait Summary {
    // Pure virtual.
    fn summarize(&self) -> String;
    fn shoutout(&self) -> String;
    // Default implementation. These can call all other methods in the same trait.
    fn advertise(&self) -> String {
        String::from(format!("Read more from {}", self.shoutout()))
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
}

// Concrete implementation of the trait.
impl Summary for NewsArticle {
    // Override pure virtual.
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
    fn shoutout(&self) -> String {
        self.author.clone()
    }
    // Ignore advertise to get default behavior.
}

// This is syntactic sugar ...
pub fn notify1(item: impl Summary) {
    println!("New publication from {}", item.shoutout());
}
// ... for this.
pub fn notify2<T: Summary>(item: T) {
    println!("New publication from {}", item.shoutout());
}
// The more verbose syntax is more powerful and lets us, for example, enforce that multiple
// arguments that implement the same trait are of same type.
// This would not be possible with the syntactic sugar version.
pub fn notify_trait_bound<T: Summary>(item: T, another_item: T) {
    println!("New publication from {}", item.shoutout());
    println!("New publication from {}", another_item.shoutout());
}

// Multiple trait bounds are possible too
use std::fmt::{Debug, Display};
pub fn multiple1(item: impl Summary + Display) {}
pub fn multiple2<T: Summary + Display>(item: T) {}

// Where clauses allow us to write more concise function signatures in the presence of many traits.
pub fn many_traits<T, U>(first: T, second: U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

pub trait ConditionalTrait {
    fn conditional(&self) {
        println!("Conditional...");
    }
}

// Conditionally implement a trait for any type that implements another trait.
// Here: Implement ConditionalTrait for any T that also implements Summary.
impl<T: Summary> ConditionalTrait for T {}

// Return types that implement traits
fn returns_summarizable() -> impl Summary {
    NewsArticle {
        headline: String::from("Aliens"),
        author: String::from("Pickle Rick"),
        content: String::from("Lorem ipsum"),
    }
}

// Lifetimes

// Lifetime annotations in function signatures.
// For some lifetime 'a, both parameters as well as the return value will live at least as long as
// lifetime 'a. This lets the borrow checker reject any values that violate these constraints.
// When passing concrete references as arguments, the generic lifetime 'a will get the shorter
// lifetime of both of the arguments. This ensures that the return value will have a lifetime that
// is atleast as long as those of the function parameters and will therefore prevent dangling
// references.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    let val = x.len() > y.len();
    match val {
        true => x,
        false => y,
    }
}

// When returning a reference from a function, the lifetime parameter for the return type needs to
// match the lifetime parameter for one of the parameters.

// Lifetime annotations in struct definitions.
// An instance of ReferenceHolder cannot outlive the reference it holds in its reference field.
struct ReferenceHolder<'a> {
    reference: &'a str,
}

// The compiler may elide lifetimes when he is able to deterministically determine all input and
// output lifetimes based on these deduction rules:
// 1) Each parameter that is a reference gets its own lifetime parameter.
// 2) If there is exactly one input lifetime parameter, its lifetime is assigned to all output
//    lifetime parameters.
// 3) If a reference parameter is &self or &mut self, its lifetime is assigned to all output
//    lifetime parameters.

// Rule (3) frees us from annotation methods that operate on structs in many cases:
impl<'a> ReferenceHolder<'a> {
    fn rule_one(&self) -> i32 {
        5
    }
    fn rule_three(&self, s: &str) -> &str {
        println!("{}", s);
        self.reference
    }
}

// Furthermore, these rules free us from annotation functions with this type of signature:
fn noop(s: &str) -> &str {
    s
}

// Example function that uses generics, traits and lifetimes:
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    let val = x.len() > y.len();
    match val {
        true => x,
        false => y,
    }
}

fn main() {
    let article = returns_summarizable();
    article.conditional();
    println!("{}, {}", article.summarize(), article.advertise());

    let x = "abc";
    let y = String::from("defgh");
    println!("{}", longest(x, &y));

    let novel = String::from("A first sentence. A second sentence.");
    // Holds a reference to the String in novel.
    let first_sentence = novel.split('.').next().unwrap();
    // Stores the reference and - critically - lives at least as long as the referenced value.
    let reference_holder = ReferenceHolder {
        reference: first_sentence,
    };

    // Static lifetimes denote the entire duration of a program. String slices have an implicit
    // static lifetime because the string literal is stored directly in the binary.
    // Therefore, these two expressions are equivalent:
    let s = "abc";
    let s: &'static str = "abc";
}
