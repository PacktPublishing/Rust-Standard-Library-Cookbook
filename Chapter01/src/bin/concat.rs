fn main() {
  by_moving();
  by_cloning();
  by_mutating();
}

fn by_moving() {
  let hello = "hello ".to_string();
  let world = "world!";

  // Moving hello into a new variable
  let hello_world = hello + world;
  // Hello CANNOT be used anymore
  println!("{}", hello_world); // Prints "hello world!"
}

fn by_cloning() {
  let hello = "hello ".to_string();
  let world = "world!";

  // Creating a copy of hello and moving it into a new variable
  let hello_world = hello.clone() + world;
  // Hello can still be used
  println!("{}", hello_world); // Prints "hello world!"
}

fn by_mutating() {
  let mut hello = "hello ".to_string();
  let world = "world!";

  // hello gets modified in place
  hello.push_str(world);
  // hello is both usable and modifiable
  println!("{}", hello); // Prints "hello world!"
}
