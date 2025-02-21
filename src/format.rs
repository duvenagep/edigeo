//! Contains Edigeo file parse formatters using the [`FormatParser`] trait.
use crate::header::*;
use chrono::NaiveDate;

/// FormatResult Enum with variants being the DTypes of the parsed values
#[derive(Debug, Clone, PartialEq)]
pub enum FormatResult {
    /// Floating point number as result type
    Float(f64),
    /// Signed Integer as result type
    Int(i32),
    /// Date in String Format as result type
    Date(NaiveDate),
    /// Plain text as result type
    Text(String),
    /// Coordinates are represented as tuple of strings
    Coordinate((String, String)),
    /// Project description which is a compound of FormatResult as result type
    Descriptor(Vec<FormatResult>),
}

/// Trait for parsing raw values into `FormatResult`.
pub trait FormatParser {
    /// Parses a raw string value into an `Option<FormatResult>`.
    ///
    /// Returns `Some(FormatResult)` if parsing is successful, or `None` if it fails.
    fn parse(&self, raw_value: &str) -> Option<FormatResult>;
}

/// A parser that returns `None` for any input.
struct NoneParser;

impl FormatParser for NoneParser {
    fn parse(&self, _raw_value: &str) -> Option<FormatResult> {
        None
    }
}

/// A parser for floating-point numbers.
struct FloatParser;

impl FormatParser for FloatParser {
    fn parse(&self, raw_value: &str) -> Option<FormatResult> {
        raw_value.parse::<f64>().ok().map(FormatResult::Float)
    }
}

/// A parser for signed integers.
struct IntParser;

impl FormatParser for IntParser {
    fn parse(&self, raw_value: &str) -> Option<FormatResult> {
        raw_value.parse::<i32>().ok().map(FormatResult::Int)
    }
}

/// A parser for dates in `YYYYMMDD` format.
struct DateParser;

impl FormatParser for DateParser {
    fn parse(&self, raw_value: &str) -> Option<FormatResult> {
        if raw_value.len() != 8 {
            return None;
        }
        Some(FormatResult::Date(
            NaiveDate::parse_from_str(raw_value, "%Y%m%d").unwrap(),
        ))
    }
}

/// A parser for plain text.
struct TextParser;

impl FormatParser for TextParser {
    fn parse(&self, raw_value: &str) -> Option<FormatResult> {
        Some(FormatResult::Text(raw_value.to_string()))
    }
}

/// A parser for descriptors, splitting input by semicolons.
struct DescriptorParser;

impl FormatParser for DescriptorParser {
    fn parse(&self, raw_value: &str) -> Option<FormatResult> {
        let sections = raw_value
            .split(";")
            .map(|v| FormatResult::Text(v.to_string()))
            .collect::<Vec<FormatResult>>();
        Some(FormatResult::Descriptor(sections))
    }
}

/// A parser for coordinates, splitting input by semicolons.
struct CoordinateParser;

impl FormatParser for CoordinateParser {
    fn parse(&self, raw_value: &str) -> Option<FormatResult> {
        let coords = raw_value.split_once(";");

        if let Some(coords) = coords {
            Some(FormatResult::Coordinate((
                coords.0.to_string(),
                coords.1.to_string(),
            )))
        } else {
            None
        }
    }
}

/// Returns a parser based on the provided `Header`.
///
/// This function selects an appropriate `FormatParser` implementation
/// based on the `value_format` and `value_type` fields of the header.
pub fn get_parser(header: &Header) -> Box<dyn FormatParser> {
    match header.value_format {
        ValueFormat::A => Box::new(TextParser),
        ValueFormat::C => Box::new(CoordinateParser),
        ValueFormat::R => Box::new(FloatParser),
        ValueFormat::D => Box::new(DateParser),
        ValueFormat::E => Box::new(TextParser),
        ValueFormat::N | ValueFormat::I => Box::new(IntParser),
        ValueFormat::P => match header.value_type {
            ValueType::T => Box::new(TextParser),
            ValueType::S => Box::new(TextParser),
            ValueType::C => Box::new(DescriptorParser),
        },
        ValueFormat::T => Box::new(TextParser),
        ValueFormat::WhiteSpace => match header.value_type {
            ValueType::T => Box::new(TextParser),
            _ => Box::new(NoneParser),
        },
    }
}
