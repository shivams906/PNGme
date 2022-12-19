use std::convert::TryFrom;
use std::fs;
use std::str::FromStr;

use crate::args::{Decode, Encode, Print, Remove};
use crate::png::{Chunk, ChunkType, Png};
use crate::Result;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: Encode) -> Result<()> {
    let mut png = Png::try_from(fs::read(&args.file_path)?.as_ref()).unwrap();
    let chunk_type = ChunkType::from_str(&args.chunk_type).unwrap();
    let chunk = Chunk::new(chunk_type, args.message.into_bytes());
    png.append_chunk(chunk);
    match args.output_file {
        Some(path) => fs::write(path, png.as_bytes())?,
        None => fs::write(args.file_path, png.as_bytes())?,
    };
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: Decode) -> Result<()> {
    let png = Png::try_from(fs::read(&args.file_path)?.as_ref()).unwrap();
    let chunk_type = ChunkType::from_str(&args.chunk_type).unwrap();
    match png.chunk_by_type(chunk_type.to_string().as_str()) {
        Some(chunk) => println!("{}", chunk.data_as_string().unwrap()),
        None => println!("Not Found"),
    };
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: Remove) -> Result<()> {
    let mut png = Png::try_from(fs::read(&args.file_path)?.as_ref()).unwrap();
    let chunk_type = ChunkType::from_str(&args.chunk_type).unwrap();
    png.remove_chunk(chunk_type.to_string().as_str()).unwrap();
    fs::write(args.file_path, png.as_bytes())?;
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: Print) -> Result<()> {
    let png = Png::try_from(fs::read(&args.file_path)?.as_ref()).unwrap();
    for chunk in png.chunks() {
        println!("{}", chunk);
    }
    Ok(())
}
