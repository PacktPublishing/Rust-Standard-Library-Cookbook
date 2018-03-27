#![feature(generators, generator_trait, conservative_impl_trait)]

fn main() {
    // A closure that uses the keyword "yield" is called a generator
    // Yielding a value "remembers" where you left off
    // when calling .resume() on the generator
    let mut generator = || {
        yield 1;
        yield 2;
    };
    if let GeneratorState::Yielded(value) = generator.resume() {
        println!("The generator yielded: {}", value);
    }
    if let GeneratorState::Yielded(value) = generator.resume() {
        println!("The generator yielded: {}", value);
    }
    // When there is nothing left to yield,
    // a generator will automatically return an empty tuple
    if let GeneratorState::Complete(value) = generator.resume() {
        println!("The generator completed with: {:?}", value);
    }

    // At the moment, you can return a different type
    // than you yield, although this feature is considered for removal
    let mut generator = || {
        yield 100;
        yield 200;
        yield 300;
        "I'm a string"
    };
    loop {
        match generator.resume() {
            GeneratorState::Yielded(value) => println!("The generator yielded: {}", value),
            GeneratorState::Complete(value) => {
                println!("The generator completed with: {}", value);
                break;
            }
        }
    }

    // Generators are great for implementing iterators.
    // Eventually, all Rust iterators are going to be rewritten with generators
    let fib: Vec<_> = fibonacci().take(10).collect();
    println!("First 10 numbers of the fibonacci sequence: {:?}", fib);
}

// As of the time of writing, a generator does not have a
// direct conversion to an iterator yet, so we need a wrapper:
use std::ops::{Generator, GeneratorState};
struct GeneratorIterator<T>(T);
impl<T> Iterator for GeneratorIterator<T>
where
    T: Generator<Return = ()>,
{
    type Item = T::Yield;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.resume() {
            GeneratorState::Yielded(value) => Some(value),
            GeneratorState::Complete(_) => None,
        }
    }
}

fn fibonacci() -> impl Iterator<Item = u32> {
    // Using our wrapper
    GeneratorIterator(move || {
        let mut curr = 0;
        let mut next = 1;
        loop {
            yield curr;
            let old = curr;
            curr = next;
            next += old;
        }
    })
}
