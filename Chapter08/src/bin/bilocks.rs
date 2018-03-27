extern crate futures;
extern crate futures_util;

use futures::prelude::*;
use futures::executor::LocalPool;
use futures::task::{Context, LocalMap, Wake, Waker};
use futures_util::lock::BiLock;

use std::sync::Arc;

struct FakeWaker;
impl Wake for FakeWaker {
    fn wake(_: &Arc<Self>) {}
}

struct Reader<T> {
    lock: BiLock<T>,
}

struct Writer<T> {
    lock: BiLock<T>,
}

fn split() -> (Reader<u32>, Writer<u32>) {
    let (a, b) = BiLock::new(0);
    (Reader { lock: a }, Writer { lock: b })
}

fn main() {
    let pool = LocalPool::new();
    let mut exec = pool.executor();
    let waker = Waker::from(Arc::new(FakeWaker));
    let mut map = LocalMap::new();
    let mut cx = Context::new(&mut map, &waker, &mut exec);

    let (reader, writer) = split();
    println!("Lock should be ready for writer: {}",
             writer.lock.poll_lock(&mut cx).is_ready());
    println!("Lock should be ready for reader: {}",
             reader.lock.poll_lock(&mut cx).is_ready());

    let mut writer_lock = match writer.lock.lock().poll(&mut cx).unwrap() {
        Async::Ready(t) => t,
        _ => panic!("We should be able to lock with writer"),
    };

    println!("Lock should now be pending for reader: {}",
             reader.lock.poll_lock(&mut cx).is_pending());
    *writer_lock = 123;

    let mut lock = reader.lock.lock();
    match lock.poll(&mut cx).unwrap() {
        Async::Ready(_) => {
            panic!("The lock should not be lockable since writer has already locked it!")
        }
        _ => println!("Couldn't lock with reader since writer has already initiated the lock"),
    };

    let writer = writer_lock.unlock();

    let reader_lock = match lock.poll(&mut cx).unwrap() {
        Async::Ready(t) => t,
        _ => panic!("We should be able to lock with reader"),
    };

    println!("The new value for the lock is: {}", *reader_lock);

    let reader = reader_lock.unlock();
    let reunited_value = reader.reunite(writer).unwrap();

    println!("After reuniting our locks, the final value is still: {}",
             reunited_value);
}
