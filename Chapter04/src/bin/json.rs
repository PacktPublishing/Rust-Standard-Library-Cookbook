extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::fs::OpenOptions;

#[derive(Serialize, Deserialize)]
struct PetOwner {
    name: String,
    age: u8,
    pets: Vec<Pet>,
}

#[derive(Serialize, Deserialize)]
struct Pet {
    name: String,
    species: AllowedSpecies,
    // It is usual for many JSON keys to be optional
    age: Option<u8>,
    colour: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
enum AllowedSpecies {
    Dog,
    Turtle,
    Cat,
}

fn main() {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("pet_owner.json")
        .expect("failed to create JSON file");

    let buf_writer = BufWriter::new(&file);
    write_json(buf_writer).expect("Failed to write JSON");

    let mut buf_reader = BufReader::new(&file);
    buf_reader
        .seek(SeekFrom::Start(0))
        .expect("Failed to jump to the beginning of the JSON file");
    read_json(buf_reader).expect("Failed to read JSON");
}


fn write_json<W>(mut writer: W) -> serde_json::Result<()>
where
    W: Write,
{
    let pet_owner = PetOwner {
        name: "John".to_string(),
        age: 23,
        pets: vec![
            Pet {
                name: "Waldo".to_string(),
                species: AllowedSpecies::Dog,
                age: Some(2),
                colour: None,
            },
            Pet {
                name: "Speedy".to_string(),
                species: AllowedSpecies::Turtle,
                age: Some(47),
                colour: Some("Green".to_string()),
            },
            Pet {
                name: "Meows".to_string(),
                species: AllowedSpecies::Cat,
                age: None,
                colour: Some("Orange".to_string()),
            },
        ],
    };

    let json = serde_json::to_string(&pet_owner)?;
    writer
        .write_all(json.as_bytes())
        .expect("Failed to write file");
    Ok(())
}

fn read_json<R>(mut reader: R) -> serde_json::Result<()>
where
    R: Read,
{
    let mut json = String::new();
    reader
        .read_to_string(&mut json)
        .expect("Failed to read TOML");
    let pet_owner: PetOwner = serde_json::from_str(&json)?;

    println!("Pet owner profile:");
    println!("  Name: {}", pet_owner.name);
    println!("  Age: {}", pet_owner.age);

    println!("\nPets:");
    for pet in pet_owner.pets {
        println!("  Name: {}", pet.name);
        println!("  Species: {:?}", pet.species);
        if let Some(age) = pet.age {
            println!("  Age: {}", age);
        }
        if let Some(colour) = pet.colour {
            println!("  Colour: {}", colour);
        }
        println!();
    }
    Ok(())
}
