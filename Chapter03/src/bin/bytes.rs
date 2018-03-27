extern crate byteorder;
use std::io::{Cursor, Seek, SeekFrom};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};

fn main() {
    let binary_nums = vec![2, 3, 12, 8, 5, 0];
    // Wrap a binary collection in a cursor
    // to provide seek functionality
    let mut buff = Cursor::new(binary_nums);
    let first_byte = buff.read_u8().expect("Failed to read byte");
    println!("first byte in binary: {:b}", first_byte);

    // Reading advances the internal position,
    // so now we read the second
    let second_byte_as_int = buff.read_i8().expect("Failed to read byte as int");
    println!("second byte as int: {}", second_byte_as_int);

    // Overwrite the current position
    println!("Before: {:?}", buff);
    buff.write_u8(123).expect("Failed to overwrite a byte");
    println!("After: {:?}", buff);


    // Set and get the current position
    println!("Old position: {}", buff.position());
    buff.set_position(0);
    println!("New position: {}", buff.position());

    // This also works using the Seek API
    buff.seek(SeekFrom::End(0)).expect("Failed to seek end");
    println!("Last position: {}", buff.position());

    // Read and write in specific endianness
    buff.set_position(0);
    let as_u32 = buff.read_u32::<LittleEndian>()
        .expect("Failed to read bytes");
    println!(
        "First four bytes as u32 in little endian order:\t{}",
        as_u32
    );

    buff.set_position(0);
    let as_u32 = buff.read_u32::<BigEndian>().expect("Failed to read bytes");
    println!("First four bytes as u32 in big endian order:\t{}", as_u32);

    println!("Before appending: {:?}", buff);
    buff.seek(SeekFrom::End(0)).expect("Failed to seek end");
    buff.write_f32::<LittleEndian>(-33.4)
        .expect("Failed to write to end");
    println!("After appending: {:?}", buff);

    // Read a sequence of bytes into another buffer
    let mut read_buffer = [0; 5];
    buff.set_position(0);
    buff.read_u16_into::<LittleEndian>(&mut read_buffer)
        .expect("Failed to read all bytes");
    println!(
        "All bytes as u16s in little endian order: {:?}",
        read_buffer
    );
}
