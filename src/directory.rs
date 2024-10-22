use std::{
    ffi::OsString,
    fs::{self},
    path::{Path, PathBuf},
};

/// The Main [`EDIGéO`] struct that holdes paths to the min required files
#[derive(Debug, Default)]
pub struct EdigeoDir {
    pub thf: String,
    pub geo: String,
    pub qal: String,
    pub t1: String,
    pub t2: String,
    pub t3: String,
    pub s1: String,
    pub dic: Option<String>,
    pub gen: Option<String>,
    pub scd: Option<String>,
}

impl EdigeoDir {
    pub fn extract_files<P: AsRef<Path>>(path: P) -> Self {
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
            acc
        });
        dir
    }

    pub fn read_dir<P: AsRef<Path>>(path: P) {
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

    pub fn list_files<P: AsRef<Path>>(path: P) -> Vec<String> {
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
