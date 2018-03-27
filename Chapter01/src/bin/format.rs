fn main() {
    let colour = "red";
    // The '{}' it the formatted string gets replaced by the parameter
    let favourite = format!("My favourite colour is {}", colour);
    println!("{}", favourite);

    // You can add multiple parameters, which will be
    // put in place one after another
    let hello = "hello ";
    let world = "world!";
    let hello_world = format!("{}{}", hello, world);
    println!("{}", hello_world); // Prints "hello world!"

    // format! can concatenate any data types that
    // implement the 'Display' trait, such as numbers
    let favourite_num = format!("My favourite number is {}", 42);
    println!("{}", favourite_num); // Prints "My favourite number is 42"

    // If you want to include certain parameters multiple times
    // into the string, you can use positional parameters
    let duck_duck_goose = format!("{0}, {0}, {0}, {1}!", "duck", "goose");
    println!("{}", duck_duck_goose); // Prints "duck, duck, duck, goose!"

    // You can even name your parameters!
    let introduction = format!(
        "My name is {surname}, {forename} {surname}",
        surname = "Bond",
        forename = "James"
    );
    println!("{}", introduction) // Prints "My name is Bond, James Bond"
}
