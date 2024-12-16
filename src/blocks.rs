//! Contains all logic for processing **Blocks**.
use crate::{Code, FormatResult, KeyWordCode, Line};

/// Block Identifiers
#[derive(Debug)]
enum BlockIdent {
    /// Indentifier value for THF Support Descriptor Block
    GTS,
    /// Indentifier value for THF Batch Descriptor Block
    GTL,
    /// Indentifier value for GEN Geographical Descriptor Block
    DEG,
    /// Indentifier value for GEN Geographical Data Block
    GSE,
}

#[derive(Debug)]
struct Block {
    _id: BlockIdent,
    entries: Vec<Line>,
}

impl Block {
    fn new(id: BlockIdent) -> Self {
        Self {
            _id: id,
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

impl BlockParse for THFFile {
    fn parse<S: AsRef<str>>(lines: S) -> Self {
        let mut support_block = Block::new(BlockIdent::GTS);
        let mut batch_block = Block::new(BlockIdent::GTL);
        let mut current_block: Option<&mut Block> = None;

        for line in lines.as_ref().lines() {
            if line.is_empty() {
                continue;
            }
            let data = Line::parse_line(&line);

            match data.header.code {
                Code::KWCode(KeyWordCode::BOM) => {}
                Code::RTY => match data.parsed_value {
                    Some(FormatResult::Text(ref value)) if value == "GTS" => {
                        current_block = Some(&mut support_block);
                    }
                    Some(FormatResult::Text(ref value)) if value == "GTL" => {
                        current_block = Some(&mut batch_block);
                    }
                    _ => {}
                },
                Code::KWCode(KeyWordCode::EOM) => current_block = None,
                _ => {
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

#[derive(Debug)]
pub struct GENFile {
    geographical_descriptor_block: Block,
    geographical_data_block: Block,
}

impl BlockParse for GENFile {
    fn parse<S: AsRef<str>>(lines: S) -> Self {
        let mut geographical_descriptor_block = Block::new(BlockIdent::DEG);
        let mut geographical_data_block = Block::new(BlockIdent::GSE);
        let mut current_block: Option<&mut Block> = None;

        for line in lines.as_ref().lines() {
            if line.is_empty() {
                continue;
            }
            let data = Line::parse_line(&line);

            match data.header.code {
                Code::KWCode(KeyWordCode::BOM) => {}
                Code::RTY => match data.parsed_value {
                    Some(FormatResult::Text(ref value)) if value == "DEG" => {
                        current_block = Some(&mut geographical_descriptor_block);
                    }
                    Some(FormatResult::Text(ref value)) if value == "GSE" => {
                        current_block = Some(&mut geographical_data_block);
                    }
                    _ => {}
                },
                Code::KWCode(KeyWordCode::EOM) => current_block = None,
                _ => {
                    if let Some(block) = &mut current_block {
                        block.add_line(data);
                    }
                }
            }
        }

        Self {
            geographical_descriptor_block,
            geographical_data_block,
        }
    }
}
