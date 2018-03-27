use std::collections::HashSet;

fn main() {
    // Most of the interface of HashSet
    // is the same as HashMap, just without
    // the methods that handle values
    let mut books = HashSet::new();
    books.insert("Harry Potter and the Philosopher's Stone");
    books.insert("The Name of the Wind");
    books.insert("A Game of Thrones");

    // A HashSet will ignore duplicate entries
    // but will return if an entry is new or not
    let is_new = books.insert("The Lies of Locke Lamora");
    if is_new {
        println!("We've just added a new book!");
    }

    let is_new = books.insert("A Game of Thrones");
    if !is_new {
        println!("Sorry, we already had that book in store");
    }

    // Check if it contains a key
    if !books.contains("The Doors of Stone") {
        println!("We sadly don't have that book yet");
    }

    // Remove an entry
    let was_removed = books.remove("The Darkness that comes before");
    if !was_removed {
        println!("Couldn't remove book; We didn't have it to begin with");
    }
    let was_removed = books.remove("Harry Potter and the Philosopher's Stone");
    if was_removed {
        println!("Oops, we lost a book");
    }

    // Compare two HashSets

    let one_to_five: HashSet<_> = (1..6).collect();
    let five_to_ten: HashSet<_> = (5..11).collect();
    let one_to_ten: HashSet<_> = (1..11).collect();
    let three_to_eight: HashSet<_> = (3..9).collect();

    // Check if two HashSets have no elements in common
    let is_disjoint = one_to_five.is_disjoint(&five_to_ten);
    println!(
        "is {:?} disjoint from {:?}?: {}",
        one_to_five,
        five_to_ten,
        is_disjoint
    );
    let is_disjoint = one_to_five.is_disjoint(&three_to_eight);
    println!(
        "is {:?} disjoint from {:?}?: {}",
        one_to_five,
        three_to_eight,
        is_disjoint
    );

    // Check if a HashSet is fully contained in another
    let is_subset = one_to_five.is_subset(&five_to_ten);
    println!(
        "is {:?} a subset of {:?}?: {}",
        one_to_five,
        five_to_ten,
        is_subset
    );
    let is_subset = one_to_five.is_subset(&one_to_ten);
    println!(
        "is {:?} a subset of {:?}?: {}",
        one_to_five,
        one_to_ten,
        is_subset
    );

    // Check if a HashSet fully contains another
    let is_superset = three_to_eight.is_superset(&five_to_ten);
    println!(
        "is {:?} a superset of {:?}?: {}",
        three_to_eight,
        five_to_ten,
        is_superset
    );
    let is_superset = one_to_ten.is_superset(&five_to_ten);
    println!(
        "is {:?} a superset of {:?}?: {}",
        one_to_ten,
        five_to_ten,
        is_superset
    );

    // Join two HashSets in various ways

    // Get the values that are in the first HashSet
    // but not in the second
    let difference = one_to_five.difference(&three_to_eight);
    println!(
        "The difference between {:?} and {:?} is {:?}",
        one_to_five,
        three_to_eight,
        difference
    );

    // Get the values that are in either HashSets, but not in both
    let symmetric_difference = one_to_five.symmetric_difference(&three_to_eight);
    println!(
        "The symmetric difference between {:?} and {:?} is {:?}",
        one_to_five,
        three_to_eight,
        symmetric_difference
    );

    // Get the values that are in both HashSets
    let intersection = one_to_five.intersection(&three_to_eight);
    println!(
        "The intersection difference between {:?} and {:?} is {:?}",
        one_to_five,
        three_to_eight,
        intersection
    );

    // Get all values in both HashSets
    let union = one_to_five.union(&three_to_eight);
    println!(
        "The union difference between {:?} and {:?} is {:?}",
        one_to_five,
        three_to_eight,
        union
    );
}
