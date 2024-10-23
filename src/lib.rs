// #![warn(missing_docs)]
//! # [`EDIGéO`](https://www.data.gouv.fr/s/resources/plan-cadastral-informatise/20170906-150737/standard_edigeo_2013.pdf) Exchange Format
//!
//! The EDIGéO (Electronic Data Interchange in the field of Geographic Information) format was established
//! by the French standards association (AFNOR). EDIGéO is a standardized format commonly used in France for
//! the exchange of geographical information.
//!
//! The top-level data structure for an EDIGéO dataset is the exchange. An exchange appears as a single .THF file.
//! This file does not hold the main data; instead it specifies which lots belong to the exchange. An exchange,
//! therefore, consists of one or more lots. A lot in EDIGéO is conceptually a dataset. Within a lot, all data is
//! self-contained. Therefore, opening an exchange file with multiple lots is conceptually identical to opening
//! several exchange files each having one lot.
//!
//! An EDIGéO lot is described in several plain text files. These files are listed below:
//! `.GEN` - General Information
pub mod directory;
use std::{
    fs::{self},
    io::{self, BufRead, Read},
    path::Path,
};

pub use directory::EdigeoDir;

/// The read function take a Path and read the bytes
/// converting the bytes with Latin1 encoding and
/// returns a string
pub fn read<P: AsRef<Path>>(path: P) -> String {
    let bytes = fs::read(path).unwrap();
    let contents = String::from_utf8_lossy(&bytes);
    contents.into_owned()
}

pub fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

pub fn read_bytes<P: AsRef<Path>>(path: P) -> String {
    let mut rdr = encoding_rs_io::DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding_rs::WINDOWS_1252))
        .build(std::fs::File::open(path.as_ref()).unwrap());
    let mut string = String::new();
    // This is guaranteed to never return a UTF-8 decoding error since the
    // transcoding guarantees that its output is valid UTF-8.
    rdr.read_to_string(&mut string).unwrap();

    string
}

/// Efficient txt file reader that passes ownership to File::open() which uses a BufReader thus reduing
/// internal allocations. The internal bytes are decoded using `WINDOWS_1252` encoding (Latin1)
pub fn read_lines_efficient<P>(path: P) -> io::Result<impl Iterator<Item = io::Result<String>>>
where
    P: AsRef<Path>,
{
    let rdr = encoding_rs_io::DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding_rs::WINDOWS_1252))
        .build(fs::File::open(path.as_ref())?);

    Ok(io::BufReader::new(rdr).lines())
}
