#[macro_use]
extern crate chapter_five_derive;

// trait definitions have to be in "consumer" crate
trait HelloWorld {
    // This method will send a friendly greeting
    fn hello_world();
}

// thanks to the code in the custom_derive crate
// we can derive from HelloWorld in order to provide
// an automatic implementation for the HelloWorld trait
#[derive(HelloWorld)]
struct Switzerland;

#[derive(HelloWorld)]
struct Britain;

#[derive(HelloWorld)]
// We can use an optional attribute to change the message
#[hello_world_name = "the Land Down Under"]
struct Australia;

fn main() {
    Switzerland::hello_world();
    Britain::hello_world();
    Australia::hello_world();
}
