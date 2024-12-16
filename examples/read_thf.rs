use edigeo::*;

use std::time::Instant;

fn main() {
    let now = Instant::now();

    let _file = "data/edigeo-740240000A01/E0000A01.THF";
    let _dir = "data/edigeo-740240000A01/";
    let tar = "data/edigeo-740240000A01.tar.bz2";

    let reader = EdigeoReader::new(tar);
    let data = reader.read_bundle();
    let thf = decode_file(&data.thf);
    let blocks = THFFile::parse(thf);
    println!("{:?}", blocks);

    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.4?}");
}
