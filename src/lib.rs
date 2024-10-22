mod directory;
mod edigeo;
mod name;
use std::{fs, path::Path};

/// The Main [`EDIGéO`] struct that holdes paths to the min required files
#[derive(Debug, Default)]
pub struct EdigeoDir {
    thf: String,
    geo: String,
    qal: String,
    dic: Option<String>,
    gen: Option<String>,
    scd: Option<String>,
    t1: String,
    t2: String,
    t3: String,
    s1: String,
}

pub fn read<'a, P: AsRef<Path>>(path: P) -> String {
    let bytes = fs::read(path).unwrap();
    let contents = String::from_utf8_lossy(&bytes);
    contents.into_owned()
}
