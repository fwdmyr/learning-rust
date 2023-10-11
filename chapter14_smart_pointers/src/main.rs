// Cons list. A recursive type that contains itself and could nest infinitely.
// The size of the instance of this type is unbounded and Rust cannot generally instantiate such
// objects on the stack. Therefore, the following does not compile:
// enum List {
//     // The nested pair.
//     Cons(i32, List),
//     // The recursive base case.
//     Nil,
// }
// The solution is to define the recursive type using a smart pointer which has a known size at
// compile time and points to some data on the heap:
enum List {
    // The nested pair.
    Cons(i32, Box<List>),
    // The recursive base case.
    Nil,
}

// Tuple struct that will be dereferencable just like the standard library Box.
struct MyBox<T: std::fmt::Debug>(T);

impl<T> MyBox<T>
where
    T: std::fmt::Debug,
{
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T>
where
    T: std::fmt::Debug,
{
    type Target = T;

    // Implementing this function allows us to use operator* to dereference MyBox without taking
    // ownership of its contents.
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> Drop for MyBox<T>
where
    T: std::fmt::Debug,
{
    fn drop(&mut self) {
        println!("Dropping MyBox<{:?}>", self.0);
    }
}

fn print_str(s: &str) {
    println!("{}", s);
}

use crate::List::{Cons, Nil};

use std::rc::Rc;

fn main() {
    // unique_ptr style smart pointer.
    let b = Box::new(5);
    println!("b = {}", b);
    // Boxes can be dereferenced to get the value they point to.
    assert_eq!(*b, 5);

    // shared_ptr style smart pointer.
    let sp1 = Rc::new(1);
    // Rc::clone only increments the reference count and does not perform a deep copy.
    let sp2 = Rc::clone(&sp1);
    // The clone method does create a deep copy and increments the reference count. Always use Rc::clone!!!
    let sp3 = sp2.clone();
    // The strong count is 3!
    println!("Reference count for sp1: {}", Rc::strong_count(&sp1));

    let b = MyBox::new(5);
    // Behind the scenes, Rust translates *b to *(b.deref()).
    assert_eq!(*b, 5);

    // Instantiating a recursive type.
    let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let s = String::from("Rust");
    let s_ptr = MyBox::new(String::from("Rules"));
    print_str(&s);
    // For any type Outer<Inner> that implements the Deref trait, rust automatically converts
    // &Outer<Inner> to &Inner when this would match a function's signature. This is called deref
    // coercion.
    // Without deref coercion, we would have to write print_str(&(*s_ptr)[..]). This dereferences
    // MyBox and takes a slice of the resulting String that is equal to the whole String.
    // Here, two derefs take place to get the following sequence of conversions:
    // &MyBox<String> -> &String -> &str.
    print_str(&s_ptr);

    // Rust suppports the following deref coercions:
    // 1) From &T to &U when T == Deref<Target=U>.
    // 2) From &mut T to &mut U when T == DerefMut<Target=U>.
    // 3) From &mut T to &U when T == Deref<Target=U>.

    let i_ptr = MyBox::new(5);
    // Drop early with std::mem::drop.
    std::mem::drop(i_ptr);
    println!("Dropped before exiting main.");

    // The interior mutability pattern allows us to mutate data even when there are immutable
    // references to it. This would otherwise be disallowed by the borrowing rules. This involves
    // unsafe code which means that the compiler cannot guarantee that the borrowing rules will be
    // followed at compile time.
    // RefCell<T> allows mutable borrows checked at runtime. We can mutate the value inside
    // RefCell<T> even when RefCell<T> is immutable. If we violate the borrowing rules at runtime,
    // the program panics.
    // Refer to pages 331 - 338 for an example.
}
