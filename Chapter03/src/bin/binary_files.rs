extern crate byteorder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt, BE, LE};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read};
use std::io::prelude::*;


fn main() {
    let path = "./bar.bin";
    write_dummy_protocol(path).expect("Failed write file");
    let payload = read_protocol(path).expect("Failed to read file");
    print!("The protocol contained the following payload: ");
    for num in payload {
        print!("0x{:X} ", num);
    }
    println!();
}

// Write a simple custom protocol
fn write_dummy_protocol(path: &str) -> io::Result<()> {
    let file = File::create(path)?;
    let mut buf_writer = BufWriter::new(file);

    // Let's say our binary file starts with a magic number
    // to show readers that this is our protocoll
    let magic = b"MyProtocol";
    buf_writer.write_all(magic)?;

    // Now comes another magic value to indicate
    // our endianness
    let endianness = b"LE";
    buf_writer.write_all(endianness)?;

    // Let's fill it with two numbers in u32
    buf_writer.write_u32::<LE>(0xDEAD)?;
    buf_writer.write_u32::<LE>(0xBEEF)?;

    Ok(())
}


fn read_protocol(path: &str) -> io::Result<Vec<u32>> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);

    // Our protocol has to begin with a certain string
    // Namely "MyProtocol", which is 10 bytes long
    let mut start = [0u8; 10];
    buf_reader.read_exact(&mut start)?;
    if &start != b"MyProtocol" {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Protocol didn't start with the expected magic string",
        ));
    }

    // Now comes the endianness indicator
    let mut endian = [0u8; 2];
    buf_reader.read_exact(&mut endian)?;
    match &endian {
        b"LE" => read_protocoll_payload::<LE, _>(&mut buf_reader),
        b"BE" => read_protocoll_payload::<BE, _>(&mut buf_reader),
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to parse endianness",
        )),
    }
}

// Read as much of the payload as possible
fn read_protocoll_payload<E, R>(reader: &mut R) -> io::Result<Vec<u32>>
where
    E: ByteOrder,
    R: ReadBytesExt,
{
    let mut payload = Vec::new();
    const SIZE_OF_U32: usize = 4;
    loop {
        let mut raw_payload = [0; SIZE_OF_U32];
        // Read the next 4 bytes
        match reader.read(&mut raw_payload)? {
            // Zero means we reached the end
            0 => return Ok(payload),
            // SIZE_OF_U32 means we read a complete number
            SIZE_OF_U32 => {
                let as_u32 = raw_payload.as_ref().read_u32::<E>()?;
                payload.push(as_u32)
            }
            // Anything else means the last element was not
            // a valid u32
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Payload ended unexpectedly",
                ))
            }
        }
    }
}
