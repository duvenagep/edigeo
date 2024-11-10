use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use bzip2::read::BzDecoder;
use tar::Archive;

#[derive(Debug, Default)]
pub struct EdigeoBundle {
    /// Path to the .thf file, containing metadata for Edigeo.
    pub thf: Vec<u8>,
    /// Path to the .geo file, containing geographical data.
    pub geo: Vec<u8>,
    /// Path to the .qal file, which includes quality attributes.
    pub qal: Vec<u8>,
    /// Path to the .t1 file, representing type-1 information.
    pub t1: Vec<u8>,
    /// Path to the .t2 file, representing type-2 information.
    pub t2: Vec<u8>,
    /// Path to the .t3 file, representing type-3 information.
    pub t3: Vec<u8>,
    /// Path to the .s1 file, representing supplementary data.
    pub s1: Vec<u8>,
    /// Optional path to the .dic file, containing dictionary data.
    pub dic: Option<Vec<u8>>,
    /// Optional path to the .gen file, which includes general data.
    pub gen: Option<Vec<u8>>,
    /// Optional path to the .scd file, including sector code data.
    pub scd: Option<Vec<u8>>,
}

impl EdigeoBundle {
    pub fn is_completed(&self) -> bool {
        !&self.thf.is_empty()
            && !&self.geo.is_empty()
            && !&self.t1.is_empty()
            && !&self.t2.is_empty()
            && !&self.t3.is_empty()
            && !&self.s1.is_empty()
            && !&self.qal.is_empty()
    }
}

pub trait EdigeoReader {
    fn read_bundle(&self) -> EdigeoBundle;
}

pub struct TarReader {
    path: PathBuf,
}

pub struct DirReader {
    path: PathBuf,
}

pub struct THFReader {
    path: PathBuf,
}

impl TarReader {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_owned(),
        }
    }
}

impl DirReader {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_owned(),
        }
    }
}

impl THFReader {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_owned(),
        }
    }
}

impl EdigeoReader for TarReader {
    fn read_bundle(&self) -> EdigeoBundle {
        let file = std::fs::File::open(&self.path).unwrap();
        let bz2_decoder = BzDecoder::new(file);
        let mut archive = Archive::new(bz2_decoder);
        let mut bundle = EdigeoBundle::default();

        for entry in archive.entries().unwrap() {
            let mut entry = entry.unwrap();
            let path = entry.path().unwrap();
            let path_str = path.to_string_lossy();

            let target = match path_str {
                p if p.ends_with(".THF") => &mut bundle.thf,
                p if p.ends_with(".GEO") => &mut bundle.geo,
                p if p.ends_with("T1.VEC") => &mut bundle.t1,
                p if p.ends_with("T2.VEC") => &mut bundle.t2,
                p if p.ends_with("T3.VEC") => &mut bundle.t3,
                p if p.ends_with("S1.VEC") => &mut bundle.s1,
                p if p.ends_with(".QAL") => &mut bundle.qal,
                _ => continue,
            };

            entry.read_to_end(target).unwrap();
        }
        if !bundle.is_completed() {
            panic!("All necesssary EIDGéO files not present.");
        }
        bundle
    }
}

impl EdigeoReader for DirReader {
    fn read_bundle(&self) -> EdigeoBundle {
        if self.path.is_file() {
            panic!("Expected Dir Path");
        }
        let mut bundle = EdigeoBundle::default();

        for entry in self.path.read_dir().unwrap() {
            let mut entry = entry.unwrap();
            let path = entry.path();
            let path_str = &path.to_string_lossy();
            let mut file = File::open(&path).unwrap();

            let target = match path_str {
                p if p.ends_with(".THF") => &mut bundle.thf,
                p if p.ends_with(".GEO") => &mut bundle.geo,
                p if p.ends_with("T1.VEC") => &mut bundle.t1,
                p if p.ends_with("T2.VEC") => &mut bundle.t2,
                p if p.ends_with("T3.VEC") => &mut bundle.t3,
                p if p.ends_with("S1.VEC") => &mut bundle.s1,
                p if p.ends_with(".QAL") => &mut bundle.qal,
                _ => continue,
            };
            file.read_to_end(target).unwrap();
        }

        bundle
    }
}

impl EdigeoReader for THFReader {
    fn read_bundle(&self) -> EdigeoBundle {
        let dir = self.path.parent().unwrap();
        let dir_reader = DirReader::new(dir);
        dir_reader.read_bundle()
    }
}

pub struct Reader {
    pub reader: Box<dyn EdigeoReader>,
}

impl Reader {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref().to_owned();

        let reader: Box<dyn EdigeoReader> = match path.is_dir() {
            true => Box::new(DirReader::new(path)),
            false => match path.extension().and_then(|ext| ext.to_str()) {
                Some("bz2") => Box::new(TarReader::new(path)),
                Some("THF") => Box::new(THFReader::new(path)),
                None | Some(_) => panic!("Invalid file format!"),
            },
        };

        Self { reader }
    }

    pub fn with_tar<P: AsRef<Path>>(path: P) -> Self {
        Self {
            reader: Box::new(TarReader::new(path)),
        }
    }

    pub fn with_thf<P: AsRef<Path>>(path: P) -> Self {
        Self {
            reader: Box::new(THFReader::new(path)),
        }
    }

    pub fn with_dir<P: AsRef<Path>>(path: P) -> Self {
        Self {
            reader: Box::new(DirReader::new(path)),
        }
    }

    pub fn into_inner(&self) -> &Box<dyn EdigeoReader> {
        &self.reader
    }
}
