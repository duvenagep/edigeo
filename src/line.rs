//! Contains Edigeo [`Line`] parsing logic.

use crate::format::*;
use crate::header::*;

/// Represents a line with its header, raw value, and parsed result.
#[derive(Debug, Clone, PartialEq)]
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
        let (head, raw_value) = line.split_once(":").unwrap();
        let header = Header::parse_header(head).unwrap();
        Self {
            header: header.clone(),
            raw_value: raw_value.to_string(),
            parsed_value: parse_value(&header, raw_value),
        }
    }

    /// Checks if [`Line`] is empty or Newline and returns True
    #[allow(unconditional_recursion)]
    pub fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

/// Parses a raw value according to the specifications in the header.
///
/// Returns an `Option<FormatResult>`. If the raw value size does not match
/// the expected size from the header, it panics. Otherwise, it uses a parser
/// specific to the header to parse the value.
pub fn parse_value(header: &Header, raw_value: &str) -> Option<FormatResult> {
    if header.value_size != raw_value.chars().count() {
        panic!("value size mismatch!");
    }

    let parser = get_parser(header);
    parser.parse(raw_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_parse_passes() {
        let test_cases = [("BOMT 12:E0000A01.THF", "E0000A01.THF", None)];
        for (line, raw_value, result) in test_cases {
            let header = Header::parse_header(line).unwrap();
            let parsed_value = parse_value(&header, raw_value);
            assert_eq!(result, parsed_value);
        }
    }

    #[test]
    #[should_panic]
    fn test_value_parse_incorrect_passes() {
        let test_cases = [(
            "BOMT 12:E0000A01.THF",
            "E0000A01.THF",
            Some(FormatResult::Text("E0000A01.THF".to_string())),
        )];
        for (line, raw_value, result) in test_cases {
            let header = Header::parse_header(line).unwrap();
            let parsed_value = parse_value(&header, raw_value);
            assert_eq!(result, parsed_value);
        }
    }

    #[test]
    fn test_line_parse_passes() {
        let test_cases = [(
            "BOMT 12:E0000A01.THF",
            Line {
                header: Header {
                    code: Code::KWCode(KeyWordCode::BOM),
                    value_type: ValueType::T,
                    value_format: ValueFormat::WhiteSpace,
                    value_size: 12,
                },
                raw_value: "E0000A01.THF".to_string(),
                parsed_value: None,
            },
        )];
        for (line, result) in test_cases {
            let line = Line::parse_line(line);
            assert_eq!(result, line);
        }
    }

    #[test]
    #[should_panic]
    fn test_line_parse_incorrect_passes() {
        let test_cases = [(
            "BOMT 12:E0000A01.THF",
            Line {
                header: Header {
                    code: Code::KWCode(KeyWordCode::BOM),
                    value_type: ValueType::T,
                    value_format: ValueFormat::WhiteSpace,
                    value_size: 7,
                },
                raw_value: "E0000A01.THF".to_string(),
                parsed_value: None,
            },
        )];
        for (line, result) in test_cases {
            let line = Line::parse_line(line);
            assert_eq!(result, line);
        }
    }
}
