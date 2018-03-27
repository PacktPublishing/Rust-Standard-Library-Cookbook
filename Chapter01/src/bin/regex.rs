extern crate regex;

fn main() {
    use regex::Regex;
    // Beginning a string with 'r' makes it a raw string,
    // in which you don't need to escape any symbols
    let date_regex = Regex::new(r"^\d{2}.\d{2}.\d{4}$").expect("Failed to create regex");
    let date = "15.10.2017";
    // Check for a match
    let is_date = date_regex.is_match(date);
    println!("Is '{}' a date? {}", date, is_date);

    // Let's use capture groups now
    let date_regex = Regex::new(r"(\d{2}).(\d{2}).(\d{4})").expect("Failed to create regex");
    let text_with_dates = "Alan Turing was born on 23.06.1912 and died on 07.06.1954. \
                           A movie about his life called 'The Imitation Game' came out on 14.11.2017";
    // Iterate over the matches
    for cap in date_regex.captures_iter(text_with_dates) {
        println!("Found date {}", &cap[0]);
        println!("Year: {} Month: {} Day: {}", &cap[3], &cap[2], &cap[1]);
    }
    // Replace the date format
    println!("Original text:\t\t{}", text_with_dates);
    let text_with_indian_dates = date_regex.replace_all(text_with_dates, "$1-$2-$3");
    println!("In indian format:\t{}", text_with_indian_dates);

    // Replacing groups is easier when we name them
    // ?P<somename> gives a capture group a name
    let date_regex = Regex::new(r"(?P<day>\d{2}).(?P<month>\d{2}).(?P<year>\d{4})")
        .expect("Failed to create regex");
    let text_with_american_dates = date_regex.replace_all(text_with_dates, "$month/$day/$year");
    println!("In american format:\t{}", text_with_american_dates);
    let rust_regex = Regex::new(r"(?i)rust").expect("Failed to create regex");
    println!("Do we match RuSt? {}", rust_regex.is_match("RuSt"));
    use regex::RegexBuilder;
    let rust_regex = RegexBuilder::new(r"rust")
        .case_insensitive(true)
        .build()
        .expect("Failed to create regex");
    println!("Do we still match RuSt? {}", rust_regex.is_match("RuSt"));
}