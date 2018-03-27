extern crate rand;

use rand::Rng;
use std::thread;
// mpsc stands for "Multi-producer, single-consumer"
use std::sync::mpsc::channel;

fn main() {
    // channel() creates a connected pair of a sender and a receiver.
    // They are usually called tx and rx, which stand for
    // "transmission" and "reception"
    let (tx, rx) = channel();
    for i in 0..10 {
        // Because an mpsc channel is "Multi-producer",
        // the sender can be cloned infinitely
        let tx = tx.clone();
        thread::spawn(move || {
            println!("sending: {}", i);
            // send() pushes arbitrary data to the connected receiver
            tx.send(i).expect("Disconnected from receiver");
        });
    }
    for _ in 0..10 {
        // recv() blocks the current thread
        // until a message was received
        let msg = rx.recv().expect("Disconnected from sender");
        println!("received: {}", msg);
    }

    let (tx, rx) = channel();
    const DISCONNECT: &str = "Goodbye!";
    // The following thread will send random messages
    // until a goodbye message was sent
    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        loop {
            let msg = match rng.gen_range(0, 5) {
                0 => "Hi",
                1 => DISCONNECT,
                2 => "Howdy there, cowboy",
                3 => "How are you?",
                4 => "I'm good, thanks",
                _ => unreachable!(),
            };
            println!("sending: {}", msg);
            tx.send(msg).expect("Disconnected from receiver");
            if msg == DISCONNECT {
                break;
            }
        }
    });

    // An iterator over messages in a receiver is infinite.
    // It will block the current thread until a message is available
    for msg in rx {
        println!("received: {}", msg);
    }
}
