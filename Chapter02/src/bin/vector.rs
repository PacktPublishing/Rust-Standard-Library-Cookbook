fn main() {
    // Create a vector with some elements
    let fruits = vec!["apple", "tomato", "pear"];
    // A vector cannot be directly printed
    // But we can debug-print it
    println!("fruits: {:?}", fruits);

    // Create an empty vector and fill it
    let mut fruits = Vec::new();
    fruits.push("apple");
    fruits.push("tomato");
    fruits.push("pear");
    println!("fruits: {:?}", fruits);

    // Remove the last element
    let last = fruits.pop();
    if let Some(last) = last {
        println!("Removed {} from {:?}", last, fruits);
    }

    // Insert an element into the middle of the vector
    fruits.insert(1, "grape");
    println!("fruits after insertion: {:?}", fruits);

    // Swap two elements
    fruits.swap(0, 1);
    println!("fruits after swap: {:?}", fruits);

    // Access the first and last elements
    let first = fruits.first();
    if let Some(first) = first {
        println!("First fruit: {}", first);
    }
    let last = fruits.last();
    if let Some(last) = last {
        println!("Last fruit: {}", last);
    }

    // Access arbitrary elements
    let second = fruits.get(1);
    if let Some(second) = second {
        println!("Second fruit: {}", second);
    }
    // Access arbitrary elements without bonds checking
    let second = fruits[1];
    println!("Second fruit: {}", second);



    // Initialize the vector with a value
    // Here, we fill our vector with five zeroes
    let bunch_of_zeroes = vec![0; 5];
    println!("bunch_of_zeroes: {:?}", bunch_of_zeroes);

    // Remove some item and shift all that come after
    // into place
    let mut nums = vec![1, 2, 3, 4];
    let second_num = nums.remove(1);
    println!("Removed {} from {:?}", second_num, nums);

    // Filter the vector in place
    let mut names = vec!["Aaron", "Felicia", "Alex", "Daniel"];
    // Only keep names starting with 'A'
    names.retain(|name| name.starts_with('A'));
    println!("Names starting with A: {:?}", names);

    // Check if the vector contains an element
    println!("Does 'names' contain \"Alex\"? {}", names.contains(&"Alex"));



    // Remove consecutive(!) duplicates
    let mut nums = vec![1, 2, 2, 3, 4, 4, 4, 5];
    nums.dedup();
    println!("Deduped, pre-sorted nums: {:?}", nums);

    // Be careful if your data is not sorted!
    let mut nums = vec![2, 1, 4, 2, 3, 5, 1, 2];
    nums.dedup();
    // Doens't print what you might expect
    println!("Deduped, unsorted nums: {:?}", nums);

    // Sort a vector
    nums.sort();
    println!("Manually sorted nums: {:?}", nums);
    nums.dedup();
    println!("Deduped, sorted nums: {:?}", nums);

    // Reverse a vector
    nums.reverse();
    println!("nums after being reversed: {:?}", nums);

    // Create a consuming iterator over a range
    let mut alphabet = vec!['a', 'b', 'c'];
    print!("The first two letters of the alphabet are: ");
    for letter in alphabet.drain(..2) {
        print!("{} ", letter);
    }
    println!();
    // The drained elements are no longer in the vector
    println!("alphabet after being drained: {:?}", alphabet);


    // Check if a vector is empty
    let mut fridge = vec!["Beer", "Leftovers", "Mayonaise"];
    println!("Is the fridge empty {}", fridge.is_empty());
    // Remove all elements
    fridge.clear();
    println!("Is the fridge now empty? {}", fridge.is_empty());

    // Split a vector into two pieces
    let mut colors = vec!["red", "green", "blue", "yellow"];
    println!("colors before splitting: {:?}", colors);
    let half = colors.len() / 2;
    let mut second_half = colors.split_off(half);
    println!("colors after splitting: {:?}", colors);
    println!("second_half: {:?}", second_half);

    // Put two vectors together
    colors.append(&mut second_half);
    println!("colors after appending: {:?}", colors);
    // This empties the second vector
    println!("second_half after appending: {:?}", second_half);

    // Splice a vector
    // Webdevs, you're gonna remember this from JavaScript
    let mut stuff = vec!["1", "2", "3", "4", "5", "6"];
    println!("Original stuff: {:?}", stuff);
    let stuff_to_insert = vec!["a", "b", "c"];
    let removed_stuff: Vec<_> = stuff.splice(1..4, stuff_to_insert).collect();
    println!("Spliced stuff: {:?}", stuff);
    println!("Removed stuff: {:?}", removed_stuff);


    // Optimizations:
    // Initialize the vector with a certain capacity
    let mut large_vec: Vec<i32> = Vec::with_capacity(1_000_000);
    println!("large_vec after creation:");
    println!("len:\t\t{}", large_vec.len());
    println!("capacity:\t{}", large_vec.capacity());

    // Shrink the vector as close as possible to its length
    large_vec.shrink_to_fit();
    println!("large_vec after shrinking:");
    println!("len:\t\t{}", large_vec.len());
    println!("capacity:\t{}", large_vec.capacity());

    // Remove some item, replacing it with the last
    let mut nums = vec![1, 2, 3, 4];
    let second_num = nums.swap_remove(1);
    // This changes the order, but works in O(1)
    println!("Removed {} from {:?}", second_num, nums);
}
