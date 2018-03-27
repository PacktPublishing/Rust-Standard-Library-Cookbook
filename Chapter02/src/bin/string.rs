fn main() {
    // As a String is a kind of vector,
    // you can construct them the same way
    let mut s = String::new();
    s.push('H');
    s.push('i');
    println!("s: {}", s);

    // The String however can also be constructed
    // from a string slice (&str)
    // The next two ways of doing to are equivalent
    let s = "Hello".to_string();
    println!("s: {}", s);
    let s = String::from("Hello");
    println!("s: {}", s);

    // A String in Rust will always be valid UTF-8
    let s = "汉语 한글 Þjóðhildur 😉 🍺".to_string();
    println!("s: {}", s);
    
    // Append strings to each other
    let mut s = "Hello ".to_string();
    s.push_str("World");

    // Iterate over the character
    // A "character" is defined here as a
    // Unicode Scalar Value
    for ch in "Tubular".chars() {
        print!("{}.", ch);
    }
    println!();
    // Be careful though, a "character" might not
    // always be what you expect
    for ch in "y̆".chars() {
        // This does NOT print y̆
        print!("{} ", ch);
    }
    println!();

    // Splitting a string in various ways

    // Split a string slice into two halves
    let (first, second) = "HelloThere".split_at(5);
    println!("first: {}, second: {}", first, second);

    // Split on individual lines
    let haiku = "\
        she watches\n\
        satisfied after love\n\
        he lies\n\
        looking up at nothing\n\
    ";
    for line in haiku.lines() {
        println!("\t{}.", line);
    }

    // Split on substrings
    for s in "Never;Give;Up".split(';') {
        println!("{}", s);
    }
    // When the splitted string is at the beginning or end,
    // it will result in the empty string
    let s: Vec<_> = "::Hi::There::".split("::").collect();
    println!("{:?}", s);
    
    // If you can eliminate the empty strings at the end
    // by using split_termitor
    let s: Vec<_> = "Mr. T.".split_terminator('.').collect();
    println!("{:?}", s);

    // char has a few method's that you can use to split on
    for s in "I'm2fast4you".split(char::is_numeric) {
        println!("{}", s);
    }

    // Split only a certain amount of times
    for s in "It's not your fault, it's mine".splitn(3, char::is_whitespace) {
        println!("{}", s);
    }

    // Get only the substrings that match a pattern
    // This is the opposite of splitting
    for c in "The Dark Knight rises".matches(char::is_uppercase) {
        println!("{}", c);
    }

    // Check if a string starts with something
    let saying = "The early bird gets the worm";
    let starts_with_the = saying.starts_with("The");
    println!("Does \"{}\" start with \"The\"?: {}", saying, starts_with_the);
    let starts_with_bird = saying.starts_with("bird");
    println!("Does \"{}\" start with \"bird\"?: {}", saying, starts_with_bird);

    // Check if a string ends with something
    let ends_with_worm = saying.ends_with("worm");
    println!("Does \"{}\" end with \"worm\"?: {}", saying, ends_with_worm);

    // Check if the string contains something somewhere
    let contains_bird = saying.contains("bird");
    println!("Does \"{}\" contain \"bird\"?: {}", saying, contains_bird);


    // Remove whitespace

    // Splitting on whitespace might not result in what you expect
    let a_lot_of_whitespace = "    I   love spaaace     ";
    let s: Vec<_> = a_lot_of_whitespace.split(' ').collect();
    println!("{:?}", s);
    // Use split_whitespace instead
    let s: Vec<_> = a_lot_of_whitespace.split_whitespace().collect();
    println!("{:?}", s);

    // Remove leading and trailing whitespace
    let username = "   P3ngu1n\n".trim();
    println!("{}", username);
    // Remove only leading whitespace
    let username = "   P3ngu1n\n".trim_left();
    println!("{}", username);
    // Remove only trailing whitespace
    let username = "   P3ngu1n\n".trim_right();
    println!("{}", username);


    // Parse a string into another data type
    // This requires type annotation
    let num = "12".parse::<i32>();
    if let Ok(num) = num {
        println!("{} * {} = {}", num, num, num * num);
    }

    // Modify the string

    // Replace all occurences of a pattern
    let s = "My dad is the best dad";
    let new_s = s.replace("dad", "mom");
    println!("new_s: {}", new_s);

    // Replace all characters with their lowercase
    let lowercase = s.to_lowercase();
    println!("lowercase: {}", lowercase);

    // Replace all characters with their uppercase
    let uppercase = s.to_uppercase();
    println!("uppercase: {}", uppercase);

    // These also work with other languages
    let greek = "ὈΔΥΣΣΕΎΣ";
    println!("lowercase greek: {}", greek.to_lowercase());

    // Repeat a string
    let hello = "Hello! ";
    println!("Three times hello: {}", hello.repeat(3));
}
