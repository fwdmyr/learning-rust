struct User {
    name: String,
    location: String,
    id: i32,
}

// Trait that enables debug printing for struct.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods for the Rectangle struct live in this impl context.
// Methods can take ownership of self or take a (im)mutable reference to it.
impl Rectangle {
    // Methods
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // Associated functions
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // Create an instance of type from key-value pairs.
    let user = User {
        name: String::from("Sebnem"),
        location: String::from("Ankara"),
        id: 69,
    };
    println!("Name: {}", user.name);

    // Create instances from other instances with struct update syntax. Create a copy by only
    // overriding fields in given set of key-value pairs.
    let parent_user = User {
        name: String::from("Sevil"),
        id: 68,
        ..user
    };

    // Use tuple structs when field names are not required but a unique named type is useful. Tuple structs behave
    // like tuples w.r.t. destructuring and accessing by index.
    struct Point(i32, i32, i32);
    let origin = Point(0, 0, 0);

    // Structs without fields, so called unit structs, are also possible.
    struct Unit();
    let u = Unit();

    // Field init shorthand syntax. See function.
    let default_user = build_user(String::from("Default"), String::from("Default"));

    // Small example:

    let rectangle = Rectangle {
        width: 100,
        height: 25,
    };

    // Debug print rectangle by using {:?}. This requires Rectangle to implement the Debug trait.
    println!("Rectangle is {:?}", rectangle);

    // Call the area function of rectangle using method syntax. Rust automatically adds the
    // required qualifiers and operators (&, *, mut) to rectangle to match the method signature.
    println!("Area is {}", rectangle.area());

    // Call the associated square function that is namespaced by Rectangle.
    println!("Square is {:?}", Rectangle::square(5));
}

fn build_user(name: String, location: String) -> User {
    // Field init shorthand syntax allows for less verbose struct definitions. The function
    // parameters get matched to the fields of the struct based on name.
    User {
        name,
        location,
        id: 0,
    }
}
