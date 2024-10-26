use std::str::FromStr;

/// `Header` struct that is used to parse the `Line` headers
#[derive(Debug, Clone)]
pub struct Header {
    pub code: String,
    pub value_type: ValueType,
    pub value_format: ValueFormat,
    pub value_size: usize,
}

impl Header {
    pub fn parse_header(line: &str) -> Self {
        let code = parse_code(line);
        let value_type = parse_value_type(line);
        let value_format = parse_value_format(line);
        let value_size = parse_value_size(line);
        Self {
            code,
            value_type,
            value_format,
            value_size,
        }
    }
}

/// `Edigeo` header consists on 8 bytes. The first 3 bytes of the header is
/// the header code for example `RIY`.
pub fn parse_code(line: &str) -> String {
    assert!(line.contains(":"), "Input str not of valid form");
    line[0..3].to_string()
}

pub fn parse_value_type(line: &str) -> ValueType {
    line[3..4]
        .parse::<ValueType>()
        .expect("Error parsing ValueType")
}

pub fn parse_value_format(line: &str) -> ValueFormat {
    line[4..5]
        .parse::<ValueFormat>()
        .expect("Error parsing ValueFormat")
}

pub fn parse_value_size(line: &str) -> usize {
    line[5..7].parse::<usize>().unwrap()
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueFormat {
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

impl FromStr for ValueFormat {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "A" => Ok(ValueFormat::A),
            "C" => Ok(ValueFormat::C),
            "D" => Ok(ValueFormat::D),
            "E" => Ok(ValueFormat::E),
            "I" => Ok(ValueFormat::I),
            "N" => Ok(ValueFormat::N),
            "P" => Ok(ValueFormat::P),
            "R" => Ok(ValueFormat::R),
            "T" => Ok(ValueFormat::T),
            " " => Ok(ValueFormat::WhiteSpace),
            _ => Err(format!("Invalid Character for ValueFormat: {}", input)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    /// reserved logical record
    T,
    /// simple field (1 value only)
    S,
    /// compound field (several values)
    C,
}

impl FromStr for ValueType {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "T" => Ok(ValueType::T),
            "S" => Ok(ValueType::S),
            "C" => Ok(ValueType::C),
            _ => Err(format!("Invalid Character for NatureField: {}", input)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_value_format() {
        let test_cases = [
            ("A", ValueFormat::A),
            ("C", ValueFormat::C),
            ("D", ValueFormat::D),
            ("E", ValueFormat::E),
            ("I", ValueFormat::I),
            ("N", ValueFormat::N),
            ("P", ValueFormat::P),
            ("R", ValueFormat::R),
            ("T", ValueFormat::T),
            (" ", ValueFormat::WhiteSpace),
        ];
        for (s, result) in test_cases {
            assert_eq!(s.parse::<ValueFormat>().unwrap(), result)
        }
    }

    #[test]
    fn test_header_code_parse_passes() {
        let test_cases = [
            ("BOMT 12:E0000A01.THF", "BOM"),
            ("CSET 03:IRV", "CSE"),
            ("RTYSA03:GTS", "RTY"),
            ("RIDSA10:SUPPORT_01", "RID"),
            ("VDASD08:19920801", "VDA"),
            ("GDNSA02:S1:", "GDN"),
            ("EOMT 00:", "EOM"),
        ];
        for (line, result) in test_cases {
            let header = Header::parse_header(line);
            assert_eq!(header.code, result.to_string());
        }
    }

    #[test]
    fn test_header_value_type_parse_passes() {
        let test_cases = [
            ("BOMT 12:E0000A01.THF", ValueType::T),
            ("CSET 03:IRV", ValueType::T),
            ("RTYSA03:GTS", ValueType::S),
            ("RIDSA10:SUPPORT_01", ValueType::S),
            ("VDASD08:19920801", ValueType::S),
            ("GDNSA02:S1:", ValueType::S),
            ("EOMT 00:", ValueType::T),
        ];
        for (line, result) in test_cases {
            let header = Header::parse_header(line);
            assert_eq!(header.value_type, result);
        }
    }

    #[test]
    fn test_header_value_format_parse_passes() {
        let test_cases = [
            ("BOMT 12:E0000A01.THF", ValueFormat::WhiteSpace),
            ("CSET 03:IRV", ValueFormat::WhiteSpace),
            ("RTYSA03:GTS", ValueFormat::A),
            ("RIDSA10:SUPPORT_01", ValueFormat::A),
            ("VDASD08:19920801", ValueFormat::D),
            ("GDNSA02:S1:", ValueFormat::A),
            ("EOMT 00:", ValueFormat::WhiteSpace),
        ];
        for (line, result) in test_cases {
            let header = Header::parse_header(line);

            assert_eq!(header.value_format, result);
        }
    }

    #[test]
    fn test_header_value_size_parse_passes() {
        let test_cases = [
            ("BOMT 12:E0000A01.THF", 12),
            ("CSET 03:IRV", 3),
            ("RTYSA03:GTS", 3),
            ("RIDSA10:SUPPORT_01", 10),
            ("VDASD08:19920801", 8),
            ("GDNSA02:S1:", 2),
            ("EOMT 00:", 0),
        ];
        for (line, result) in test_cases {
            let header = Header::parse_header(line);

            assert_eq!(header.value_size, result);
        }
    }

    #[test]
    #[should_panic(expected = "Invalid Character for ValueFormat: :")]
    fn test_header_parse_incorrect_data_passes() {
        let test_cases = ["BOMT:E0000A01.THF"];
        for line in test_cases {
            let _header = Header::parse_header(line);
        }
    }
}
