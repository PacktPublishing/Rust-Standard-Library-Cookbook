extern crate glob;
use glob::{glob, glob_with, MatchOptions};

fn main() {
    println!("All all Rust files in all subdirectories:");
    for entry in glob("**/*.rs").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("Failed to read file: {:?}", e),
        }
    }

    // Set the glob to be case insensitive and ignore hidden files
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_leading_dot: true,
        ..Default::default()
    };


    println!(
        "All files that contain the word \"ferris\" case insensitive \
         and don't contain an underscore:"
    );
    for entry in glob_with("*Ferris[!_]*", &options).expect("Failed to read glob pattern") {
        if let Ok(path) = entry {
            println!("{:?}", path.display())
        }
    }
}
