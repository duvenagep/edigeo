use edigeo::{read_lines_efficient, EdigeoDir};
use std::str::FromStr;

fn main() {
    println!("Hello World");
    let dir = "data/edigeo-740240000A01/";

    let e = EdigeoDir::extract_files(dir);

    // println!("{:#?}", &e);

    if let Ok(lines) = read_lines_efficient(e.t1) {
        for line in lines {
            if !line.is_empty() {
                let data = Line::parse_line(&line);
                println!("{:?}", data);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    header: Header,
    raw_value: String,
    parsed_value: Option<FormatResult>,
}

impl Line {
    fn parse_line(line: &str) -> Self {
        let (header, raw_value) = line.split_once(":").unwrap();
        let header = Header::parse_header(&header);
        Self {
            header: header.clone(),
            raw_value: raw_value.to_string(),
            parsed_value: parse_value(header, raw_value),
        }
    }
}

pub fn parse_value(header: Header, raw_value: &str) -> Option<FormatResult> {
    if header.value_size != raw_value.chars().count() {
        panic!("value size mismatch!");
    }
    let parser = get_parser(header.value_format);
    let parsed_value = parser.parse(raw_value);

    parsed_value
}

/// `Header` struct that is used to parse the `Line` headers
#[derive(Debug, Clone)]
pub struct Header {
    code: String,
    value_type: NatureField,
    value_format: FormatField,
    value_size: usize,
}

impl Header {
    pub fn parse_header(line: &str) -> Self {
        let value_type = line[3..4]
            .parse::<NatureField>()
            .expect("Error parsing NatureField");
        let value_format = line[4..5]
            .parse::<FormatField>()
            .expect("Error parsing FormatField");
        Self {
            code: line[0..3].to_string(),
            value_type,
            value_format,
            value_size: line[5..7].parse::<usize>().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FormatField {
    /// String of Characters
    A,
    /// Coordinates
    C,
    /// Date in 20240116 format
    D,
    /// Real number with exponent
    E,
    /// Signed Integer (e.g. -7)
    I,
    /// Unsigned Integer
    N,
    /// Descriptor Reference
    P,
    /// Real number
    R,
    /// Plain text
    T,
    /// White Space
    WhiteSpace,
}

impl FromStr for FormatField {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "A" => Ok(FormatField::A),
            "C" => Ok(FormatField::C),
            "D" => Ok(FormatField::D),
            "E" => Ok(FormatField::E),
            "I" => Ok(FormatField::I),
            "N" => Ok(FormatField::N),
            "P" => Ok(FormatField::P),
            "R" => Ok(FormatField::R),
            "T" => Ok(FormatField::T),
            " " => Ok(FormatField::WhiteSpace),
            _ => Err(format!("Invalid Character for FormatField: {}", input)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum NatureField {
    /// reserved logical record
    T,
    /// simple field (1 value only)
    S,
    /// compound field (several values)
    C,
}

impl FromStr for NatureField {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "T" => Ok(NatureField::T),
            "S" => Ok(NatureField::S),
            "C" => Ok(NatureField::C),
            _ => Err(format!("Invalid Character for NatureField: {}", input)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FormatResult {
    Float(f64),
    Int(i32),
    Date(String),
    Text(String),
    Descriptor(Vec<String>),
}

pub trait FormatParser {
    fn parse(&self, raw_value: &str) -> Option<FormatResult>;
}

pub struct NoneParser;
impl FormatParser for NoneParser {
    fn parse(&self, _raw_value: &str) -> Option<FormatResult> {
        None
    }
}

pub struct FloatParser;
impl FormatParser for FloatParser {
    fn parse(&self, raw_value: &str) -> Option<FormatResult> {
        raw_value.parse::<f64>().ok().map(FormatResult::Float)
    }
}

pub struct IntParser;
impl FormatParser for IntParser {
    fn parse(&self, raw_value: &str) -> Option<FormatResult> {
        raw_value.parse::<i32>().ok().map(FormatResult::Int)
    }
}

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

pub struct TextParser;
impl FormatParser for TextParser {
    fn parse(&self, raw_value: &str) -> Option<FormatResult> {
        Some(FormatResult::Text(raw_value.to_string()))
    }
}

// pub struct DescriptorParser;
// impl FormatParser for DescriptorParser {
//     fn parse(&self, raw_value: &str) -> Option<FormatResult> {

//         let v = raw_value.split(";").collect::<String>();

//     }
// }

pub fn get_parser(value_format: FormatField) -> Box<dyn FormatParser> {
    match value_format {
        FormatField::A => Box::new(TextParser),
        FormatField::C | FormatField::R => Box::new(FloatParser),
        FormatField::D => Box::new(DateParser),
        FormatField::E => Box::new(TextParser),
        FormatField::N | FormatField::I => Box::new(IntParser),
        FormatField::P => Box::new(TextParser),
        FormatField::T => Box::new(TextParser),
        FormatField::WhiteSpace => Box::new(NoneParser),
    }
}
