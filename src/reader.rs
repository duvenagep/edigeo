//! Contains all logic for processing [`EDIGéO`] files from directories, such as [`EdigeoDir`].
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

/// Raw `Bytes` are encoded in `Latin1 (WINDOWS_1252)` and are decoded to
/// `UTF-8` strings
pub fn decode_file(data: &[u8]) -> Cow<'_, str> {
    let (cow, _encoding_used, had_errors) = WINDOWS_1252.decode(data);
    if had_errors {
        eprintln!("Warning: Encoding errors occurred");
    }
    cow
}

/// The [`ExchangeReader`] Trait used for reading the [`EdigeoBundle`] from various sources
/// namely:
/// - `.thf` file
/// - `.tar.bz2` compressed file
/// - `directory` where .thf file is located
pub trait ExchangeReader {
    /// Reads each of the mandatory files and builds a [`EdigeoBundle`] struct.
    fn read_bundle(&self) -> EdigeoBundle;
}

/// Tar file `.tar.bz2` is the most common exchange format for Edigeo
struct TarReader {
    path: PathBuf,
}

/// The directory where the main `.THF` file is located
struct DirReader {
    path: PathBuf,
}

/// The main `.THF` file reader
struct THFReader {
    path: PathBuf,
}

impl TarReader {
    /// Constructor method for creating a new [`TarReader`].
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_owned(),
        }
    }
}

impl DirReader {
    /// Constructor method for creating a new [`DirReader`].
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_owned(),
        }
    }
}

impl THFReader {
    /// Constructor method for creating a new [`THFReader`].
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

/// The main Edigeo Excahnge can be read from 3 different sources
/// The [`Reader`] enum has these 3 variants
/// - Directories
/// - Tar Archives
/// - `.THF` files
enum Reader {
    /// Directory with all necessary [`EdigeoBundle`] files
    Dir(DirReader),
    /// `.tar.bz2` archive file with all necessary [`EdigeoBundle`] files
    Tar(TarReader),
    /// `.thf` file with all necessary [`EdigeoBundle`] files in same directory
    File(THFReader),
}

/// The main EdigeoReader struct that enables reading any input file type.
/// ```ignore
///     let file = "data/edigeo-740240000A01/E0000A01.THF";
///     let reader = EdigeoReader::new(file);
///     let data = reader.read_bundle();
///
///     println!("{}", data.decode_file(&data.thf));
/// ```
pub struct EdigeoReader {
    /// Enum representing the Reader variants to read the [`ExchangeReader`]
    reader: Reader,
}

impl EdigeoReader {
    /// Constructor method to create a [`EdigeoReader`] from any object that can be
    /// [`AsRef<Path>`] into a path.
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

    /// Consumes the [`EdigeoReader`] and returns the [`EdigeoBundle`]
    pub fn read_bundle(&self) -> EdigeoBundle {
        match self.into_inner() {
            Reader::Dir(dir_reader) => dir_reader.read_bundle(),
            Reader::Tar(tar_reader) => tar_reader.read_bundle(),
            Reader::File(thfreader) => thfreader.read_bundle(),
        }
    }

    /// Create a reader `with_tar` to create a TAR file reader
    pub fn with_tar<P: AsRef<Path>>(path: P) -> Self {
        Self {
            reader: Reader::Tar(TarReader::new(path)),
        }
    }

    /// Create a reader `with_thf` to create a .THF file reader
    pub fn with_thf<P: AsRef<Path>>(path: P) -> Self {
        Self {
            reader: Reader::File(THFReader::new(path)),
        }
    }

    /// Create a reader `with_dir` to create a Directory reader
    pub fn with_dir<P: AsRef<Path>>(path: P) -> Self {
        Self {
            reader: Reader::Dir(DirReader::new(path)),
        }
    }

    /// Returns the inner [`EdigeoExchange`] Reader Enum
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
