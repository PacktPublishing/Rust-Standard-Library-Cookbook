use std::thread;

fn main() {
    // Spawning a thread lets it execute a lambda
    let child = thread::spawn(|| println!("Hello from a new thread!"));
    println!("Hello from the main thread!");
    // Joining a child thread with the main thread means
    // that the main thread waits until the child has
    // finished it's work
    child.join().expect("Failed to join the child thread");

    let sum = parallel_sum(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    println!("The sum of the numbers 1 to 10 is {}", sum);
}

// We are going to write a function that
// sums the numbers in a slice in parallel
fn parallel_sum(range: &[i32]) -> i32 {
    // We are going to use exactly 4 threads to sum the numbers
    const NUM_THREADS: usize = 4;

    // If we have less numbers than threads,
    // there's no point in multithreading them
    if range.len() < NUM_THREADS {
        sum_bucket(range)
    } else {
        // We define "bucket" as the amount of numbers
        // we sum in a single thread
        let bucket_size = range.len() / NUM_THREADS;
        let mut count = 0;
        // This vector will keep track of our threads
        let mut threads = Vec::new();
        // We try to sum as much as possible in othe threads
        while count + bucket_size < range.len() {
            let bucket = range[count..count + bucket_size].to_vec();
            let thread = thread::Builder::new()
                .name("calculation".to_string())
                .spawn(move || sum_bucket(&bucket))
                .expect("Failed to create the thread");
            threads.push(thread);

            count += bucket_size
        }
        // We are going to sum the rest in the main thread
        let mut sum = sum_bucket(&range[count..]);

        // Time to add the results up
        for thread in threads {
            sum += thread.join().expect("Failed to join thread");
        }
        sum
    }
}

// This is the function that will be executed in the threads
fn sum_bucket(range: &[i32]) -> i32 {
    let mut sum = 0;
    for num in range {
        sum += *num;
    }
    sum
}
