use std::{error, fmt, io, num, result};
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug)]
// This is going to be our custom Error type
enum AgeReaderError {
    Io(io::Error),
    Parse(num::ParseIntError),
    NegativeAge(),
}

// It is common to alias Result in an Error module
type Result<T> = result::Result<T, AgeReaderError>;

impl error::Error for AgeReaderError {
    fn description(&self) -> &str {
        // Defer to the existing description if possible
        match *self {
            AgeReaderError::Io(ref err) => err.description(),
            AgeReaderError::Parse(ref err) => err.description(),
            // Descriptions should be as short as possible
            AgeReaderError::NegativeAge() => "Age is negative",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        // Return the underlying error, if any
        match *self {
            AgeReaderError::Io(ref err) => Some(err),
            AgeReaderError::Parse(ref err) => Some(err),
            AgeReaderError::NegativeAge() => None,
        }
    }
}

impl fmt::Display for AgeReaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write a detailed description of the problem
        match *self {
            AgeReaderError::Io(ref err) => write!(f, "IO error: {}", err),
            AgeReaderError::Parse(ref err) => write!(f, "Parse error: {}", err),
            AgeReaderError::NegativeAge() => write!(f, "Logic error: Age cannot be negative"),
        }
    }
}

// Implement From<T> for every sub-error
impl From<io::Error> for AgeReaderError {
    fn from(err: io::Error) -> AgeReaderError {
        AgeReaderError::Io(err)
    }
}

impl From<num::ParseIntError> for AgeReaderError {
    fn from(err: num::ParseIntError) -> AgeReaderError {
        AgeReaderError::Parse(err)
    }
}

fn main() {
    // Assuming a file called age.txt exists
    const FILENAME: &str = "age.txt";
    let result = read_age(FILENAME);
    match result {
        Ok(num) => println!("{} contains the age {}", FILENAME, num),
        Err(AgeReaderError::Io(err)) => eprintln!("Failed to open the file {}: {}", FILENAME, err),
        Err(AgeReaderError::Parse(err)) => eprintln!(
            "Failed to read the contents of {} as a number: {}",
            FILENAME, err
        ),
        Err(AgeReaderError::NegativeAge()) => eprintln!("The age in the file is negative"),
    }
}

// Read an age out of a file
fn read_age(filename: &str) -> Result<i32> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    let age: i32 = content.trim().parse()?;
    if age.is_positive() {
        Ok(age)
    } else {
        Err(AgeReaderError::NegativeAge())
    }
}
