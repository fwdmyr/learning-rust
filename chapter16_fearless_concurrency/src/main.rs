use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Spawn a thread by calling spawn with a lambda.
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Spawned thread i = {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let v = vec![1, 2, 3];

    // Closures inside threads cannot borrow variables from the scope they are created in as there
    // is no guarantee that the variable will live until the thread returns.
    // We therefore need to explicitly move variables into the closure.
    thread::spawn(move || {
        println!("I got a vector: {:?}", v);
    });

    for i in 1..5 {
        println!("Main thread i = {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Blocks the main thread until closure from handle returns.
    handle.join().unwrap();

    // Message passing to transfer data between threads.
    // "Do not communicate by sharing memory; instead, share memory by communicating."

    // Channel defined by a transmitter-receiver pair. The channel is closed when either transmitter or receiver are dropped.
    let (tx, rx) = mpsc::channel();
    let other_tx = mpsc::Sender::clone(&tx);

    // Move the transmitter into the sender thread.
    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("sender"),
            String::from("thread"),
        ];

        // Value is sent along the channel by transferring ownership to the send function.
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        // Value is sent along the channel by transferring ownership to the send function.
        for val in vals {
            other_tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Block the main thread and waits until the transmitter(s) has been dropped. As the transmitter(s)
    // has been moved into the closure(s) that sends messages, the main thread is unblocked when the
    // last closure returns and implicitly drops the last transmitter.
    // Remember: Dropping the transmitter closes the channel!
    for received in &rx {
        println!("Got: {}", received);
    }

    // Nonblocking call to receive that returns Err enum when there is no message in the channel.
    let maybe_received = match rx.try_recv() {
        Ok(msg) => println!("Got: {}", msg),
        Err(e) => println!("No message"),
    };

    // Shared-state concurrency.

    // In Rust, Mutex<T> is a smart ptr that holds a MutexGuard<T>.
    let m = Mutex::new(5);

    {
        // The call to lock returns a Result enum where Ok(MutexGuard<T>) iff aquiring the lock
        // succeeded. The lock is then released when the MutexGuard<T> goes out of scope.
        // The MutexGuard<T> implements the Deref trait and operator* returns a mutable reference
        // to the date inside.
        let mut num = m.lock().unwrap();
        *num += 1;
    }

    println!("m = {:?}", m);

    // A small example.

    // Wrap the mutex in an atomically referenced counted, i.e. thread-safe, smart ptr.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // Create a clone of the arc instance and move this into the closure of the newly spawned
        // thread.
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // Note:
    // In the same way we use RefCell<T> to allow us to mutate contents inside Rc<T>, we use
    // Mutex<T> to mutate contents inside Arc<T>.

    // The Send marker trait indicates that ownership of the type implementing Send can be
    // transferred between threads. Almost all standard types are Send (expect for Rc<T> among a
    // few others). Structs that consist of fields that all implement the Send trait are themselves
    // implicitly Send too.
    // The Sync marker trait indicates that it is safe for the type implementing Sync to be
    // referenced from multiple threads. In other words, any type T is Sync iff &T is Send. Structs
    // that consist of fields that all implement the Sync trait are themselves implicitly Sync too.
    // For those reasons, manually implementing Send and Sync is unsafe!!!
}
