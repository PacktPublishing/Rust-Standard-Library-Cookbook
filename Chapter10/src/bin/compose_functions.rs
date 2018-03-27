#![feature(conservative_impl_trait)]

// The compose! macro takes a variadic amount of closures and returns
// a closure that applies them all one after another
macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $ ($tail:expr), +) => {
        compose_two($head, compose!($ ($tail), +))
    };
}

// compose_two is a helper function used to
// compose only two closures into one
fn compose_two<FunOne, FunTwo, Input, Intermediate, Output>(
    fun_one: FunOne,
    fun_two: FunTwo,
) -> impl Fn(Input) -> Output
where
    FunOne: Fn(Input) -> Intermediate,
    FunTwo: Fn(Intermediate) -> Output,
{
    move |x| fun_two(fun_one(x))
}

fn main() {
    let add = |x| x + 2.0;
    let multiply = |x| x * 3.0;
    let divide = |x| x / 4.0;
    // itermediate(x) returns ((x + 2) * 3) / 4
    let intermediate = compose!(add, multiply, divide);

    let subtract = |x| x - 5.0;
    // finally(x) returns (((x + 2) * 3) / 4) - 5
    let finally = compose!(intermediate, subtract);

    println!("(((10 + 2) * 3) / 4) - 5 is: {}", finally(10.0));
}
