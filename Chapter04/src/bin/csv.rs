extern crate csv;


use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::fs::OpenOptions;


fn main() {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("solar_system_compared_to_earth.csv")
        .expect("failed to create csv file");

    let buf_writer = BufWriter::new(&file);
    write_records(buf_writer).expect("Failed to write csv");

    let mut buf_reader = BufReader::new(&file);
    buf_reader
        .seek(SeekFrom::Start(0))
        .expect("Failed to jump to the beginning of the csv");
    read_records(buf_reader).expect("Failed to read csv");
}

fn write_records<W>(writer: W) -> csv::Result<()>
where
    W: Write,
{
    let mut wtr = csv::Writer::from_writer(writer);

    // The header is just a normal record
    wtr.write_record(&["name", "radius", "distance_from_sun", "gravity"])?;

    wtr.write_record(&["Mercury", "0.38", "0.47", "0.38"])?;
    wtr.write_record(&["Venus", "0.95", "0.73", "0.9"])?;
    wtr.write_record(&["Earth", "1", "1", "1"])?;
    wtr.write_record(&["Mars", "0.53", "1.67", "0.38"])?;
    wtr.write_record(&["Jupiter", "11.21", "5.46", "2.53"])?;
    wtr.write_record(&["Saturn", "9.45", "10.12", "1.07"])?;
    wtr.write_record(&["Uranus", "4.01", "20.11", "0.89"])?;
    wtr.write_record(&["Neptune", "3.88", "30.33", "1.14"])?;
    wtr.flush()?;
    Ok(())
}

fn read_records<R>(reader: R) -> csv::Result<()>
where
    R: Read,
{
    let mut rdr = csv::Reader::from_reader(reader);
    println!("Comparing planets in the solar system with the earth");
    println!("where a value of '1' means 'equal to earth'");
    for result in rdr.records() {
        println!("-------");
        let record = result?;
        if let Some(name) = record.get(0) {
            println!("Name: {}", name);
        }
        if let Some(radius) = record.get(1) {
            println!("Radius: {}", radius);
        }
        if let Some(distance) = record.get(2) {
            println!("Distance from sun: {}", distance);
        }
        if let Some(gravity) = record.get(3) {
            println!("Surface gravity: {}", gravity);
        }
    }
    Ok(())
}
