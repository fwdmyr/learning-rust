fn main() {
    // Vector

    // Creating
    // Requires type annotation as no push is present.
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    // Updating
    // No type annotation as first push allows for type inference.
    let mut v3 = Vec::new();
    v3.push(1);
    v3.push(2);

    // Accessing
    // Returns a ref and panics when out of bounds access occurs.
    let first: &i32 = &v3[0];
    // Pushing to v3 would not compile as reference to first element might dangle (think
    // std::vector reallocation when size == capacity).
    // Returns an Option enum that either holds the reference or is None.
    let maybe_first: Option<&i32> = v3.get(0);
    match maybe_first {
        Some(first) => println!("First element"),
        None => println!("Out of bounds access"),
    }

    let mut v4 = vec![1, 2, 3, 4, 5];

    // Loop over elements as mutable references.
    for i in &mut v4 {
        // Dereference i to operate on it.
        *i += 1;
    }
    // Loop over elements as immutable references.
    for i in &v4 {
        println!("{}", i);
    }

    // Use enums as vector elements to represent heterogenous collections.
    enum Cell {
        Int(i32),
        Float(f32),
        Text(String),
    }

    let mut v5 = Vec::new();
    v5.push(Cell::Int(1));
    v5.push(Cell::Float(1.1));
    v5.push(Cell::Text(String::from("Text")));
    let last_element: Option<Cell> = v5.pop();

    // Strings

    // Creating
    let data = "initial contents";
    let mut s = String::new();
    s = "initial contents".to_string();
    // These two ways are equivalent.
    s = data.to_string();
    s = String::from(data);

    // Updating
    // Push a string slice.
    s.push_str(" and more");
    // Push a char.
    s.push('!');
    // Use operator+
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    // The signature of operator+ looks something like this:
    // fn operator+(self, s : &str) -> String;
    // It takes self by value, so tic is moved from and invalidated.
    // The other argument is a string slice, i.e. an immutable reference.
    // We therefore get let s = operator+(operator+(tic, &tac), &toe);
    // Note that implicit conversion from &String to &str happens for tac and toe.
    let s = tic + &tac + &toe;
    // Alternatively, using the format macro yields the same result but does not take ownership of
    // any of its parameters.
    let s = format!("{}{}", tac, toe);
    // This still compiles!
    println!("{}", tac);

    // Accessing
    // Rust strings support Unicode, so characters that are larger than a single byte are possible.
    // That is why indexing using the bracket-notation is not possible and len() returns the number
    // of bytes in a string which does not necessarily match the number of Unicode characters.
    // Slicing the string is possible but may cause runtime panics, so tread with caution!
    let cyka_blyat = "сука блядь";
    // Initializes the slice "сука" as each Cyrillic character is two bytes long.
    // Taking a slice of uneven length, i.e. &cyka_blyat[0..1] would cause the program to panic.
    let cyka = &cyka_blyat[0..8];
    println!("{}", cyka);
    for c in cyka_blyat.chars() {
        // Iterates over the Unicode characters.
    }
    for b in cyka_blyat.bytes() {
        // Iterates over the raw bytes.
    }

    // HashMap
    use std::collections::HashMap;

    // Creating
    let mut scores = HashMap::new();
    // Arguments to insert get moved into the hashmap if they implement Drop.
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let points = vec![10, 50];
    // Create by calling collect on a vector of tuples.
    // Here, we zip the key and value collections (like in Python) to a vector of tuples and then
    // call collect on them. The type annotation is required because collect's return type is
    // overloaded for different collections. The underscore serves as an instruction for the compiler to
    // deduce the type similar to auto.
    let scores: HashMap<_, _> = teams.iter().zip(points.iter()).collect();
    // This style of constructing a HashMap does not invalidate the key and value collections even
    // if their element types implement Drop!
    let t0 = &teams[0];

    // Updating
    let mut scores = HashMap::new();
    scores.insert(String::from("Green"), 15);
    // Keys that already exist in the HashMap are overridden by default.
    scores.insert(String::from("Green"), 20);

    // Only insert a new key-value pair if key does not exist with this idiom.
    // Inserts the pair {"Red" : 5}
    scores.entry(String::from("Red")).or_insert(5);
    // Does not override the pair {"Green" : 20}
    scores.entry(String::from("Green")).or_insert(5);
    // The method or_insert additionally returns a mutable reference to the corresponding value regardless of whether the key exists or not.
    let thirty = scores.entry(String::from("Pink")).or_insert(30);
    println!("{}", thirty);
    println!("{:?}", scores);

    // Looking up a value and updating based on the old value is also easy and non-verbose.
    let text = "hello world cruel world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // Type deduction works too, this is just to emphasize that we get a mutable reference.
        let count: &mut i32 = map.entry(word).or_insert(0);
        // Dereference and increment.
        *count += 1;
    }
    println!("{:?}", map);
}
