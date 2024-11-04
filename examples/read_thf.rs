use bzip2::read::BzDecoder;
use edigeo::*;
use std::{fs::File, io::Read, path::Path, time::Instant};
use tar::Archive;

fn main() {
    let now = Instant::now();

    let file = "data/edigeo-740240000A01/E0000A01.THF";
    let dir = "data/edigeo-740240000A01/";
    let tar = "data/edigeo-740240000A01.tar.bz2";

    let tar_path = "data/edigeo-740240000A01.tar.bz2";

    // Open the .tar.bz2 file
    let file = File::open(tar_path).unwrap();

    // Decompress the .bz2 archive
    let decompressed = BzDecoder::new(file);

    // Create a tar archive from the decompressed data
    let mut archive = Archive::new(decompressed);

    // Iterate over the entries in the tar archive
    for entry in archive.entries().unwrap() {
        let mut e = entry.unwrap();
        // let p = e.path().unwrap();
        let mut buf = String::new();
        let data = e.read_to_string(&mut buf);
        println!("{}", buf);
    }

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

fn read<P: AsRef<Path>>(path: P) {
    let path = path.as_ref();

    if path.is_dir() {
        todo!()
    } else if path.is_file() && path.ends_with(".THF") {
        let dir = path.parent().unwrap();

        todo!()
    } else if path.is_file() && path.ends_with(".tar.bz2") {
        todo!()
    }
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
fn parse_blocks(lines: Vec<String>) -> ParsedFile {
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
