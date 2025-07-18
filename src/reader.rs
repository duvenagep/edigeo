use crate::bundle::EdigeoBundle;
use bzip2::read::BzDecoder;
use encoding_rs::WINDOWS_1252;
use std::{
    borrow::Cow,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};
use tar::Archive;

pub fn decode_file(data: &[u8]) -> Cow<'_, str> {
    let (cow, _encoding_used, had_errors) = WINDOWS_1252.decode(data);
    if had_errors {
        eprintln!("Warning: Encoding errors occurred");
    }
    cow
}

pub trait ExchangeReader {
    fn read_bundle(&self) -> EdigeoBundle;
}

struct TarReader {
    path: PathBuf,
}

struct DirReader {
    path: PathBuf,
}

struct THFReader {
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

impl ExchangeReader for TarReader {
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
                p if p.ends_with(".DIC") => &mut bundle.dic.get_or_insert(Vec::new()),
                p if p.ends_with(".GEN") => &mut bundle.gen.get_or_insert(Vec::new()),
                p if p.ends_with(".SCD") => &mut bundle.scd.get_or_insert(Vec::new()),
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

impl ExchangeReader for DirReader {
    fn read_bundle(&self) -> EdigeoBundle {
        if self.path.is_file() {
            panic!("Expected Dir Path");
        }

        let mut bundle = EdigeoBundle::default();

        for entry in self.path.read_dir().unwrap() {
            let entry = entry.unwrap();
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
                p if p.ends_with(".DIC") => &mut bundle.dic.get_or_insert(Vec::new()),
                p if p.ends_with(".GEN") => &mut bundle.gen.get_or_insert(Vec::new()),
                p if p.ends_with(".SCD") => &mut bundle.scd.get_or_insert(Vec::new()),
                _ => continue,
            };
            file.read_to_end(target).unwrap();
        }

        if !bundle.is_completed() {
            panic!("All necesssary EIDGéO files not present.");
        }
        bundle
    }
}

impl ExchangeReader for THFReader {
    fn read_bundle(&self) -> EdigeoBundle {
        let dir = self.path.parent().unwrap();
        let dir_reader = DirReader::new(dir);
        dir_reader.read_bundle()
    }
}

enum Reader {
    Dir(DirReader),
    Tar(TarReader),
    File(THFReader),
}

pub struct EdigeoReader {
    reader: Reader,
}

impl EdigeoReader {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref().to_owned();

        let reader = match path.is_dir() {
            true => Reader::Dir(DirReader::new(path)),
            false => match path.extension().and_then(|ext| ext.to_str()) {
                Some("bz2") => Reader::Tar(TarReader::new(path)),
                Some("THF") => Reader::File(THFReader::new(path)),
                None | Some(_) => panic!("Invalid file format!"),
            },
        };

        Self { reader }
    }

    pub fn read_bundle(&self) -> EdigeoBundle {
        match self.into_inner() {
            Reader::Dir(dir_reader) => dir_reader.read_bundle(),
            Reader::Tar(tar_reader) => tar_reader.read_bundle(),
            Reader::File(thfreader) => thfreader.read_bundle(),
        }
    }

    pub fn with_tar<P: AsRef<Path>>(path: P) -> Self {
        Self {
            reader: Reader::Tar(TarReader::new(path)),
        }
    }

    pub fn with_thf<P: AsRef<Path>>(path: P) -> Self {
        Self {
            reader: Reader::File(THFReader::new(path)),
        }
    }

    pub fn with_dir<P: AsRef<Path>>(path: P) -> Self {
        Self {
            reader: Reader::Dir(DirReader::new(path)),
        }
    }

    fn into_inner(&self) -> &Reader {
        &self.reader
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edigeo_bundel_is_complete() {
        let bundle = EdigeoBundle::default();
        assert_eq!(false, bundle.is_completed());
    }

    #[test]
    #[should_panic]
    fn test_edigeo_bundel_is_complete_incorrect() {
        let bundle = EdigeoBundle::default();
        assert_eq!(true, bundle.is_completed());
    }
}
