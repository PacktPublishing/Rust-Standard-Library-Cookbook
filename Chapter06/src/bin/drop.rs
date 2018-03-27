use std::fmt::Debug;

struct CustomSmartPointer<D>
where
    D: Debug,
{
    data: D,
}

impl<D> CustomSmartPointer<D>
where
    D: Debug,
{
    fn new(data: D) -> Self {
        CustomSmartPointer { data }
    }
}

impl<D> Drop for CustomSmartPointer<D>
where
    D: Debug,
{
    // This will automatically be called when a variable is dropped
    // It cannot be called manually
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{:?}`", self.data);
    }
}

fn main() {
    let a = CustomSmartPointer::new("A");
    let b = CustomSmartPointer::new("B");
    let c = CustomSmartPointer::new("C");
    let d = CustomSmartPointer::new("D");

    // The next line would cause a compiler error,
    // as destructors cannot be explicitely called
    // c.drop();

    // The correct way to drop variables early is the following:
    std::mem::drop(c);
}
