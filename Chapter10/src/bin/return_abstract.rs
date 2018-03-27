#![feature(conservative_impl_trait)]

trait Animal {
    fn do_sound(&self);
}

struct Dog;
impl Animal for Dog {
    fn do_sound(&self) {
        println!("Woof");
    }
}

fn main() {
    // The caller doesn't know which exact object he gets
    // He knows only that it implements the Animal trait
    let animal = create_animal();
    animal.do_sound();

    for word in caps_words_iter("do you feel lucky, punk‽") {
        println!("{}", word);
    }

    let multiplier = create_multiplier(23);
    let result = multiplier(3);
    println!("23 * 3 = {}", result);
}

// The impl trait syntax allows us to use abstract return types
// This means that we don't specify which exact struct we return
// but which trait(s) it implements
fn create_animal() -> impl Animal {
    Dog {}
}

// Any iterator can be returned as an abstract return type
fn caps_words_iter<'a>(text: &'a str) -> impl Iterator<Item = String> + 'a {
    // Return an iterator over every word converted into ALL_CAPS
    text.trim().split(' ').map(|word| word.to_uppercase())
}

// Same goes for closures
fn create_multiplier(a: i32) -> impl Fn(i32) -> i32 {
    move |b| a * b
}
