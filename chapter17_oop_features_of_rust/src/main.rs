use chapter17_oop_features_of_rust::blog_post::Post;
use chapter17_oop_features_of_rust::{Button, Draw, Screen};

// User-defined type.
struct SelectBox {}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox::draw()");
    }
}

fn main() {
    let screen = Screen {
        components: vec![Box::new(Button {}), Box::new(SelectBox {})],
    };

    // Does not need to know about SelectBox. Trait objects allow us to use libraries with
    // user-defined types that the library implementers did not know about as long as these new
    // types implement all the necessary types.
    // The address of SelectBox::draw() is determined by a lookup in the vtable and the call is
    // dispatched accordingly at runtime.
    screen.run();

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today.");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today.", post.content());
}
