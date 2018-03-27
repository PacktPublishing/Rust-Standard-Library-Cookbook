use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering, ATOMIC_BOOL_INIT, ATOMIC_USIZE_INIT};
use std::thread;
use std::ops::{Deref, DerefMut};
use std::cell::UnsafeCell;

fn main() {
    // Atomics are primitive types suited for
    // well defined concurrent behaviour
    let some_number = AtomicUsize::new(0);
    // They are usually initialized by copying them from
    // their global constants, so the following line does the same:
    let some_number = ATOMIC_USIZE_INIT;

    // load() gets the current value of the atomic
    // Ordering tells the compiler how exactly to handle the interactions
    // with other threads. SeqCst ("Sequentially Consistent") can always be used
    // as it results in the same thing as if no parallelism was involved
    let curr_val = some_number.load(Ordering::SeqCst);
    println!("The current value of some_number is {}", curr_val);

    // store() sets the variable
    some_number.store(123, Ordering::SeqCst);
    let curr_val = some_number.load(Ordering::SeqCst);
    println!("The current value of some_number is {}", curr_val);

    // swap() sets the variable and returns the old value
    let old_val = some_number.swap(12_345, Ordering::SeqCst);
    let curr_val = some_number.load(Ordering::SeqCst);
    println!("The old value of some_number was {}", old_val);
    println!("The current value of some_number is {}", curr_val);

    // compare_and_swap only swaps the variable if it
    // is currently equal to the first argument.
    // It will always return the old variable
    let comparison = 12_345;
    let new_val = 6_789;
    let old_val = some_number.compare_and_swap(comparison, new_val, Ordering::SeqCst);
    if old_val == comparison {
        println!("The value has been updated");
    }

    // The previous atomic code is equivalent to
    // the following sequential code
    let mut some_normal_number = 12_345;
    let old_val = some_normal_number;
    if old_val == comparison {
        some_normal_number = new_val;
        println!("The value has been updated sequentially");
    }

    // fetch_add() and fetch_sub() add/subtract a number from the value,
    // returning the old value
    let old_val_one = some_number.fetch_add(12, Ordering::SeqCst);
    let old_val_two = some_number.fetch_sub(24, Ordering::SeqCst);
    let curr_val = some_number.load(Ordering::SeqCst);
    println!(
        "some_number was first {}, then {} and is now {}",
        old_val_one, old_val_two, curr_val
    );

    // fetch_or() performs an "or" ("||") operation on the variable and
    // an argument and sets the variable to the result. It then returns the old value.
    // For the other logical operations, fetch_and(), fetch_nand() and fetch_xor also exist
    let some_bool = ATOMIC_BOOL_INIT;
    let old_val = some_bool.fetch_or(true, Ordering::SeqCst);
    let curr_val = some_bool.load(Ordering::SeqCst);
    println!("({} || true) is {}", old_val, curr_val);

    // The following is a demonstration of our own Mutex implementation,
    // based on an AtomicBool that checks if it's locked or not
    let naive_mutex = Arc::new(NaiveMutex::new(1));

    // The updater thread will set the value in the mutex to 2
    let updater = {
        let naive_mutex = naive_mutex.clone();
        thread::spawn(move || {
            let mut val = naive_mutex.lock();
            *val = 2;
        })
    };

    // The updater thread will print the value in the mutex
    let printer = {
        let naive_mutex = naive_mutex.clone();
        thread::spawn(move || {
            let val = naive_mutex.lock();
            println!("The value in the naive mutex is: {}", *val);
        })
    };

    // The exact order of execution is unpredictable,
    // but our mutex guarantees that the two threads will
    // never access the data at the same time
    updater.join().expect("The updater thread panicked");
    printer.join().expect("The printer thread panicked");
}

// NaiveMutex is an easy, albeit very suboptimal,
// implementation of a Mutex ("Mutual Exclusion"), similar to std::sync::Mutex
// A mutex is a lock that only allows one thread to access a ressource at all times
pub struct NaiveMutex<T> {
    locked: AtomicBool,
    // UnsafeCell is the underlying struct of every
    // internally mutable container such as ours
    data: UnsafeCell<T>,
}

// This is a RAII guard, identical to the one from the last chapter
pub struct NaiveMutexGuard<'a, T: 'a> {
    naive_mutex: &'a NaiveMutex<T>,
}

impl<T> NaiveMutex<T> {
    pub fn new(data: T) -> Self {
        NaiveMutex {
            locked: ATOMIC_BOOL_INIT,
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> NaiveMutexGuard<T> {
        // The following algorithm is called a "spinlock", because it keeps
        // the current thread blocked by doing nothing (it keeps it "spinning")
        while self.locked.compare_and_swap(false, true, Ordering::SeqCst) {}
        NaiveMutexGuard { naive_mutex: self }
    }
}

// Every type that is safe to send between threads is automatically
// safe to share between threads if wrapped in our mutex, as it
// guarantees that no threads will access it ressource at the same time
unsafe impl<T: Send> Sync for NaiveMutex<T> {}

// Automatically unlock the mutex on drop
impl<'a, T> Drop for NaiveMutexGuard<'a, T> {
    fn drop(&mut self) {
        self.naive_mutex.locked.store(false, Ordering::SeqCst);
    }
}

// Automatically dereference to the underlying data
impl<'a, T> Deref for NaiveMutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.naive_mutex.data.get() }
    }
}

impl<'a, T> DerefMut for NaiveMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.naive_mutex.data.get() }
    }
}
