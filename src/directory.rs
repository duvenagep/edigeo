//! Contains all logic for processing [`EDIGéO`] files from directories, such as [`EdigeoDir`].

use std::{
    fs::{self},
    path::{Path, PathBuf},
};

/// Represents a directory of Edigeo files for various file types.
///
/// This struct is designed to hold the filenames of each component
/// required for Edigeo data processing. Some files are mandatory, while
/// others are optional, depending on the context of usage.
#[derive(Debug, Default)]
pub struct EdigeoDir {
    /// Path to the .thf file, containing metadata for Edigeo.
    pub thf: String,

    /// Path to the .geo file, containing geographical data.
    pub geo: String,

    /// Path to the .qal file, which includes quality attributes.
    pub qal: String,

    /// Path to the .t1 file, representing type-1 information.
    pub t1: String,

    /// Path to the .t2 file, representing type-2 information.
    pub t2: String,

    /// Path to the .t3 file, representing type-3 information.
    pub t3: String,

    /// Path to the .s1 file, representing supplementary data.
    pub s1: String,

    /// Optional path to the .dic file, containing dictionary data.
    pub dic: Option<String>,

    /// Optional path to the .gen file, which includes general data.
    pub gen: Option<String>,

    /// Optional path to the .scd file, including sector code data.
    pub scd: Option<String>,
}

impl EdigeoDir {
    /// Extracts specific Edigeo-related files from the provided directory path.
    ///
    /// This function scans the given directory and maps files with specific
    /// extensions (e.g., `.THF`, `.GEO`, `.QAL`) to the corresponding fields in
    /// `EdigeoDir`. Required files are stored as `String`, while optional files
    /// are stored as `Option<String>`.
    pub fn extract_files<P: AsRef<Path>>(path: P) -> Self {
        let files = path
            .as_ref()
            .read_dir()
            .unwrap()
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
                    _ => {}
                }
            }
            acc
        });
        dir
    }

    /// Lists all file names in the directory containing the specified `.thf` file.
    ///
    /// This function finds the parent directory of the given `.thf` file and
    /// collects the names of all files within it.
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
