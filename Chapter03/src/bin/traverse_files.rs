extern crate walkdir;
use walkdir::{DirEntry, WalkDir};

fn main() {
    println!("All file paths in this directory:");
    for entry in WalkDir::new(".") {
        if let Ok(entry) = entry {
            println!("{}", entry.path().display());
        }
    }

    println!("All non-hidden file names in this directory:");
    WalkDir::new("../chapter_three")
    .into_iter()
    .filter_entry(|entry| !is_hidden(entry)) // Look only at non-hidden enthries
    .filter_map(Result::ok) // Keep all entries we have access to
    .for_each(|entry| {
        // Convert the name returned by theOS into a Rust string
        // If there are any non-UTF8 symbols in it, replace them with placeholders
        let name = entry.file_name().to_string_lossy();
        println!("{}", name)
    });

    println!("Paths of all subdirectories in this directory:");
    WalkDir::new(".")
    .into_iter()
    .filter_entry(is_dir) // Look only at directories
    .filter_map(Result::ok) // Keep all entries we have access to
    .for_each(|entry| {
        let path = entry.path().display();
        println!("{}", path)
    });

    let are_any_readonly = WalkDir::new("..")
    .into_iter()
    .filter_map(Result::ok) // Keep all entries we have access to
    .filter(|e| has_file_name(e, "vector.rs")) // Get the ones with a certain name
    .filter_map(|e| e.metadata().ok()) // Get metadata if the OS allows it
    .any(|e| e.permissions().readonly()); // Check if at least one entry is readonly
    println!(
        "Are any the files called 'vector.rs' readonly? {}",
        are_any_readonly
    );

    let total_size = WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok) // Keep all entries we have access to
        .filter_map(|entry| entry.metadata().ok()) // Get metadata if supported
        .filter(|metadata| metadata.is_file()) // Keep all files
        .fold(0, |acc, m| acc + m.len()); // Accumulate sizes

    println!("Size of current directory: {} bytes", total_size);
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false) // Return false if the filename is invalid UTF8
}

fn is_dir(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
}

fn has_file_name(entry: &DirEntry, name: &str) -> bool {
    // Check if file name contains valid unicode
    match entry.file_name().to_str() {
        Some(entry_name) => entry_name == name,
        None => false,
    }
}
