extern crate flate2;

use std::io::{self, SeekFrom};
use std::io::prelude::*;

use flate2::{Compression, FlateReadExt};
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;

use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read};

fn main() {
    let bytes = b"I have a dream that one day this nation will rise up, \
        and live out the true meaning of its creed";
    println!("Original: {:?}", bytes.as_ref());
    // Conpress some bytes
    let encoded = encode_bytes(bytes.as_ref()).expect("Failed to encode bytes");
    println!("Encoded: {:?}", encoded);
    // Decompress them again
    let decoded = decode_bytes(&encoded).expect("Failed to decode bytes");
    println!("Decoded: {:?}", decoded);

    // Open file to compress
    let original = File::open("ferris.png").expect("Failed to open file");
    let mut original_reader = BufReader::new(original);

    // Compress it
    let data = encode_file(&mut original_reader).expect("Failed to encode file");

    // Write compressed file to disk
    let encoded = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("ferris_encoded.zlib")
        .expect("Failed to create encoded file");
    let mut encoded_reader = BufReader::new(&encoded);
    let mut encoded_writer = BufWriter::new(&encoded);
    encoded_writer
        .write_all(&data)
        .expect("Failed to write encoded file");


    // Jump back to the beginning of the compressed file
    encoded_reader
        .seek(SeekFrom::Start(0))
        .expect("Failed to reset file");

    // Decompress it
    let data = decode_file(&mut encoded_reader).expect("Failed to decode file");

    // Write the decompressed file to disk
    let mut decoded = File::create("ferris_decoded.png").expect("Failed to create decoded file");
    decoded
        .write_all(&data)
        .expect("Failed to write decoded file");
}


fn encode_bytes(bytes: &[u8]) -> io::Result<Vec<u8>> {
    // You can choose your compression algorithm and it's efficiency
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::Default);
    encoder.write_all(bytes)?;
    encoder.finish()
}

fn decode_bytes(bytes: &[u8]) -> io::Result<Vec<u8>> {
    let mut encoder = ZlibDecoder::new(bytes);
    let mut buffer = Vec::new();
    encoder.read_to_end(&mut buffer)?;
    Ok(buffer)
}


fn encode_file(file: &mut Read) -> io::Result<Vec<u8>> {
    // Files have a built-in encoder
    let mut encoded = file.zlib_encode(Compression::Best);
    let mut buffer = Vec::new();
    encoded.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn decode_file(file: &mut Read) -> io::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    // Files have a built-in decoder
    file.zlib_decode().read_to_end(&mut buffer)?;
    Ok(buffer)
}
