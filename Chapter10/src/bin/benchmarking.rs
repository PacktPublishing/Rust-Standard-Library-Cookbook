#![feature(test)]
// The test crate was primarily designed for
// the Rust compiler itself, so it has no stability guaranteed
extern crate test;

pub fn slow_fibonacci_recursive(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => slow_fibonacci_recursive(n - 1) + slow_fibonacci_recursive(n - 2),
    }
}

pub fn fibonacci_imperative(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut penultimate;
            let mut last = 1;
            let mut fib = 0;
            for _ in 0..n {
                penultimate = last;
                last = fib;
                fib = penultimate + last;
            }
            fib
        }
    }
}

pub fn memoized_fibonacci_recursive(n: u32) -> u32 {
    fn inner(n: u32, penultimate: u32, last: u32) -> u32 {
        match n {
            0 => penultimate,
            1 => last,
            _ => inner(n - 1, last, penultimate + last),
        }
    }
    inner(n, 0, 1)
}

pub fn fast_fibonacci_recursive(n: u32) -> u32 {
    fn inner(n: u32, penultimate: u32, last: u32) -> u32 {
        match n {
            0 => last,
            _ => inner(n - 1, last, penultimate + last),
        }
    }
    match n {
        0 => 0,
        _ => inner(n - 1, 0, 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    // Functions annotated with the bench attribute will
    // undergo a performance evaluation when running "cargo bench"
    #[bench]
    fn bench_slow_fibonacci_recursive(b: &mut Bencher) {
        b.iter(|| {
            // test::block_box is "black box" for the compiler and LLVM
            // Telling them to not optimize a variable away
            let n = test::black_box(20);
            slow_fibonacci_recursive(n)
        });
    }

    #[bench]
    fn bench_fibonacci_imperative(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(20);
            fibonacci_imperative(n)
        });
    }

    #[bench]
    fn bench_memoized_fibonacci_recursive(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(20);
            memoized_fibonacci_recursive(n)
        });
    }

    #[bench]
    fn bench_fast_fibonacci_recursive(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(20);
            fast_fibonacci_recursive(n)
        });
    }
}
