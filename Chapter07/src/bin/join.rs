extern crate rayon;

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn perimeter(&self) -> u32 {
        2 * (self.height + self.width)
    }
}

fn main() {
    let rect = Rectangle {
        height: 30,
        width: 20,
    };
    // rayon::join makes closures run potentially in parallel and
    // returns their returned values in a tuple
    let (area, perimeter) = rayon::join(|| rect.area(), || rect.perimeter());
    println!("{:?}", rect);
    println!("area: {}", area);
    println!("perimeter: {}", perimeter);

    let fib = fibonacci(6);
    println!("The sixth number in the fibonacci sequence is {}", fib);
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        n
    } else {
        // rayon::join can really shine in recursive functions
        let (a, b) = rayon::join(|| fibonacci(n - 1), || fibonacci(n - 2));
        a + b
    }
}
