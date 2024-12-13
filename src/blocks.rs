//! Contains all logic for processing **Blocks**.
use crate::{FormatResult, Line};

// /// BlockTypes Enum with variants for each Block
// #[derive(Debug)]
// pub enum BlockTypes {
//     FEA,
//     LNK,
//     PNO,
// //     PAR,
// //     PFE,
// //     GEO,
// //     QUP,
// // }

// #[derive(Debug)]
// enum BlockIdent {
//     /// Indentifier value for THF Support Descriptor Block
//     GTS,
//     /// Indentifier value for THF Batch Descriptor Block
//     GTL,
// }

// #[derive(Debug)]
// struct Block {
//     id: BlockIdent,
//     entries: Vec<Line>,
// }

// impl Block {
//     fn new(id: BlockIdent) -> Self {
//         Self {
//             id,
//             entries: Vec::new(),
//         }
//     }

//     fn add_line(&mut self, line: Line) {
//         self.entries.push(line);
//     }
// }

// #[derive(Debug)]
// struct THFFile {
//     support_block: Block,
//     batch_block: Block,
// }

// impl THFFile {
//     fn new(&self, lines: Vec<Line>) -> Self {
//         let mut support_block = Block::new(BlockIdent::GTS);
//         let mut batch_block = Block::new(BlockIdent::GTL);
//         let mut current = support_block;

//         for line in lines {
//             if !line.is_empty() {
//                 match line.header.code.as_str() {
//                     "BOM" | "CSE" => {}
//                     "RTY" => {
//                         if line.parsed_value == Some(FormatResult::Text("GTS".to_string())) {
//                             if let Some(block) = current.take() {
//                                 batch_block = block;
//                             }

//                             current = support_block;
//                         } else if line.parsed_value == Some(FormatResult::Text("GTL".to_string())) {
//                             if let Some(block) = current.take() {
//                                 support_block = block;
//                             }
//                             current = batch_block;
//                         }
//                     }
//                     "EOM" => {}
//                     _ => {
//                         if let Some(ref mut block) = current {
//                             block.entries.push(line);
//                         }
//                     }
//                 }
//             }
//         }

//         Self {
//             support_block,
//             batch_block,
//         }
//     }
// }

// // #[derive(Debug)]
// // struct ParsedFile {
// //     support_block: Option<Block>,
// //     batch_block: Option<Block>,
// // }

// // #[derive(Debug)]
// // struct Block {
// //     block_type: String,
// //     entries: Vec<Line>,
// // }

// // /// Parses the data and categorizes it into a support block and a batch block.
// // fn parse_blocks(lines: Vec<&str>) -> ParsedFile {
// //     let mut support_block = None;
// //     let mut batch_block = None;
// //     let mut current_block = None;

// //     for line in lines {
// //         if !line.is_empty() {
// //             let data = Line::parse_line(&line);
// //             match data.header.code.as_str() {
// //                 "BOM" => {
// //                     // Start of file (BOMT)
// //                 }
// //                 "CSE" => {
// //                     // Character set (CSET)
// //                 }
// //                 "RTY" => {
// //                     // Block switch based on "GTS" or "GTL"
// //                     if data.parsed_value == Some(FormatResult::Text("GTS".to_string())) {
// //                         // Save any current block to batch_block if it exists
// //                         if let Some(block) = current_block.take() {
// //                             batch_block = Some(block);
// //                         }
// //                         // Start a new support block
// //                         current_block = Some(Block {
// //                             block_type: "GTS".to_string(),
// //                             entries: Vec::new(),
// //                         });
// //                     } else if data.parsed_value == Some(FormatResult::Text("GTL".to_string())) {
// //                         // Save any current block to support_block if it exists
// //                         if let Some(block) = current_block.take() {
// //                             support_block = Some(block);
// //                         }
// //                         // Start a new batch block
// //                         current_block = Some(Block {
// //                             block_type: "GTL".to_string(),
// //                             entries: Vec::new(),
// //                         });
// //                     }
// //                 }
// //                 "EOM" => {
// //                     // End of file (EOMT)
// //                     if let Some(block) = current_block.take() {
// //                         batch_block = Some(block);
// //                     }
// //                 }
// //                 _ => {
// //                     // Collect entries into the current block
// //                     if let Some(ref mut block) = current_block {
// //                         block.entries.push(data);
// //                     }
// //                 }
// //             }
// //         }
// //     }

// //     ParsedFile {
// //         support_block,
// //         batch_block,
// //     }
// // }
