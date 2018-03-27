extern crate rand;

fn main() {
    // random_num1 will be any integer between
    // std::i32::MIN and std::i32::MAX
    let random_num1 = rand::random::<i32>();
    println!("random_num1: {}", random_num1);
    let random_num2: i32 = rand::random();
    println!("random_num2: {}", random_num2);
    // The initialization of random_num1 and random_num2
    // is equivalent.

    // Every primitive data type can be randomized
    let random_char = rand::random::<char>();
    // Altough random_char will probably not be
    // representable on most operating systems
    println!("random_char: {}", random_char);


    use rand::Rng;
    // We can use a reusable generator
    let mut rng = rand::thread_rng();
    // This is equivalent to rand::random()
    if rng.gen() {
        println!("This message has a 50-50 chance of being printed");
    }
    // A generator enables us to use ranges
    // random_num3 will be between 0 and 9
    let random_num3 = rng.gen_range(0, 10);
    println!("random_num3: {}", random_num3);

    // random_float will be between 0.0 and 0.999999999999...
    let random_float = rng.gen_range(0.0, 1.0);
    println!("random_float: {}", random_float);

    // Per default, the generator uses a uniform distribution,
    // which should be good enough for nearly all of your
    // use cases. If you require a particular distribution,
    // you specify it when creating the generator:
    let mut chacha_rng = rand::ChaChaRng::new_unseeded();
    let random_chacha_num = chacha_rng.gen::<i32>();
    println!("random_chacha_num: {}", random_chacha_num);
}
