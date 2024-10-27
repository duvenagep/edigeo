#![warn(missing_docs)]
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
pub mod error;
pub mod header;
use std::{
    fs::{self},
    io::{self, BufRead},
    path::Path,
};

pub use directory::EdigeoDir;

/// Efficient txt file reader that passes ownership to File::open() which uses a BufReader thus reducing
/// internal allocations. The internal bytes are decoded using `WINDOWS_1252` encoding (Latin1)
pub fn read_lines_efficient<P>(path: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let rdr = encoding_rs_io::DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding_rs::WINDOWS_1252))
        .build(fs::File::open(path.as_ref())?);

    let lines = io::BufReader::new(rdr)
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();
    Ok(lines)
    // Ok(io::BufReader::new(rdr).lines())
}
