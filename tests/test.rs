use std::io::Read;

use edigeo::*;

#[test]
fn read_full_dir() {
    let full_dir_path = "data/edigeo-740240000A01/";
    let reader = EdigeoReader::new(full_dir_path);
    let e = reader.read_bundle();

    let all_required = !e.geo.is_empty()
        && !e.qal.is_empty()
        && !e.thf.is_empty()
        && !e.t1.is_empty()
        && !e.t2.is_empty()
        && !e.t3.is_empty()
        && !e.s1.is_empty();

    // Assert that min required files are present
    assert!(all_required);
}

#[test]
#[should_panic]
fn read_missing_dir() {
    let full_dir_path = "data/edigeo-740240000A01-missing";
    let reader = EdigeoReader::new(full_dir_path);
    let e = reader.read_bundle();

    let all_required = !e.geo.is_empty()
        && !e.qal.is_empty()
        && !e.thf.is_empty()
        && !e.t1.is_empty()
        && !e.t2.is_empty()
        && !e.t3.is_empty()
        && !e.s1.is_empty();

    // Assert that min required files are present -> This test should panic because
    // the t3.vec file is missing
    assert!(all_required);
}

#[test]
fn edigeo_read_thf() {
    let full_dir_path = "data/edigeo-740240000A01/E0000A01.THF";
    let e = EdigeoReader::new(full_dir_path).read_bundle();

    let mut thf = Vec::new();
    let file = std::fs::File::open(full_dir_path).unwrap();
    let mut reader = std::io::BufReader::new(file);
    reader
        .read_to_end(&mut thf)
        .expect("Failed to read THF from path");

    assert_eq!(thf, e.thf);
}
