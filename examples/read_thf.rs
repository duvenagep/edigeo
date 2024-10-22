use std::{
    ffi::OsString,
    fs::{self, ReadDir},
    path::{Path, PathBuf},
    vec,
};

extern crate edigeo;

fn main() {
    let thf = "data/edigeo-740240000A01/E0000A01.THF";
    let geo = "data/edigeo-740240000A01/ED0A01SE.GEO";

    let dir = "data/edigeo-740240000A01/";
    // read_dir(dir);

    let e = EdigeoDir::extract_files(dir);

    println!("{:#?}", e);

    let data = edigeo::read(e.geo);
    println!("{}", data);

    // let file_name = data.split("\r").collect::<Vec<&str>>();

    // println!("{:?}", file_name);
    // let names = list_files(thf);
    // println!("{:?}", names);
}

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

impl EdigeoDir {
    fn extract_files<P: AsRef<Path>>(path: P) -> Self {
        let files = path
            .as_ref()
            .read_dir()
            .unwrap()
            .into_iter()
            .map(|file| file.unwrap().path())
            .collect::<Vec<PathBuf>>();

        let dir = files.iter().fold(EdigeoDir::default(), |mut acc, num| {
            if let Some(ext) = num.to_str() {
                match ext {
                    x if x.ends_with(".THF") => acc.thf = num.to_string_lossy().to_string(),
                    x if x.ends_with(".GEO") => acc.geo = num.to_string_lossy().to_string(),
                    x if x.ends_with(".QAL") => acc.qal = num.to_string_lossy().to_string(),
                    x if x.ends_with("S1.VEC") => acc.s1 = num.to_string_lossy().to_string(),
                    x if x.ends_with("T1.VEC") => acc.t1 = num.to_string_lossy().to_string(),
                    x if x.ends_with("T2.VEC") => acc.t2 = num.to_string_lossy().to_string(),
                    x if x.ends_with("T3.VEC") => acc.t3 = num.to_string_lossy().to_string(),
                    x if x.ends_with(".DIC") => acc.dic = Some(num.to_string_lossy().to_string()),
                    x if x.ends_with(".SCD") => acc.scd = Some(num.to_string_lossy().to_string()),
                    x if x.ends_with(".GEN") => acc.gen = Some(num.to_string_lossy().to_string()),
                    _ => println!(""),
                }
            }
            acc // Return the updated accumulator
        });
        dir
    }

    fn read_dir<P: AsRef<Path>>(path: P) {
        if path.as_ref().is_dir() {
            let files = path
                .as_ref()
                .read_dir()
                .unwrap()
                .into_iter()
                .map(|file| file.unwrap().file_name())
                .collect::<Vec<OsString>>();

            println!("{:?}", files);

            for entry in path.as_ref().read_dir().unwrap() {
                println!("{:?}", entry.unwrap().path());
            }
            println!("Is dir s that's good!");
        }
    }

    fn list_files<P: AsRef<Path>>(path: P) -> Vec<String> {
        let thf_file = Path::new(path.as_ref());
        let mut names = Vec::new();

        if let Some(dir) = thf_file.parent() {
            if dir.is_dir() {
                for entry in fs::read_dir(dir).unwrap() {
                    let name = entry.unwrap().file_name().into_string().unwrap();
                    names.push(name);
                }
            }
        }
        names
    }
}
