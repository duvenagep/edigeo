use edigeo::*;

use std::{str::FromStr, time::Instant};

fn main() {
    let now = Instant::now();

    let _file = "data/edigeo-740240000A01/E0000A01.THF";
    let _dir = "data/edigeo-740240000A01/";
    let tar = "data/edigeo-740240000A01.tar.bz2";

    let reader = EdigeoReader::new(tar);
    let data = reader.read_bundle();
    let thf = decode_file(&data.thf);

    for line in thf.lines() {
        if !line.is_empty() {
            let d = Line::parse_line(line);
            // println!("{:?}", d);
        }
    }
    let blocks = THFFile::parse(thf);
    println!("{:?}", blocks);

    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.4?}");
}
