extern crate slab;
use slab::{Slab, VacantEntry};

fn main() {
    // A slab is meant to be used as a limited buffer
    // As such, you should initialize it with a pre-
    // defined capacity
    const CAPACITY: usize = 1024;
    let mut slab = Slab::with_capacity(CAPACITY);

    // You cannot simply access a slab's entry by
    // index or by searching it. Instead, every
    // insert gives you a key that you can use to
    // access its entry
    let hello_key = slab.insert("hello");
    let world_key = slab.insert("world");

    println!("hello_key -> '{}'", slab[hello_key],);
    println!("world_key -> '{}'", slab[world_key],);


    // You can pass an "empty spot" around
    // in order to be filled
    let data_key = { 
        let entry = slab.vacant_entry();
        fill_some_data(entry)
    };
    println!("data_key -> '{}'", slab[data_key],);

    // When iterating, you get a key-value pair
    for (key, val) in &slab {
        println!("{} -> {}", key, val);
    }

    // If you want to keep your slab at a constant
    // capacity, you have to manually check its
    // length before inserting data
    if slab.len() != slab.capacity() {
        slab.insert("the slab is not at capacity yet");
    }
}


fn fill_some_data(entry: VacantEntry<&str>) -> usize {
    let data = "Some data";
    // insert() consumes the entry
    // so we need to get the key before
    let key = entry.key();
    entry.insert(data);
    key
}