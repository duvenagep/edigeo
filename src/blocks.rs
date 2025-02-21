//! Contains all logic for processing **Blocks**.
use std::path::PathBuf;

use crate::{decode_file, Code, FormatResult, KeyWordCode, Line};

/// Block Identifiers
#[derive(Debug)]
enum BlockIdent {
    /// Indentifier value for THF Support Descriptor Block
    GTS,
    /// Indentifier value for THF Batch Descriptor Block
    GTL,
    // /// Indentifier value for GEN Geographical Descriptor Block
    // DEG,
    // /// Indentifier value for GEN Geographical Data Block
    // GSE,
    // /// Indentifier value for GEO Geographical Data Block
    // GEO,
}

#[derive(Debug)]
struct Block {
    _id: String,
    entries: Vec<Line>,
}

impl Block {
    fn new(id: &str) -> Self {
        Self {
            _id: id.to_owned(),
            entries: Vec::new(),
        }
    }

    fn add_line(&mut self, line: Line) {
        self.entries.push(line);
    }
}

pub trait BlockParse {
    fn parse<S: AsRef<str>>(lines: S) -> Self;
}

#[derive(Debug)]
pub struct THFFile {
    support_block: Block,
    batch_block: Block,
}

impl THFFile {
    pub fn new(data: &[u8]) -> Self {
        let data = decode_file(data);
        let thf = THFFile::parse(data);
        thf
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl BlockParse for THFFile {
    fn parse<S: AsRef<str>>(lines: S) -> Self {
        let mut support_block = Block::new("GTS");
        let mut batch_block = Block::new("GTL");
        let mut current_block: Option<&mut Block> = None;

        for line in lines.as_ref().lines() {
            if line.is_empty() {
                continue;
            }
            let data = Line::parse_line(&line);

            match &data.header.code {
                Code::KWCode(kwc) => match kwc {
                    KeyWordCode::BOM => {}
                    KeyWordCode::CSE => {}
                    KeyWordCode::EOM => current_block = None,
                },
                Code::TypeCode(type_code) => match data.parsed_value {
                    Some(FormatResult::Text(ref value)) if value == "GTS" => {
                        current_block = Some(&mut support_block);
                    }
                    Some(FormatResult::Text(ref value)) if value == "GTL" => {
                        current_block = Some(&mut batch_block);
                    }
                    _ => {}
                },
                Code::ZoneCode(zone_name) => {
                    if let Some(block) = &mut current_block {
                        block.add_line(data);
                    }
                }
            }
        }

        Self {
            support_block,
            batch_block,
        }
    }
}

// #[derive(Debug)]
// pub struct GENFile {
//     geographical_descriptor_block: Block,
//     geographical_data_block: Block,
// }

// impl BlockParse for GENFile {
//     fn parse<S: AsRef<str>>(lines: S) -> Self {
//         let mut geographical_descriptor_block = Block::new("DEG");
//         let mut geographical_data_block = Block::new("GSE");
//         let mut current_block: Option<&mut Block> = None;

//         for line in lines.as_ref().lines() {
//             if line.is_empty() {
//                 continue;
//             }
//             let data = Line::parse_line(&line);

//             match data.header.code.as_str() {
//                 "BOM" | "CSE" => {}
//                 "RTY" => match data.parsed_value {
//                     Some(FormatResult::Text(ref value)) if value == "DEG" => {
//                         current_block = Some(&mut geographical_descriptor_block);
//                     }
//                     Some(FormatResult::Text(ref value)) if value == "GSE" => {
//                         current_block = Some(&mut geographical_data_block);
//                     }
//                     _ => {}
//                 },
//                 "EOM" => current_block = None,
//                 _ => {
//                     if let Some(block) = &mut current_block {
//                         block.add_line(data);
//                     }
//                 }
//             }
//         }

//         Self {
//             geographical_descriptor_block,
//             geographical_data_block,
//         }
//     }
// }

// #[derive(Debug)]
// pub struct GEOFile {
//     geo_reference_block: Block,
// }

// impl BlockParse for GEOFile {
//     fn parse<S: AsRef<str>>(lines: S) -> Self {
//         let mut geo_reference_block = Block::new("GEO");
//         let mut current_block: Option<&mut Block> = None;

//         for line in lines.as_ref().lines() {
//             if line.is_empty() {
//                 continue;
//             }
//             let data = Line::parse_line(&line);

//             match data.header.code.as_str() {
//                 "BOM" => {}
//                 "RTY" => match data.parsed_value {
//                     Some(FormatResult::Text(ref value)) if value == "GEO" => {
//                         current_block = Some(&mut geo_reference_block);
//                     }
//                     _ => {}
//                 },
//                 "EOM" => current_block = None,
//                 _ => {
//                     if let Some(block) = &mut current_block {
//                         block.add_line(data);
//                     }
//                 }
//             }
//         }

//         Self {
//             geo_reference_block,
//         }
//     }
// }

// #[derive(Debug)]
// pub struct DICFile {
//     object_definition_block: Block,
//     attribute_definition_block: Block,
//     semantic_relationship_block: Block,
// }

// impl BlockParse for DICFile {
//     fn parse<S: AsRef<str>>(lines: S) -> Self {
//         let mut object_definition_block = Block::new("DID");
//         let mut attribute_definition_block = Block::new("DIA");
//         let mut semantic_relationship_block = Block::new("DIR");
//         let mut current_block: Option<&mut Block> = None;

//         for line in lines.as_ref().lines() {
//             if line.is_empty() {
//                 continue;
//             }
//             let data = Line::parse_line(&line);

//             match data.header.code.as_str() {
//                 "BOM" => {}
//                 "RTY" => match data.parsed_value {
//                     Some(FormatResult::Text(ref value)) if value == "DID" => {
//                         current_block = Some(&mut object_definition_block);
//                     }
//                     Some(FormatResult::Text(ref value)) if value == "DIA" => {
//                         current_block = Some(&mut attribute_definition_block);
//                     }
//                     Some(FormatResult::Text(ref value)) if value == "DIR" => {
//                         current_block = Some(&mut semantic_relationship_block);
//                     }
//                     _ => {}
//                 },
//                 "EOM" => current_block = None,
//                 _ => {
//                     if let Some(block) = &mut current_block {
//                         block.add_line(data);
//                     }
//                 }
//             }
//         }

//         Self {
//             object_definition_block,
//             attribute_definition_block,
//             semantic_relationship_block,
//         }
//     }
// }

// #[derive(Debug)]
// pub struct SCDFile {
//     object_type_block: Block,
//     attribute_type_block: Block,
//     primitive_type_block: Block,
//     semantic_relationship_block: Block,
//     construction_relationship_type_block: Block,
// }

// impl BlockParse for SCDFile {
//     fn parse<S: AsRef<str>>(lines: S) -> Self {
//         let mut object_type_block = Block::new("OBJ");
//         let mut attribute_type_block = Block::new("ATT");
//         let mut primitive_type_block = Block::new("PGE");
//         let mut semantic_relationship_block = Block::new("ASS");
//         let mut construction_relationship_type_block = Block::new("REL");
//         let mut current_block: Option<&mut Block> = None;

//         for line in lines.as_ref().lines() {
//             if line.is_empty() {
//                 continue;
//             }
//             let data = Line::parse_line(&line);

//             match data.header.code.as_str() {
//                 "BOM" => {}
//                 "RTY" => match data.parsed_value {
//                     Some(FormatResult::Text(ref value)) if value == "OBJ" => {
//                         current_block = Some(&mut object_type_block);
//                     }
//                     Some(FormatResult::Text(ref value)) if value == "ATT" => {
//                         current_block = Some(&mut attribute_type_block);
//                     }
//                     Some(FormatResult::Text(ref value)) if value == "PGE" => {
//                         current_block = Some(&mut primitive_type_block);
//                     }
//                     Some(FormatResult::Text(ref value)) if value == "ASS" => {
//                         current_block = Some(&mut semantic_relationship_block);
//                     }
//                     Some(FormatResult::Text(ref value)) if value == "REL" => {
//                         current_block = Some(&mut construction_relationship_type_block);
//                     }
//                     _ => {}
//                 },
//                 "EOM" => current_block = None,
//                 _ => {
//                     if let Some(block) = &mut current_block {
//                         block.add_line(data);
//                     }
//                 }
//             }
//         }

//         Self {
//             object_type_block,
//             attribute_type_block,
//             primitive_type_block,
//             semantic_relationship_block,
//             construction_relationship_type_block,
//         }
//     }
// }

// #[derive(Debug)]
// pub struct QALFile {
//     descriptor_block: Block,
// }

// impl BlockParse for QALFile {
//     fn parse<S: AsRef<str>>(lines: S) -> Self {
//         let mut descriptor_block = Block::new("QUP");
//         let mut current_block: Option<&mut Block> = None;

//         for line in lines.as_ref().lines() {
//             if line.is_empty() {
//                 continue;
//             }
//             let data = Line::parse_line(&line);

//             match data.header.code.as_str() {
//                 "BOM" => {}
//                 "RTY" => match data.parsed_value {
//                     Some(FormatResult::Text(ref value)) if value == "QUP" => {
//                         current_block = Some(&mut descriptor_block);
//                     }
//                     _ => {}
//                 },
//                 "EOM" => current_block = None,
//                 _ => {
//                     if let Some(block) = &mut current_block {
//                         block.add_line(data);
//                     }
//                 }
//             }
//         }

//         Self { descriptor_block }
//     }
// }

// #[derive(Debug)]
// pub struct VECFile {
//     geographic_objects_blocks: Block,
//     node_descriptor_block: Block,
//     arc_descriptor_block: Block,
//     face_descriptor_block: Block,
//     relationship_descriptor_block: Block,
// }

// impl BlockParse for VECFile {
//     fn parse<S: AsRef<str>>(lines: S) -> Self {
//         let mut geographic_objects_blocks = Block::new("PNO");
//         let mut node_descriptor_block = Block::new("PAR");
//         let mut arc_descriptor_block = Block::new("PFE");
//         let mut face_descriptor_block = Block::new("FEA");
//         let mut relationship_descriptor_block = Block::new("LNK");
//         let mut current_block: Option<&mut Block> = None;

//         for line in lines.as_ref().lines() {
//             if line.is_empty() {
//                 continue;
//             }
//             let data = Line::parse_line(&line);

//             match data.header.code.as_str() {
//                 "BOM" => {}
//                 "RTY" => match data.parsed_value {
//                     Some(FormatResult::Text(ref value)) if value == "PNO" => {
//                         current_block = Some(&mut geographic_objects_blocks);
//                     }
//                     Some(FormatResult::Text(ref value)) if value == "PAR" => {
//                         current_block = Some(&mut node_descriptor_block);
//                     }
//                     Some(FormatResult::Text(ref value)) if value == "PFE" => {
//                         current_block = Some(&mut arc_descriptor_block);
//                     }
//                     Some(FormatResult::Text(ref value)) if value == "FEA" => {
//                         current_block = Some(&mut face_descriptor_block);
//                     }
//                     Some(FormatResult::Text(ref value)) if value == "LNK" => {
//                         current_block = Some(&mut relationship_descriptor_block);
//                     }
//                     _ => {}
//                 },
//                 "EOM" => current_block = None,
//                 _ => {
//                     if let Some(block) = &mut current_block {
//                         block.add_line(data);
//                     }
//                 }
//             }
//         }

//         Self {
//             geographic_objects_blocks,
//             node_descriptor_block,
//             arc_descriptor_block,
//             face_descriptor_block,
//             relationship_descriptor_block,
//         }
//     }
// }
