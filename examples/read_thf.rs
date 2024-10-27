use edigeo::*;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let dir = "data/edigeo-740240000A01/";

    let e = EdigeoDir::extract_files(dir);

    if let Ok(lines) = EdigeoDir::read_lines_efficient(e.thf) {
        for line in lines {
            println!("{}", line);
            // if !line.is_empty() {
            //     let data = Line::parse_line(&line);
            //     println!("{:?}", data);
            // }
        }
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.4?}");
}

pub struct SupportDescriptorBlock {
    pub support: Vec<Line>,
}

pub struct BatchDescriptorBlock {
    pub batch: Vec<Line>,
}

pub struct THFFile {
    pub support: SupportDescriptorBlock,
    pub batch: BatchDescriptorBlock,
}
