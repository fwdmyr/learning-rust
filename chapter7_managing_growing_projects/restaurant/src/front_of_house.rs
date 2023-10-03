// Root of the module tree.
// The front_of_house module is the parent of the hosting and serving modules.
// The hosting and serving modules are children of the front_of_house module.
// The hosting and serving modules are siblings.

// Lets front_of_house see the contents if hosting.
pub mod hosting {
    // Lets eat_at_restaurant see add_to_waitlist.
    pub fn add_to_waitlist() {}

    fn seat_at_table() {}
}

// Lives in front_of_house/serving.rs
pub mod serving;

fn take_and_serve_order() {
    serving::take_order();
    serving::serve_order();
}
