use crate::Line;

#[derive(Debug)]
pub enum BlockTypes {
    FEA,
    LNK,
    PNO,
    PAR,
    PFE,
    GEO,
    QUP,
}

pub trait ParseBlock {
    fn parse_block(&self) -> Self;
}

pub struct THFFile {
    data: Vec<u8>,
    blocks: Vec<Block>,
}

impl ParseBlock for THFFile {
    fn parse_block(&self) -> Self {
        todo!()
    }
}

#[derive(Debug)]
struct Block {
    id: String,
    block_type: BlockTypes,
    entries: Vec<Line>,
}

impl Block {
    fn new() {
        todo!()
    }
}

// #[derive(Debug)]
// struct ParsedFile {
//     support_block: Option<Block>,
//     batch_block: Option<Block>,
// }

// #[derive(Debug)]
// struct Block {
//     block_type: String,
//     entries: Vec<Line>,
// }

// /// Parses the data and categorizes it into a support block and a batch block.
// fn parse_blocks(lines: Vec<&str>) -> ParsedFile {
//     let mut support_block = None;
//     let mut batch_block = None;
//     let mut current_block = None;

//     for line in lines {
//         if !line.is_empty() {
//             let data = Line::parse_line(&line);
//             match data.header.code.as_str() {
//                 "BOM" => {
//                     // Start of file (BOMT)
//                 }
//                 "CSE" => {
//                     // Character set (CSET)
//                 }
//                 "RTY" => {
//                     // Block switch based on "GTS" or "GTL"
//                     if data.parsed_value == Some(FormatResult::Text("GTS".to_string())) {
//                         // Save any current block to batch_block if it exists
//                         if let Some(block) = current_block.take() {
//                             batch_block = Some(block);
//                         }
//                         // Start a new support block
//                         current_block = Some(Block {
//                             block_type: "GTS".to_string(),
//                             entries: Vec::new(),
//                         });
//                     } else if data.parsed_value == Some(FormatResult::Text("GTL".to_string())) {
//                         // Save any current block to support_block if it exists
//                         if let Some(block) = current_block.take() {
//                             support_block = Some(block);
//                         }
//                         // Start a new batch block
//                         current_block = Some(Block {
//                             block_type: "GTL".to_string(),
//                             entries: Vec::new(),
//                         });
//                     }
//                 }
//                 "EOM" => {
//                     // End of file (EOMT)
//                     if let Some(block) = current_block.take() {
//                         batch_block = Some(block);
//                     }
//                 }
//                 _ => {
//                     // Collect entries into the current block
//                     if let Some(ref mut block) = current_block {
//                         block.entries.push(data);
//                     }
//                 }
//             }
//         }
//     }

//     ParsedFile {
//         support_block,
//         batch_block,
//     }
// }
