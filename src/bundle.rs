//! EdigeoBundle is a grouping of [`EDIGéO`] files from directories, such as [`EdigeoDir`].
use encoding_rs::WINDOWS_1252;
use std::{borrow::Cow, path::Path};

use crate::THFFile;

struct Bundle {
    pub thf: THFFile,
}

/// Represents a collections of Edigeo files for various file types.
///
/// This struct is designed to hold the data of each component
/// required for Edigeo data processing. Some files are mandatory, while
/// others are optional, depending on the context of usage.
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
    /// An [`EdigeoBundle`] completeness check. Check if all mandatory files are present in the
    /// exchange
    pub fn is_completed(&self) -> bool {
        !&self.thf.is_empty()
            && !&self.geo.is_empty()
            && !&self.t1.is_empty()
            && !&self.t2.is_empty()
            && !&self.t3.is_empty()
            && !&self.s1.is_empty()
            && !&self.qal.is_empty()
    }

    /// Raw `Bytes` are encoded in `Latin1 (WINDOWS_1252)` and are decoded to
    /// `UTF-8` strings
    pub fn decode_thf(&self) -> Cow<'_, str> {
        let (cow, _encoding_used, had_errors) = WINDOWS_1252.decode(&self.thf);
        if had_errors {
            eprintln!("Warning: Encoding errors occurred");
        }
        cow
    }
}
