use std::fs::File;
use std::io::BufReader;
use std::result::Result;
use std::error::Error;
use std::io::Read;
use std::fmt::Debug;

#[derive(Debug)]
struct Node<T> {
    data: T,
    child_nodes: Option<(BoxedNode<T>, BoxedNode<T>)>,
}
type BoxedNode<T> = Box<Node<T>>;

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            child_nodes: None,
        }
    }

    fn is_leaf(&self) -> bool {
        self.child_nodes.is_none()
    }

    fn add_child_nodes(&mut self, a: Node<T>, b: Node<T>) {
        assert!(
            self.is_leaf(),
            "Tried to add child_nodes to a node that is not a leaf"
        );
        self.child_nodes = Some((Box::new(a), Box::new(b)));
    }
}

// Boxes enable you to use traditional OOP polymorph
trait Animal: Debug {
    fn sound(&self) -> &'static str;
}

#[derive(Debug)]
struct Dog;
impl Animal for Dog {
    fn sound(&self) -> &'static str {
        "Woof!"
    }
}

#[derive(Debug)]
struct Cat;
impl Animal for Cat {
    fn sound(&self) -> &'static str {
        "Meow!"
    }
}

fn main() {
    let mut root = Node::new(12);
    root.add_child_nodes(Node::new(3), Node::new(-24));
    root.child_nodes
        .as_mut()
        .unwrap()
        .0
        .add_child_nodes(Node::new(0), Node::new(1803));
    println!("Our binary tree looks like this: {:?}", root);

    // Polymorphism
    let mut zoo: Vec<Box<Animal>> = Vec::new();
    zoo.push(Box::new(Dog {}));
    zoo.push(Box::new(Cat {}));
    for animal in zoo {
        println!("{:?} says {}", animal, animal.sound());
    }

    for word in caps_words_iter("do you feel lucky, punk‽") {
        println!("{}", word);
    }

    // Assuming a file called number.txt exists
    let num = read_file_as_number("number.txt").expect("Failed read the file as a number");
    println!("number.txt contains the number {}", num);

    // Dynamically composing functions
    let multiplier = create_multiplier(23);
    let result = multiplier(3);
    println!("23 * 3 = {}", result);
}

// Via trait objects we can return any iterator
fn caps_words_iter<'a>(text: &'a str) -> Box<Iterator<Item = String> + 'a> {
    // Return an iterator over every word converted into ALL_CAPS
    Box::new(text.trim().split(' ').map(|word| word.to_uppercase()))
}

// Same goes for errors
fn read_file_as_number(filename: &str) -> Result<i32, Box<Error>> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    let number: i32 = content.parse()?;
    Ok(number)
}

fn create_multiplier(a: i32) -> Box<Fn(i32) -> i32> {
    Box::new(move |b| a * b)
}
