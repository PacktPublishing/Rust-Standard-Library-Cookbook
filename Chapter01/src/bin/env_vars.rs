use std::env;

fn main() {
    // We can iterate over all the env vars for the current process
    println!("Listing all env vars:");
    for (key, val) in env::vars() {
        println!("{}: {}", key, val);
    }

    let key = "PORT";
    println!("Setting env var {}", key);
    // Setting an env var for the current process
    env::set_var(key, "8080");

    print_env_var(key);

    // Removing an env var for the current process
    println!("Removing env var {}", key);
    env::remove_var(key);

    print_env_var(key);
}

fn print_env_var(key: &str) {
    // Accessing an env var
    match env::var(key) {
        Ok(val) => println!("{}: {}", key, val),
        Err(e) => println!("Couldn't print env var {}: {}", key, e),
    }
}