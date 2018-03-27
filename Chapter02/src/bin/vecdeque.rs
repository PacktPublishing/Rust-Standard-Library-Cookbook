use std::collections::VecDeque;

fn main() {
    // A VecDeque is best thought of as a
    // First-In-First-Out (FIFO) queue

    // Usually, you will use it to push_back data
    // and then remove it again with pop_front
    let mut orders = VecDeque::new();
    println!("A guest ordered oysters!");
    orders.push_back("oysters");

    println!("A guest ordered fish and chips!");
    orders.push_back("fish and chips");

    let prepared = orders.pop_front();
    if let Some(prepared) = prepared {
        println!("{} are ready", prepared);
    }

    println!("A guest ordered mozarella sticks!");
    orders.push_back("mozarella sticks");

    let prepared = orders.pop_front();
    if let Some(prepared) = prepared {
        println!("{} are ready", prepared);
    }

    println!("A guest ordered onion rings!");
    orders.push_back("onion rings");

    let prepared = orders.pop_front();
    if let Some(prepared) = prepared {
        println!("{} are ready", prepared);
    }

    let prepared = orders.pop_front();
    if let Some(prepared) = prepared {
        println!("{} are ready", prepared);
    }

    // You can freely switch your pushing
    // from front to back and vice versa
    let mut sentence = VecDeque::new();
    sentence.push_back("a");
    sentence.push_front("had");
    sentence.push_back("little");
    sentence.push_front("Mary");
    sentence.push_back("Lamb");
    println!("sentence: {:?}", sentence);

    // The same applies to popping data
    sentence.pop_front();
    sentence.push_front("Jimmy");
    sentence.pop_back();
    sentence.push_back("Cat");
    println!("sentence: {:?}", sentence);


    // The rest of the VecDeque's methods are
    // pretty much the same as the vector's
    // However, the VecDeque has additional options
    // when swap removing!
    let mut some_queue = VecDeque::with_capacity(5);
    some_queue.push_back("A");
    some_queue.push_back("B");
    some_queue.push_back("C");
    some_queue.push_back("D");
    some_queue.push_back("E");
    println!("some_queue: {:?}", some_queue);

    // This is the same as Vec's swap_remove
    some_queue.swap_remove_back(2);
    println!("some_quere after swap_remove_back: {:?}", some_queue);

    // This is the nearly the same, but swaps the removed
    // element with the first one instead of the last one
    some_queue.swap_remove_front(2);
    println!("some_quere after swap_remove_front: {:?}", some_queue);
}
