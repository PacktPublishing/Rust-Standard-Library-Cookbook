use std::thread;
use std::sync::Arc;

fn main() {
    // An Arc ("Atomically Reference Counted") is used the exact
    // same way as an Rc, but also work in a parallel context
    let some_resource = Arc::new("Hello World".to_string());

    // We use it to give a new thread ownership of a clone of the Arc
    let thread_a = {
        // It is very common to give the clone the same name as the original
        let some_resource = some_resource.clone();
        // The clone is then moved into the closure:
        thread::spawn(move || {
            println!("Thread A says: {}", some_resource);
        })
    };
    let thread_b = {
        let some_resource = some_resource.clone();
        thread::spawn(move || {
            println!("Thread B says: {}", some_resource);
        })
    };

    // .join() blocks the main thread until the other thread is done
    thread_a.join().expect("Thread A panicked");
    thread_b.join().expect("Thread B panicked");
}
