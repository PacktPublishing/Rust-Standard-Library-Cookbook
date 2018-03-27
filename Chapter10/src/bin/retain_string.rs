#![feature(string_retain)]

fn main() {
    let mut some_text = "H_el_l__o_ ___Wo_r__l_d_".to_string();
    println!("Original text: {}", some_text);
    // retain() removes all chars that don't fulfill a
    // predicate in place, making it very efficient
    some_text.retain(|c| c != '_');
    println!("Text without underscores: {}", some_text);
    some_text.retain(char::is_lowercase);
    println!("Text with only lowercase letters: {}", some_text);

    // Before retain, you had to filter the string as an iterator over chars
    // This will however create a new String, generating overhead
    let filtered: String = "H_el_l__o_ ___Wo_r__l_d_"
        .chars()
        .filter(|c| *c != '_')
        .collect();
    println!("Text filtered by an iterator: {}", filtered);
}
