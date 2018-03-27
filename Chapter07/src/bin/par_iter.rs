extern crate rayon;
use rayon::prelude::*;

fn main() {
    let legend = "Did you ever hear the tragedy of Darth Plagueis The Wise?";
    let words: Vec<_> = legend.split_whitespace().collect();

    // The following will execute in parallel,
    // so the exact order of execution is not foreseeable
    words.par_iter().for_each(|val| println!("{}", val));

    // par_iter can do everything that a normal iterator does, but
    // in parallel. This way you can easily parallelize any algorithm
    let words_with_a: Vec<_> = words
        .par_iter()
        .filter(|val| val.find('a').is_some())
        .collect();

    println!(
        "The following words contain the letter 'a': {:?}",
        words_with_a
    );
}
