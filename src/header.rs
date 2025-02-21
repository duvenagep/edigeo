//! Contains Edigeo file [`Header`] definition & logic..
use crate::error::*;
use std::str::FromStr;

/// Represents a parsed header in an Edigeo `Line`.
///
/// Stores metadata about a header, including code, data type, format, and size.
#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    /// Identifier code for the header.
    pub code: Code,

    /// Data type of the header's value.
    pub value_type: ValueType,

    /// Format of the header's value.
    pub value_format: ValueFormat,

    /// Size of the value in bytes.
    pub value_size: usize,
}

impl Header {
    /// Parses a header line into a `Header` struct.
    ///
    /// Extracts the header code, value type, format, and size from specified
    /// byte positions in the line.
    pub fn parse_header(line: &str) -> EdigeoResult<Self> {
        let code = parse_code(line);
        let value_type = parse_value_type(line);
        let value_format = parse_value_format(line);
        let value_size = parse_value_size(line)?;
        Ok(Self {
            code,
            value_type,
            value_format,
            value_size,
        })
    }
}

/// Parses the first 3 bytes of a header line to extract the header code.
///
/// Example:  `RTYSA03:GTS` -> `RIY`.
///
/// # Panics
/// Panics if the input does not contain a colon (`:`).
pub fn parse_code(line: &str) -> Code {
    // assert!(line.contains(":"), "Input str not of valid form");
    // line[0..3].to_string()
    line[0..3]
        .parse::<Code>()
        .expect("Invalid Code value in Header String")
}

/// Parses the value type from the 4th byte of the header line.
///
/// Example:  `RTYSA03:GTS` -> `S`.
///
/// # Panics
/// Panics if parsing `ValueType` fails.
pub fn parse_value_type(line: &str) -> ValueType {
    // assert!(line.contains(":"), "Input str not of valid form");

    line[3..4]
        .parse::<ValueType>()
        .expect("Invalid Value Type Field.")
}

/// Parses the value format from the 5th byte of the header line.
///
/// Example:  `RTYSA03:GTS` -> `A`.
///
/// # Panics
/// Panics if parsing `ValueFormat` fails.
pub fn parse_value_format(line: &str) -> ValueFormat {
    // assert!(line.contains(":"), "Input str not of valid form");
    line[4..5]
        .parse::<ValueFormat>()
        .expect("Invalid Value Format Field.")
}

/// Parses the value size from the 6th and 7th bytes of the header line.
///
/// Example:  `RTYSA03:GTS` -> `3`.
///
/// # Panics
/// Panics if parsing `usize` fails.
pub fn parse_value_size(line: &str) -> EdigeoResult<usize> {
    // assert!(line.contains(":"), "Input str not of valid form");
    Ok(line[5..7].parse::<usize>()?)
}

/// Specifies the format of a value in an Edigeo header.
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
    type Err = EdigeoError;
    fn from_str(input: &str) -> EdigeoResult<Self> {
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
            _ => Err(EdigeoError::InvalidFormat(input.to_string())),
        }
    }
}

/// Specifies the type of a value in an Edigeo header.
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
    type Err = EdigeoError;
    fn from_str(input: &str) -> EdigeoResult<Self> {
        match input {
            "T" => Ok(ValueType::T),
            "S" => Ok(ValueType::S),
            "C" => Ok(ValueType::C),
            _ => Err(EdigeoError::InvalidFormat(input.to_string())),
        }
    }
}

/// Specifies the Special Codes used to indicate File Position Metadata.
#[derive(Debug, Clone, PartialEq)]
pub enum KeyWordCode {
    /// Indicates the logical start of the file.
    BOM,
    /// Indicates the name of the character set used.
    CSE,
    /// Indicates the logical end of the file; its value is always zero.
    EOM,
}

/// Type of descriptor
#[derive(Debug, Clone, PartialEq)]
pub enum TypeCode {
    /// Type of descriptor
    RTY,
}

/// Descriptor Identifier
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub enum ZoneName {
    RID,
    AUT,
    ADR,
    LOC,
    VOC,
    SEC,
    RDI,
    VER,
    VDA,
    TRL,
    EDN,
    TDA,
    INF,
    LON,
    GNN,
    GNI,
    GON,
    GOI,
    QAN,
    QAI,
    DIN,
    DII,
    SCN,
    SCI,
    GDC,
    GDN,
    GDI,
    CM1,
    CM2,
    STR,
    REG,
    RET,
    REN,
    REL,
    DIM,
    ALS,
    UNH,
    LAB,
    DEF,
    ORI,
    CAT,
    TYP,
    UNI,
    AVC,
    AVL,
    AVD,
    DIP,
    KND,
    AAC,
    AAP,
    QAC,
    CAN,
    AV1,
    AV2,
    CA1,
    CA2,
    SCC,
    SCP,
    OCC,
    ODA,
    UTY,
    ULO,
    UDA,
    RAT,
    EDA,
    COC,
    COP,
    COR,
    ATC,
    PTC,
    REF,
    ATP,
    ATV,
    QAP,
    FTC,
    FTP,
    SNS,
    TEX,
}

/// Specifies the type of a code in an Edigeo header.
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub enum Code {
    /// Special File Keywords
    KWCode(KeyWordCode),
    /// Type of descriptor
    TypeCode(TypeCode),
    /// Zonename value Codes
    ZoneCode(ZoneName),
}

impl FromStr for Code {
    type Err = EdigeoError;
    fn from_str(input: &str) -> EdigeoResult<Self> {
        match input {
            "BOM" => Ok(Code::KWCode(KeyWordCode::BOM)),
            "CSE" => Ok(Code::KWCode(KeyWordCode::CSE)),
            "EOM" => Ok(Code::KWCode(KeyWordCode::EOM)),
            "RTY" => Ok(Code::TypeCode(TypeCode::RTY)),
            "RID" => Ok(Code::ZoneCode(ZoneName::RID)),
            "AUT" => Ok(Code::ZoneCode(ZoneName::AUT)),
            "ADR" => Ok(Code::ZoneCode(ZoneName::ADR)),
            "LOC" => Ok(Code::ZoneCode(ZoneName::LOC)),
            "VOC" => Ok(Code::ZoneCode(ZoneName::VOC)),
            "SEC" => Ok(Code::ZoneCode(ZoneName::SEC)),
            "RDI" => Ok(Code::ZoneCode(ZoneName::RDI)),
            "VER" => Ok(Code::ZoneCode(ZoneName::VER)),
            "VDA" => Ok(Code::ZoneCode(ZoneName::VDA)),
            "TRL" => Ok(Code::ZoneCode(ZoneName::TRL)),
            "EDN" => Ok(Code::ZoneCode(ZoneName::EDN)),
            "TDA" => Ok(Code::ZoneCode(ZoneName::TDA)),
            "INF" => Ok(Code::ZoneCode(ZoneName::INF)),
            "LON" => Ok(Code::ZoneCode(ZoneName::LON)),
            "GNN" => Ok(Code::ZoneCode(ZoneName::GNN)),
            "GNI" => Ok(Code::ZoneCode(ZoneName::GNI)),
            "GON" => Ok(Code::ZoneCode(ZoneName::GON)),
            "GOI" => Ok(Code::ZoneCode(ZoneName::GOI)),
            "QAN" => Ok(Code::ZoneCode(ZoneName::QAN)),
            "QAI" => Ok(Code::ZoneCode(ZoneName::QAI)),
            "DIN" => Ok(Code::ZoneCode(ZoneName::DIN)),
            "DII" => Ok(Code::ZoneCode(ZoneName::DII)),
            "SCN" => Ok(Code::ZoneCode(ZoneName::SCN)),
            "SCI" => Ok(Code::ZoneCode(ZoneName::SCI)),
            "GDC" => Ok(Code::ZoneCode(ZoneName::GDC)),
            "GDN" => Ok(Code::ZoneCode(ZoneName::GDN)),
            "GDI" => Ok(Code::ZoneCode(ZoneName::GDI)),
            "CM1" => Ok(Code::ZoneCode(ZoneName::CM1)),
            "CM2" => Ok(Code::ZoneCode(ZoneName::CM2)),
            "STR" => Ok(Code::ZoneCode(ZoneName::STR)),
            "REG" => Ok(Code::ZoneCode(ZoneName::REG)),
            "RET" => Ok(Code::ZoneCode(ZoneName::RET)),
            "REN" => Ok(Code::ZoneCode(ZoneName::REN)),
            "REL" => Ok(Code::ZoneCode(ZoneName::REL)),
            "DIM" => Ok(Code::ZoneCode(ZoneName::DIM)),
            "ALS" => Ok(Code::ZoneCode(ZoneName::ALS)),
            "UNH" => Ok(Code::ZoneCode(ZoneName::UNH)),
            "LAB" => Ok(Code::ZoneCode(ZoneName::LAB)),
            "DEF" => Ok(Code::ZoneCode(ZoneName::DEF)),
            "ORI" => Ok(Code::ZoneCode(ZoneName::ORI)),
            "CAT" => Ok(Code::ZoneCode(ZoneName::CAT)),
            "TYP" => Ok(Code::ZoneCode(ZoneName::TYP)),
            "UNI" => Ok(Code::ZoneCode(ZoneName::UNI)),
            "AVC" => Ok(Code::ZoneCode(ZoneName::AVC)),
            "AVL" => Ok(Code::ZoneCode(ZoneName::AVL)),
            "AVD" => Ok(Code::ZoneCode(ZoneName::AVD)),
            "DIP" => Ok(Code::ZoneCode(ZoneName::DIP)),
            "KND" => Ok(Code::ZoneCode(ZoneName::KND)),
            "AAC" => Ok(Code::ZoneCode(ZoneName::AAC)),
            "AAP" => Ok(Code::ZoneCode(ZoneName::AAP)),
            "QAC" => Ok(Code::ZoneCode(ZoneName::QAC)),
            "CAN" => Ok(Code::ZoneCode(ZoneName::CAN)),
            "AV1" => Ok(Code::ZoneCode(ZoneName::AV1)),
            "AV2" => Ok(Code::ZoneCode(ZoneName::AV2)),
            "CA1" => Ok(Code::ZoneCode(ZoneName::CA1)),
            "CA2" => Ok(Code::ZoneCode(ZoneName::CA2)),
            "SCC" => Ok(Code::ZoneCode(ZoneName::SCC)),
            "SCP" => Ok(Code::ZoneCode(ZoneName::SCP)),
            "OCC" => Ok(Code::ZoneCode(ZoneName::OCC)),
            "ODA" => Ok(Code::ZoneCode(ZoneName::ODA)),
            "UTY" => Ok(Code::ZoneCode(ZoneName::UTY)),
            "ULO" => Ok(Code::ZoneCode(ZoneName::ULO)),
            "UDA" => Ok(Code::ZoneCode(ZoneName::UDA)),
            "RAT" => Ok(Code::ZoneCode(ZoneName::RAT)),
            "EDA" => Ok(Code::ZoneCode(ZoneName::EDA)),
            "COC" => Ok(Code::ZoneCode(ZoneName::COC)),
            "COP" => Ok(Code::ZoneCode(ZoneName::COP)),
            "COR" => Ok(Code::ZoneCode(ZoneName::COR)),
            "ATC" => Ok(Code::ZoneCode(ZoneName::ATC)),
            "PTC" => Ok(Code::ZoneCode(ZoneName::PTC)),
            "REF" => Ok(Code::ZoneCode(ZoneName::REF)),
            "ATP" => Ok(Code::ZoneCode(ZoneName::ATP)),
            "ATV" => Ok(Code::ZoneCode(ZoneName::ATV)),
            "QAP" => Ok(Code::ZoneCode(ZoneName::QAP)),
            "FTC" => Ok(Code::ZoneCode(ZoneName::FTC)),
            "FTP" => Ok(Code::ZoneCode(ZoneName::FTP)),
            "SNS" => Ok(Code::ZoneCode(ZoneName::SNS)),
            "TEX" => Ok(Code::ZoneCode(ZoneName::TEX)),
            _ => Err(EdigeoError::InvalidFormat(input.to_string())),
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

    // #[test]
    // fn test_header_code_parse_passes() {
    //     let test_cases = [
    //         ("BOMT 12:E0000A01.THF", "BOM"),
    //         ("CSET 03:IRV", "CSE"),
    //         ("RTYSA03:GTS", "RTY"),
    //         ("RIDSA10:SUPPORT_01", "RID"),
    //         ("VDASD08:19920801", "VDA"),
    //         ("GDNSA02:S1:", "GDN"),
    //         ("EOMT 00:", "EOM"),
    //     ];
    //     for (line, result) in test_cases {
    //         let header = Header::parse_header(line).unwrap();
    //         assert_eq!(header.code, result);
    //     }
    // }

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
            let header = Header::parse_header(line).unwrap();
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
            let header = Header::parse_header(line).unwrap();

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
            let header = Header::parse_header(line).unwrap();

            assert_eq!(header.value_size, result);
        }
    }

    #[test]
    #[should_panic]
    fn test_header_parse_incorrect_data_passes() {
        let test_cases = ["BOMT:E0000A01.THF"];
        for line in test_cases {
            let _header = Header::parse_header(line).unwrap();
        }
    }
}
