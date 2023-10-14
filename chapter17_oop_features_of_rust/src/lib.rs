// You can only make object-safe traits into trait objects. A trait is object safe iff:
// 1) The return type of all methods defined in the trait is not Self.
//    &&
// 2) There are no generic type parameters in any of the methods defined in the trait.

pub trait Draw {
    fn draw(&self);
}

pub struct Button {}

impl Draw for Button {
    fn draw(&self) {
        println!("Button::draw()");
    }
}

pub struct Screen {
    // Box<dyn Draw> is a trait object, i.e. a stand-in for any type inside a Box that implements
    // the Draw trait.
    // Trait objects allow for multiple concrete types to fill in for the trait object at runtime.
    // Compile-time polymorphism:
    // templates and function overloading in C++ <==> Generic types with trait bounds in Rust.
    // Runtime polymorphism:
    // Virtual functions in C++ <==> Trait objects in Rust.
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// A small example that uses the state pattern.
// Note: This is not the best way to solve this particular toy problem in Rust. One should make use
// of the type system instead of forcing OOP paradigms.

pub mod blog_post {
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        // Only way to create a new post is through the call to Post::new().
        // Enforces that new posts always start as Draft.
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }

        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn content(&self) -> &str {
            // We dont want to move out of self.state -> get state as_ref().
            // Deref coercion takes place and the type &Box<dyn State> returned from unwrap()
            // transform to the concrete type that implements State and has the content() method.
            self.state.as_ref().unwrap().content(&self)
        }

        pub fn request_review(&mut self) {
            // Takes the Box<dyn State> out of the Option and leaves a None behind (temporarily).
            // This allows us to move the state variable out of the field rather than borrowing it.
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review());
            }
        }

        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve());
            }
        }
    }

    trait State {
        // Takes parameter with type Box<Self> rather than self, &self, &mut self.
        // This method is valid only when called on a Box holding the type.
        // The function takes ownership of Box<Self> and invalidates the old state.
        fn request_review(self: Box<Self>) -> Box<dyn State>;

        fn approve(self: Box<Self>) -> Box<dyn State>;

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            ""
        }
    }

    struct Draft {}

    impl State for Draft {
        // Transition function from Draft to PendingReview.
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

    struct PendingReview {}

    impl State for PendingReview {
        // Noop.
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }
    }

    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        // Lifetimes of post and the return type (i.e. a field of post) are related.
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }
}
