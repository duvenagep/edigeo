use edigeo::*;

use std::{path::Path, time::Instant};

fn main() {
    let now = Instant::now();

    let _file = "data/edigeo-740240000A01/E0000A01.THF";
    let _dir = "data/edigeo-740240000A01/";
    let tar = "data/edigeo-740240000A01.tar.bz2";

    let reader = EdigeoReader::new(tar);
    let data = reader.into_inner().read_bundle();
    let thf = decode_file(&data.thf);
    let lines: Vec<&str> = thf.lines().filter(|l| !l.is_empty()).collect();
    let pf = parse_blocks(lines);
    println!("{:?}", pf);
    // for line in lines {
    //     let l = Line::parse_line(&line);
    //     println!("{:?}", l);
    // }

    // let e = EdigeoDir::extract_files(dir);

    // if let Ok(lines) = EdigeoDir::read_lines_efficient(e.thf) {
    //     let pf = parse_blocks(lines);
    //     println!("{:#?}", pf);
    //     // for line in lines {
    //     //     // println!("{line}");
    //     //     let data = Line::parse_line(&line);
    //     //     println!("{:?}", data);
    //     // }
    // }

    // let e = EdigeoDir::extract_files(dir);

    // if let Ok(lines) = EdigeoDir::read_lines_efficient(e.thf) {
    //     let pf = parse_blocks(lines);
    //     println!("{:#?}", pf);
    //     // for line in lines {
    //     //     // println!("{line}");
    //     //     let data = Line::parse_line(&line);
    //     //     println!("{:?}", data);
    //     // }
    // }

    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.4?}");
}

#[derive(Debug)]
struct ParsedFile {
    support_block: Option<Block>,
    batch_block: Option<Block>,
}

#[derive(Debug)]
struct Block {
    block_type: String,
    entries: Vec<Line>,
}

/// Parses the data and categorizes it into a support block and a batch block.
fn parse_blocks(lines: Vec<&str>) -> ParsedFile {
    let mut support_block = None;
    let mut batch_block = None;
    let mut current_block = None;

    for line in lines {
        if !line.is_empty() {
            let data = Line::parse_line(&line);
            match data.header.code.as_str() {
                "BOM" => {
                    // Start of file (BOMT)
                }
                "CSE" => {
                    // Character set (CSET)
                }
                "RTY" => {
                    // Block switch based on "GTS" or "GTL"
                    if data.parsed_value == Some(FormatResult::Text("GTS".to_string())) {
                        // Save any current block to batch_block if it exists
                        if let Some(block) = current_block.take() {
                            batch_block = Some(block);
                        }
                        // Start a new support block
                        current_block = Some(Block {
                            block_type: "GTS".to_string(),
                            entries: Vec::new(),
                        });
                    } else if data.parsed_value == Some(FormatResult::Text("GTL".to_string())) {
                        // Save any current block to support_block if it exists
                        if let Some(block) = current_block.take() {
                            support_block = Some(block);
                        }
                        // Start a new batch block
                        current_block = Some(Block {
                            block_type: "GTL".to_string(),
                            entries: Vec::new(),
                        });
                    }
                }
                "EOM" => {
                    // End of file (EOMT)
                    if let Some(block) = current_block.take() {
                        batch_block = Some(block);
                    }
                }
                _ => {
                    // Collect entries into the current block
                    if let Some(ref mut block) = current_block {
                        block.entries.push(data);
                    }
                }
            }
        }
    }

    ParsedFile {
        support_block,
        batch_block,
    }
}
