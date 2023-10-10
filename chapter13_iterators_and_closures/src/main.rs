// Storing closures using generic parameters and the Fn traits.
struct Cacher<T>
where
    // Using the Fn trait declares a closure type.
    T: Fn(u32, u32) -> u32,
{
    // The private closure field.
    callable: T,
    // The private value cache.
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32, u32) -> u32,
{
    fn new(callable: T) -> Cacher<T> {
        Cacher {
            callable,
            value: None,
        }
    }

    // Memoization value method.
    fn value(&mut self, x: u32, y: u32) -> u32 {
        match self.value {
            Some(val) => val,
            None => {
                let val = (self.callable)(x, y);
                self.value = Some(val);
                val
            }
        }
    }
}

// The standard library Iterator trait.
pub trait StdIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided.
}

// Implementing custom iterators using the Iterator trait.
struct CounterToFive {
    count: u32,
}

impl CounterToFive {
    fn new() -> CounterToFive {
        CounterToFive { count: 0 }
    }
}

impl Iterator for CounterToFive {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    // This closure is not generic and the parameter and return types are inferred upon invoking
    // the closure for the first time. Function-style type annotation is possible but optional.
    let add_closure = |x, y| {
        println!("Adding {} to {}", x, y);
        x + y
    };

    let mut c = Cacher::new(add_closure);
    // Executes the callable.
    let res = c.value(1, 2);
    // Returns the cached result.
    let res = c.value(1, 2);

    // Closures can capture its enclosing scope in three different ways encoded in the three Fn traits:
    // FnOnce: Takes ownership of the captured variable (i.e. moves out of it). This closure can
    //         only be invoked once
    // FnMut:  Mutably borrows the captured variable and can therefore change the environment.
    // Fn:     Immutably borrows the captured variable.
    // The compiler infers the appropriate trait for us.

    let x = vec![1, 2, 3];
    let mut y = vec![1, 2, 3];
    let z = vec![1, 2, 3];

    // Fn
    let immutably_borrow = || {
        println!("{:?}", x);
    };
    immutably_borrow();

    // FnMut
    let mut mutably_borrow = || {
        y.push(4);
        println!("{:?}", y);
    };
    mutably_borrow();

    // FnOnce
    let take_ownership = move || {
        println!("{:?}", z);
    };
    take_ownership();

    let v = vec![1, 2, 3];
    let mut v_iter = v.iter();

    // Calling next on an iterable consumes the iterator and moves to the next element.
    // This is what happens behind the scenes in a for-loop.
    for i in 1..4 {
        if v_iter.next() != Some(&i) {
            panic!();
        }
    }
    if v_iter.next() != None {
        panic!();
    }

    // There are types of iterators:
    // into_iter: Takes ownership and iterates.
    let x = vec![1, 2, 3];
    let x_iter = x.into_iter();
    // iter_mut: Iterate over mutable references.
    let mut x = vec![1, 2, 3];
    let x_iter = x.iter_mut();
    // iter: Iterate over non-mutable references.
    let x = vec![1, 2, 3];
    let x_iter = x.iter();

    // There are methods that consume the iterator (consuming adaptors).
    let v = vec![1, 2, 3];
    let s: u32 = v.iter().sum();

    // There are methods that produce other iterators (iterator adaptors).
    let v: Vec<u32> = vec![1, 2, 3];
    // Use a closer to transform a sequence. This does not do anything by itself.
    let incr_iter = v.iter().map(|x| x + 1);
    // Consumes the transformed iterator by calling collect on it.
    let incr_v: Vec<u32> = incr_iter.collect();
    println!("{:?}", incr_v);

    // The filter method takes a closure that evaluates to a boolean and returns a reduced sequence
    // of the filtered elements (i.e. for which the closure evaluated to true).
    // This only works when we use an iterator that takes ownership of the iterable.
    let v: Vec<i32> = vec![1, 2, 3, 4];
    let denominator: i32 = 2;
    let divisible: Vec<i32> = v
        .into_iter()
        .filter(|numerator| numerator % denominator == 0)
        .collect();
    println!("{:?}", divisible);

    // Using the Iterator trait for our custom type.
    // Take the values of a CounterToFive instance.
    // [1, 2, 3, 4, 5]
    let sum: u32 = CounterToFive::new()
        // Combine with the values of another CounterToFive instance after skipping the first value.
        // [(1, 2), (2, 3), (3, 4), (4, 5)]
        .zip(CounterToFive::new().skip(1))
        // Multiply each pair together.
        // [2, 6, 12, 20]
        .map(|(a, b)| a * b)
        // Keep only those divisible by three.
        // [6, 12]
        .filter(|x| x % 3 == 0)
        // Sum them up.
        // 18
        .sum();
}
