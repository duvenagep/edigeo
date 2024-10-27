//! Contains Edigeo [`Line`] parsing logic.

use crate::format::*;
use crate::header::*;

/// Represents a line with its header, raw value, and parsed result.
#[derive(Debug, Clone)]
pub struct Line {
    /// The header associated with the line.
    pub header: Header,

    /// The raw string value of the line.
    pub raw_value: String,

    /// The parsed result of the raw value, if available.
    pub parsed_value: Option<FormatResult>,
}

impl Line {
    /// Parses a line of text into a `Line` struct.
    ///
    /// The line is expected to be in the format `header:raw_value`.
    /// The header is parsed into a `Header` struct, and the raw value
    /// is processed to obtain a parsed value.
    pub fn parse_line(line: &str) -> Self {
        let (header, raw_value) = line.split_once(":").unwrap();
        let header = Header::parse_header(header).unwrap();
        Self {
            header: header.clone(),
            raw_value: raw_value.to_string(),
            parsed_value: parse_value(header, raw_value),
        }
    }
}

/// Parses a raw value according to the specifications in the header.
///
/// Returns an `Option<FormatResult>`. If the raw value size does not match
/// the expected size from the header, it panics. Otherwise, it uses a parser
/// specific to the header to parse the value.
pub fn parse_value(header: Header, raw_value: &str) -> Option<FormatResult> {
    if header.value_size != raw_value.chars().count() {
        panic!("value size mismatch!");
    }

    let parser = get_parser(header);
    parser.parse(raw_value)
}
