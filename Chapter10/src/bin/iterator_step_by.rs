#![feature(iterator_step_by)]

fn main() {
    // step_by() will start on the first element of an iterator,
    // but then skips a certain number of elements on every iteration
    let even_numbers: Vec<_> = (0..100).step_by(2).collect();
    println!("The first one hundred even numbers: {:?}", even_numbers);

    // step_by() will always start at the beginning.
    // If you need to skip the first few elements as well, use skip()
    let some_data = ["Andrei", "Romania", "Giuseppe", "Italy", "Susan", "Britain"];
    let countries: Vec<_> = some_data.iter().skip(1).step_by(2).collect();
    println!("Countries in the data: {:?}", countries);

    let grouped_stream = "Aaron 182cm 70kg Alice 160cm 90kg Bob 197cm 83kg";
    let weights: Vec<_> = grouped_stream
        .split_whitespace()
        .skip(2)
        .step_by(3)
        .collect();
    println!("The weights of the people are: {:?}", weights);
}
