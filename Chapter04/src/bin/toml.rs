#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::fs::OpenOptions;

#[derive(Serialize, Deserialize)]
struct Preferences {
    person: Person,
    language: Language,
    privacy: Privacy,
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct Language {
    display: String,
    autocorrect: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
struct Privacy {
    share_anonymous_statistics: bool,
    public_name: bool,
    public_email: bool,
}

fn main() {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("preferences.toml")
        .expect("failed to create TOML file");

    let buf_writer = BufWriter::new(&file);
    write_toml(buf_writer).expect("Failed to write TOML");

    let mut buf_reader = BufReader::new(&file);
    buf_reader
        .seek(SeekFrom::Start(0))
        .expect("Failed to jump to the beginning of the TOML file");
    read_toml(buf_reader).expect("Failed to read TOML");
}

type SerializeResult<T> = Result<T, toml::ser::Error>;
fn write_toml<W>(mut writer: W) -> SerializeResult<()>
where
    W: Write,
{
    let preferences = Preferences {
        person: Person {
            name: "Jan Nils Ferner".to_string(),
            email: "jn_ferner@hotmail.de".to_string(),
        },
        language: Language {
            display: "en-GB".to_string(),
            autocorrect: Some(vec![
                "en-GB".to_string(),
                "en-US".to_string(),
                "de-CH".to_string(),
            ]),
        },
        privacy: Privacy {
            share_anonymous_statistics: false,
            public_name: true,
            public_email: true,
        },
    };

    let toml = toml::to_string(&preferences)?;
    writer
        .write_all(toml.as_bytes())
        .expect("Failed to write file");
    Ok(())
}

type DeserializeResult<T> = Result<T, toml::de::Error>;
fn read_toml<R>(mut reader: R) -> DeserializeResult<()>
where
    R: Read,
{
    let mut toml = String::new();
    reader
        .read_to_string(&mut toml)
        .expect("Failed to read TOML");
    let preferences: Preferences = toml::from_str(&toml)?;

    println!("Personal data:");
    let person = &preferences.person;
    println!("  Name: {}", person.name);
    println!("  Email: {}", person.email);

    println!("\nLanguage preferences:");
    let language = &preferences.language;
    println!("  Display language: {}", language.display);
    println!("  Autocorrect priority: {:?}", language.autocorrect);


    println!("\nPrivacy settings:");
    let privacy = &preferences.privacy;
    println!(
        "  Share anonymous usage statistics: {}",
        privacy.share_anonymous_statistics
    );
    println!("  Display name publically: {}", privacy.public_name);
    println!("  Display email publically: {}", privacy.public_email);

    Ok(())
}
