pub mod directory;
use std::{fs, io::Read, path::Path};

pub use directory::EdigeoDir;

/// The read function take a Path and read the bytes
/// converting the bytes with Latin1 encoding and
/// returns a string
pub fn read<'a, P: AsRef<Path>>(path: P) -> String {
    let bytes = fs::read(path).unwrap();
    let contents = String::from_utf8_lossy(&bytes);
    contents.into_owned()
}

pub fn read_bytes<'a, P: AsRef<Path>>(path: P) -> String {
    let mut rdr = encoding_rs_io::DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding_rs::WINDOWS_1252))
        .build(std::fs::File::open(path.as_ref()).unwrap());
    let mut string = String::new();
    // This is guaranteed to never return a UTF-8 decoding error since the
    // transcoding guarantees that its output is valid UTF-8.
    rdr.read_to_string(&mut string).unwrap();

    string
}
