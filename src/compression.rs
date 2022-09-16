use std::io;
use std::io::{Cursor, Error};

// Include compression dict
const DICT: &[u8] = include_bytes!("./dict.bin");

pub fn compress(data: &[u8], level: usize) -> Result<Vec<u8>, Error> {
    let mut compressed = Vec::new();
    let mut encoder = zstd::Encoder::with_dictionary(&mut compressed, level as i32, DICT).unwrap();
    // encoder.multithread(10).unwrap();

    let mut cur = Cursor::new(data);

    match io::copy(&mut cur, &mut encoder) {
        Ok(_) => {}
        Err(e) => return Err(e),
    }
    encoder.finish().unwrap();
    Ok(compressed)
}

pub fn decompress(data: &[u8]) -> Result<Vec<u8>, Error> {
    let mut decompressed = Vec::new();
    let mut decoder = zstd::Decoder::with_dictionary(data, DICT).unwrap();

    match io::copy(&mut decoder, &mut decompressed) {
        Ok(_) => {}
        Err(e) => return Err(e),
    }
    Ok(decompressed)
}
