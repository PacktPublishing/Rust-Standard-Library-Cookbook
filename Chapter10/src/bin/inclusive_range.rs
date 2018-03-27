#![feature(inclusive_range_syntax)]

fn main() {
    // Retrieve the entire alphabet in lower and uppercase:
    let alphabet: Vec<_> = (b'A' .. b'z' + 1) // Start as u8
        .map(|c| c as char)            // Convert all to chars
        .filter(|c| c.is_alphabetic()) // Filter only alphabetic chars
        .collect(); // Collect as Vec<char>
    println!("alphabet: {:?}", alphabet);

    // Do the same, but using the inclusive range syntax:
    let alphabet: Vec<_> = (b'A' ..= b'z') // Start as u8
        .map(|c| c as char)            // Convert all to chars
        .filter(|c| c.is_alphabetic()) // Filter only alphabetic chars
        .collect(); // Collect as Vec<char>
    println!("alphabet: {:?}", alphabet);
}
