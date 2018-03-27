use std::ops::MulAssign;
use std::fmt::Display;

// This structure doubles all elements it stores
#[derive(Debug)]
struct DoubleVec<T>(Vec<T>);


// Allowing conversion from a Vec<T>,
// where T is multipliable with an integer
impl<T> From<Vec<T>> for DoubleVec<T>
where
    T: MulAssign<i32>,
{
    fn from(mut vec: Vec<T>) -> Self {
        for elem in &mut vec {
            *elem *= 2;
        }
        DoubleVec(vec)
    }
}

// Allowing conversion from a slice of Ts
// where T is again multipliable with an integer
impl<'a, T> From<&'a [T]> for DoubleVec<T>
where
    T: MulAssign<i32> + Clone,
{
    fn from(slice: &[T]) -> Self {
        // Vec<T: MulAssign<i32>> automatically
        // implements Into<DoubleVec<T>>
        slice.to_vec().into()
    }
}

// Allowing conversion from a &DoubleVec<T> to a &Vec<T>
impl<T> AsRef<Vec<T>> for DoubleVec<T> {
    fn as_ref(&self) -> &Vec<T> {
        &self.0
    }
}


fn main() {
    // The following three are equivalent
    let hello_world = "Hello World".to_string();
    let hello_world: String = "Hello World!".into();
    let hello_world = String::from("Hello World!");

    // Vec<u8> implements From<&str>
    // so hello_world_bytes has the value b"Hello World!"
    let hello_world_bytes: Vec<u8> = "Hello World!".into();
    let hello_world_bytes = Vec::<u8>::from("Hello World!");

    // We can convert a Vec<T: MulAssign<i32>> into a DoubleVec
    let vec = vec![1, 2, 3];
    let double_vec = DoubleVec::from(vec);
    println!("Creating a DoubleVec from a Vec: {:?}", double_vec);

    // Vec<T: MulAssign<i32>> automatically implements Into<DoubleVec<T>>
    let vec = vec![1, 2, 3];
    let double_vec: DoubleVec<_> = vec.into();
    println!("Converting a Vec into a DoubleVec: {:?}", double_vec);

    // A reference to DoubleVec can be converted to a reference to Vec
    // Which in turn dereferences to a slice
    print_elements(double_vec.as_ref());

    // The standard library provides From<T> for Option<T>
    // You can design your API in an ergonomic way thanks to this
    easy_public_func(Some(1337), Some(123), None);
    ergonomic_public_func(1337, 123, None);
}


fn print_elements<T>(slice: &[T])
where
    T: Display,
{
    for elem in slice {
        print!("{} ", elem);
    }
    println!();
}


// Easily written but cumbersome to use
fn easy_public_func(foo: Option<i32>, bar: Option<i32>, baz: Option<i32>) {
    println!(
        "easy_public_func = foo: {:?}, bar: {:?}, baz: {:?}",
        foo,
        bar,
        baz
    );
}


// This is quite some extra typing, so it's only worth to do for
// public functions with many optional parameters
fn ergonomic_public_func<Foo, Bar, Baz>(foo: Foo, bar: Bar, baz: Baz)
where
    Foo: Into<Option<i32>>,
    Bar: Into<Option<i32>>,
    Baz: Into<Option<i32>>,
{
    let foo: Option<i32> = foo.into();
    let bar: Option<i32> = bar.into();
    let baz: Option<i32> = baz.into();

    println!(
        "ergonomic_public_func = foo: {:?}, bar: {:?}, baz: {:?}",
        foo,
        bar,
        baz
    );
}
