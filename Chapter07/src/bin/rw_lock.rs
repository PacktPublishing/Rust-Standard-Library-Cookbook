use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    // An RwLock works like the RefCell, but blocks the current
    // thread if the resource is unavailable
    let resource = Arc::new(RwLock::new("Hello World!".to_string()));

    // The reader_a thread will print the current content of
    // our resource fourty times
    let reader_a = {
        let resource = resource.clone();
        thread::spawn(move || {
            for _ in 0..40 {
                // Lock resource for reading access
                let resource = resource
                    .read()
                    .expect("Failed to lock resource for reading");
                println!("Reader A says: {}", resource);
            }
        })
    };

    // The reader_b thread will print the current content of
    // our resource fourty times as well. Because RwLock allows
    // multiple readers, it will execute at the same time as reader_a
    let reader_b = {
        let resource = resource.clone();
        thread::spawn(move || {
            for _ in 0..40 {
                // Lock resource for reading access
                let resource = resource
                    .read()
                    .expect("Failed to lock resource for reading");
                println!("Reader B says: {}", resource);
            }
        })
    };

    // The writer thread will modify the resource ten times.
    // Because RwLock enforces Rust's access rules
    // (multiple readers xor one writer), this thread will wait until
    // thread_a and thread_b are not using the resource and then block
    // them both until its done.
    let writer = {
        let resource = resource.clone();
        thread::spawn(move || {
            for _ in 0..10 {
                // Lock resource for writing access
                let mut resource = resource
                    .write()
                    .expect("Failed to lock resource for writing");

                resource.push('!');
            }
        })
    };

    reader_a.join().expect("Reader A panicked");
    reader_b.join().expect("Reader B panicked");
    writer.join().expect("Writer panicked");
}
