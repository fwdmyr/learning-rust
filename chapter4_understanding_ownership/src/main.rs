fn main() {
    let lit = "Hello";

    {
        // Allocate String object on the heap.
        // All objects on the heap are RAII types.
        let mut s = String::from(lit);

        println!("{}", s);

        s.push_str(", world!");

        println!("{}", s);
        // String object goes out of scope here.
        // Rust automatically cleans it up.
    }

    // Per default, Rust moves objects that hold heap resources when assigning them to a new
    // variable. All copies at runtime are inexpensive shallow copies.
    // These types implement the Drop trait.
    let s1 = String::from("hello");
    // Performs a move and invalidates s1. Using s1 after this point results in a compiler error.
    let s2 = s1;
    // Deep copies can be created via cloning.
    let s3 = s2.clone();

    // Moves s3 into the string parameter of the function and invalidates s3.
    takes_ownership(s3);

    // Moves the function's return value into s4.
    let s4 = gives_ownership();

    // Moves s4 into the function parameter and moves the function's return value into s5.
    let s5 = takes_and_gives_back(s4);

    // Passing s5 by reference does not perform a move. The argument s5 stays valid after the
    // function returns.
    // The function parameter only borrows the argument but never assumes ownership.
    let l5 = compute_length(&s5);

    // Pass by mutable reference if the argument shall be modified in the scope of the function.
    // This requires the argument and function parameter to be mutable.
    // NOTE: Only a single mutable reference to a particular piece of data can exist in a
    // particular scope at any given point in time! This prevents data races at compile time and
    // allows Rust to forego implementing a synchronization mechanism for multi-threaded
    // applications.
    let mut s6 = s5;
    append(&mut s6);

    // Define a mutable reference.
    {
        let mr1 = &mut s6;
    }
    // Previous mutable reference went out of scope. This is fine.
    let mr2 = &mut s6;

    // This is fine. However using the mutable reference mr2 while an immutable reference to the
    // same piece of data exists, results in a compiler error.
    let r1 = &s6;
    let r2 = &s6;
    // This would not compile: println!("{}", mr2);

    // The ownership and borrowing mechanisms detect dangling references at compile-time and result
    // in an error.

    // Stack-only types can be copied without invalidating the rhs variable.
    // These types implement the Copy trait and neither it nor any of its parts must implement the Drop trait.
    let i1 = 5;
    let i2 = i1;

    // Copies i2 into the integral parameter of the function. The argument i2 remains valid after the function
    // returns.
    creates_copy(i2);

    // Slices represent a reference to a contiguous sequence of elements in a collection and do not
    // take ownership. They prevent the invalidation of the object that they slice by existing as
    // an immutable reference to it.
    // Note that String literals are also slices that point to a location in the compiled binary and are therefore immutable.
    let mut s = String::from("Hello world!");
    // The slice word is an immutable reference to s.
    let word = first_word(&s);

    // This would not compile as clear() modifies s and would therefore need to operate on a
    // mutable reference which cannot exist in parallel to the immutable reference word:
    // s.clear();
    // println!("{}", word);
}

// Returns a String slice ,i.e. a str type. An improved API would also take a String slice as a
// parameter allowing callers to pass Strings and String literals as arguments.
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    // Python-style structured bindings using enumerate.
    for (i, &item) in bytes.iter().enumerate() {
        // Whitespace found.
        if item == b' ' {
            // Slice [0, i) of s. 0 may be omitted. i may be omitted iff i==s.len().
            return &s[0..i];
        }
    }

    // No whitespaces. The first word slice is over the whole String.
    &s[..]
}

fn compute_length(s: &String) -> usize {
    s.len()
}

fn append(s: &mut String) {
    s.push_str(" appended");
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn gives_ownership() -> String {
    let s = String::from("hello");
    s
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn creates_copy(i: i32) {
    println!("{}", i);
}
