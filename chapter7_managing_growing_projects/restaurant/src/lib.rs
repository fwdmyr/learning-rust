// Declaration only. Definition lives in src/front_of_house.rs
// The file name serves as the module name.
// Nested modules in separate files need to live in a directory with the same name as the module.
mod front_of_house;

mod back_of_house {

    // The fields of (public) structs are private by default.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // This function is required to create Breakfast outside of the back_of_house module as
        // client code does not have access to the seasonal_fruit field and could therefore never
        // construct an instance of Breakfast.
        pub fn summer(toast: String) -> Breakfast {
            Breakfast {
                toast,
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }

    // The fields of (public) enums are public by default.
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        // Searches for serve_order in the parent module of back_of_house (the crate module).
        // Like using ../desired/path in filesystems.
        super::serve_order();
    }

    fn cook_order() {}
}

fn prepare_meal() {
    let breakfast = back_of_house::Breakfast::summer(String::from("Wheat"));
    // This would not compile:
    // let breakfast = Breakfast {toast : "Wheat", seasonal_fruit : "Melon",};

    let appetizer = back_of_house::Appetizer::Soup;
}

fn serve_order() {}

// Keyword pub marks this function as part of the public API.
// Per default, everything is private and only child modules can see the methods defined at parent
// scope but not vice-versa.
// Siblings can also see each other. That is why the front_of_house module (a sibling of
// eat_at_restaurant) does not need to be public.
pub fn eat_at_restaurant() {
    // Use absolute path to refer to function.
    // When in doubt, prefer using absolute paths.
    // This is akin to prepending a cpp namespace with ::
    crate::hosting::add_to_waitlist();

    // Use relative path to refer to function.
    hosting::add_to_waitlist();
}

// Bring the nested module namespace into scope.
// This also works with relative paths, i.e. using front_of_house::hosting.
// Prefer this idiomatic way that brings the parent namespace over bringing the single entity into scope with
// use crate::front_of_house::hosting::add_to_waitlist!
pub use crate::front_of_house::hosting;

pub fn add_two_to_waitlist() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// Re-export names with pub use if you want to make them accessible for external callers.
// Breakfast and Breakfast::summer can now be used from other translation units that include this
// library.
pub use crate::back_of_house::Breakfast;

// Keeping use lists small.

// Bring std::cmp::Ordering and std::io into scope.
use std::{cmp::Ordering, io};
// Bring std::collections and std::collections::HashMap into scope.
use std::collections::{self, HashMap};
// Bring all public items into scope using the glob operator.
use std::fmt::*;
