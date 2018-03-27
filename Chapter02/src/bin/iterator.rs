fn main() {
    let names = vec!["Joe", "Miranda", "Alice"];
    // Iterators can be accessed in many ways.
    // Nearly all collections implement .iter() for this purpose
    let mut iter = names.iter();
    // A string itself is not iterable, but its characters are
    let mut alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
    // Ranges are also (limited) iterators
    let nums = 0..10;
    // You can even create infinite iterators!
    let all_nums = 0..;

    // As the name says, you can iterate over iterators
    // This will consume the iterator
    for num in nums {
        print!("{} ", num);
    }
    // nums is no longer usable
    println!();

    // Get the index of the current item
    for (index, letter) in "abc".chars().enumerate() {
        println!("#{}. letter in the alphabet: {}", index + 1, letter);
    }

    // going through an iterator, step by step
    if let Some(name) = iter.next() {
        println!("First name: {}", name);
    }
    if let Some(name) = iter.next() {
        println!("Second name: {}", name);
    }
    if let Some(name) = iter.next() {
        println!("Third name: {}", name);
    }
    if iter.next().is_none() {
        println!("No names left");
    }

    // Arbitrary access to an item in the iterator
    let letter = alphabet.nth(3);
    if let Some(letter) = letter {
        println!("the fourth letter in the alphabet is: {}", letter);
    }
    // This works by consuming all items up to a point
    let current_first = alphabet.nth(0);
    if let Some(current_first) = current_first {
        // This will NOT print 'A'
        println!(
            "The first item in the iterator is currently: {}",
            current_first
        );
    }
    let current_first = alphabet.nth(0);
    if let Some(current_first) = current_first {
        println!(
            "The first item in the iterator is currently: {}",
            current_first
        );
    }

    // Accessing the last item; This will
    // consume the entire iterator
    let last_letter = alphabet.last();
    if let Some(last_letter) = last_letter {
        println!("The last letter of the alphabet is: {}", last_letter);
    }

    // Collect iterators into collections
    // This requires an anotation of which collection we want
    // The following two are equivalent:
    let nums: Vec<_> = (1..10).collect();
    println!("nums: {:?}", nums);
    let nums = (1..10).collect::<Vec<_>>();
    println!("nums: {:?}", nums);

    // Change which items are being iterated over

    // Taking only the first n items
    // This is often used to make an infinite iterator finite
    let nums: Vec<_> = all_nums.take(5).collect();
    println!("The first five numbers are: {:?}", nums);

    // Skip the first few items
    let nums: Vec<_> = (0..11).skip(2).collect();
    println!("The last 8 letters in a range from zero to 10: {:?}", nums);

    // take and skip accept predicates in the form of
    // take_while and skip_while
    let nums: Vec<_> = (0..).take_while(|x| x * x < 50).collect();
    println!(
        "All positive numbers that are less than 50 when squared: {:?}",
        nums
    );

    // This is useful to filter an already sorted vector
    let names = ["Alfred", "Andy", "Jose", "Luke"];
    let names: Vec<_> = names.iter().skip_while(|x| x.starts_with('A')).collect();
    println!("Names that don't start with 'A': {:?}", names);

    // Filtering iterators
    let countries = [
        "U.S.A.", "Germany", "France", "Italy", "India", "Pakistan", "Burma"
    ];
    let countries_with_i: Vec<_> = countries
        .iter()
        .filter(|country| country.contains('i'))
        .collect();
    println!(
        "Countries containing the letter 'i': {:?}",
        countries_with_i
    );

    // Checking if an iterator contains an element

    // Find the first element that satisfies a condition
    if let Some(country) = countries.iter().find(|country| country.starts_with('I')) {
        println!("First country starting with the letter 'I': {}", country);
    }

    // Don't get the searched item but rather its index
    if let Some(pos) = countries
        .iter()
        .position(|country| country.starts_with('I'))
    {
        println!("It's index is: {}", pos);
    }

    // Check if at least one item satisfies a condition
    let are_any = countries.iter().any(|country| country.len() == 5);
    println!(
        "Is there at least one country that has exactly five letters? {}",
        are_any
    );

    // Check if ALL items satisfy a condition
    let are_all = countries.iter().all(|country| country.len() == 5);
    println!("Do all countries have exactly five letters? {}", are_all);

    // Useful operations for numeric items
    let sum: i32 = (1..11).sum();
    let product: i32 = (1..11).product();
    println!(
        "When operating on the first ten positive numbers\n\
         their sum is {} and\n\
         their product is {}.",
        sum, product
    );

    let max = (1..11).max();
    let min = (1..11).min();
    if let Some(max) = max {
        println!("They have a highest number, and it is {}", max);
    }
    if let Some(min) = min {
        println!("They have a smallest number, and it is {}", min);
    }

    // Combine iterators

    // Combine an iterator with itself, making it infinite
    // When it reaches its end, it starts again
    let some_numbers: Vec<_> = (1..4).cycle().take(10).collect();
    // Reader exercise: Try to guess what this will print
    println!("some_numbers: {:?}", some_numbers);

    // Combine two iterators by putting them after another
    let some_numbers: Vec<_> = (1..4).chain(10..14).collect();
    println!("some_numbers: {:?}", some_numbers);

    // Zip two iterators together by grouping their first items
    // together, their second items together, etc.
    let swiss_post_codes = [8957, 5000, 5034];
    let swiss_towns = ["Spreitenbach", "Aarau", "Suhr"];
    let zipped: Vec<_> = swiss_post_codes.iter().zip(swiss_towns.iter()).collect();
    println!("zipped: {:?}", zipped);

    // Because zip is lazy, you can use two infine ranges
    let zipped: Vec<_> = (b'A'..)
        .zip(1..)
        .take(10)
        .map(|(ch, num)| (ch as char, num))
        .collect();
    println!("zipped: {:?}", zipped);

    // Apply functions to all items

    // Change the items' types
    let numbers_as_strings: Vec<_> = (1..11).map(|x| x.to_string()).collect();
    println!("numbers_as_strings: {:?}", numbers_as_strings);

    // Access all items
    println!("First ten squares:");
    (1..11).for_each(|x| print!("{} ", x));
    println!();

    // filter and map items at the same time!
    let squares: Vec<_> = (1..50)
        .filter_map(|x| if x % 3 == 0 { Some(x * x) } else { None })
        .collect();
    println!(
        "Squares of all numbers under 50 that are divisible by 3: {:?}",
        squares
    );

    // The real strength of iterators comes from combining them

    // Retrieve the entire alphabet in lower and uppercase:
    let alphabet: Vec<_> = (b'A' .. b'z' + 1) // Start as u8
        .map(|c| c as char)            // Convert all to chars
        .filter(|c| c.is_alphabetic()) // Filter only alphabetic chars
        .collect(); // Collect as Vec<char>
    println!("alphabet: {:?}", alphabet);
}
