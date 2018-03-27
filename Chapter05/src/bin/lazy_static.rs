#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::sync::RwLock;

// Global immutable static
lazy_static! {
    static ref CURRENCIES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("EUR", "Euro");
        m.insert("USD", "U.S. Dollar");
        m.insert("CHF", "Swiss Francs");
        m
    };
}

// Global mutable static
lazy_static! {
    static ref CLIENTS: RwLock<Vec<String>> = RwLock::new(Vec::new());
}

// Local static
fn extract_day(date: &str) -> Option<&str> {
    // lazy static objects are perfect for
    // compiling regexes only once
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(\d{2}).(\d{2}).(\d{4})")
            .expect("Failed to create regex");
    }
    RE.captures(date)
        .and_then(|cap| cap.get(1).map(|day| day.as_str()))
}

fn main() {
    // The first access to CURRENCIES initializes it
    let usd = CURRENCIES.get("USD");
    if let Some(usd) = usd {
        println!("USD stands for {}", usd);
    }

    // All accesses will now refer to the same,
    // already constructed object
    if let Some(chf) = CURRENCIES.get("CHF") {
        println!("CHF stands for {}", chf);
    }

    // Mutable the global static
    CLIENTS
        .write()
        .expect("Failed to unlock clients for writing")
        .push("192.168.0.1".to_string());

    // Get an immutable reference to the global static
    let clients = CLIENTS
        .read()
        .expect("Failed to unlock clients for reading");
    let first_client = clients.get(0).expect("CLIENTS is empty");
    println!("The first client is: {}", first_client);

    let date = "12.01.2018";
    // The static object is nicely hidden inside
    // the definition of extract_day()
    if let Some(day) = extract_day(date) {
        println!("The date \"{}\" contains the day \"{}\"", date, day);
    }
}
