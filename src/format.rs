//! Contains Edigeo file parse formatters using the [`FormatParser`] trait.
use crate::header::*;

/// FormatResult Enum with variants being the DTypes of the parsed values
#[derive(Debug, Clone, PartialEq)]
pub enum FormatResult {
    /// Floating point number as result type
    Float(f64),
    /// Signed Integer as result type
    Int(i32),
    /// Date in String Format as result type
    Date(String),
    /// Plain text as result type
    Text(String),
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
pub struct NoneParser;

impl FormatParser for NoneParser {
    fn parse(&self, _raw_value: &str) -> Option<FormatResult> {
        None
    }
}

/// A parser for floating-point numbers.
pub struct FloatParser;

impl FormatParser for FloatParser {
    fn parse(&self, raw_value: &str) -> Option<FormatResult> {
        raw_value.parse::<f64>().ok().map(FormatResult::Float)
    }
}

/// A parser for signed integers.
pub struct IntParser;

impl FormatParser for IntParser {
    fn parse(&self, raw_value: &str) -> Option<FormatResult> {
        raw_value.parse::<i32>().ok().map(FormatResult::Int)
    }
}

/// A parser for dates in `YYYYMMDD` format.
pub struct DateParser;

impl FormatParser for DateParser {
    fn parse(&self, raw_value: &str) -> Option<FormatResult> {
        if raw_value.len() != 8 {
            return None;
        }
        Some(FormatResult::Date(format!(
            "{}-{}-{}",
            &raw_value[0..4],
            &raw_value[4..6],
            &raw_value[6..8]
        )))
    }
}

/// A parser for plain text.
pub struct TextParser;

impl FormatParser for TextParser {
    fn parse(&self, raw_value: &str) -> Option<FormatResult> {
        Some(FormatResult::Text(raw_value.to_string()))
    }
}

/// A parser for descriptors, splitting input by semicolons.
pub struct DescriptorParser;

impl FormatParser for DescriptorParser {
    fn parse(&self, raw_value: &str) -> Option<FormatResult> {
        let sections = raw_value
            .split(";")
            .map(|v| FormatResult::Text(v.to_string()))
            .collect::<Vec<FormatResult>>();
        Some(FormatResult::Descriptor(sections))
    }
}

/// Returns a parser based on the provided `Header`.
///
/// This function selects an appropriate `FormatParser` implementation
/// based on the `value_format` and `value_type` fields of the header.
pub fn get_parser(header: Header) -> Box<dyn FormatParser> {
    match header.value_format {
        ValueFormat::A => Box::new(TextParser),
        ValueFormat::C | ValueFormat::R => Box::new(FloatParser),
        ValueFormat::D => Box::new(DateParser),
        ValueFormat::E => Box::new(TextParser),
        ValueFormat::N | ValueFormat::I => Box::new(IntParser),
        ValueFormat::P => match header.value_type {
            ValueType::T => Box::new(TextParser),
            ValueType::S => Box::new(TextParser),
            ValueType::C => Box::new(DescriptorParser),
        },
        ValueFormat::T => Box::new(TextParser),
        ValueFormat::WhiteSpace => Box::new(NoneParser),
    }
}
