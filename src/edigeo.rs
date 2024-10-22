use std::{fs, path::Path};

/// The [`.THF`] file defines a single batch of general data specific to the transmission.
pub struct THF;

/// The [`.GEN`] file contains general data indicating the structure and geographical extent of the data (one per set).
pub struct GEN;

/// The [`.GEO`] file  contains the coordinate reference (example: one of the 9 conforming conic zones, one per set).
pub struct GEO;

/// The [`.QAL`] file provides quality information (0 or 1 per set).
pub struct QAL;

/// The [`.DIC`] file defines the nomenclature of objects, attributes and relationships (1 per set is mandatory if there is at least one VEC file).
pub struct DIC;

/// The [`.SCD`] file defines the conceptual data schema (1 per set is mandatory if there is at least one VEC file).
pub struct SCD;

/// The [`.VEC`] file contains vector geographic data (0 or N per set; in the PCI/EDIGéO exchange, N = 4).
pub struct VEC;

/// The [`.MAT`] file dcontains raster geographic data (0 or N per set; in the PCI/EDIGéO exchange, N = 0).
pub struct MAT;

/// The [`EDIGéO`] Echange file format consists of the following structure
pub struct Edigeo {
    thf: THF,
    gen: GEN,
    qal: Option<QAL>,
    dic: Option<DIC>,
    scd: Option<SCD>,
    vec: Option<Vec<VEC>>,
    mat: Option<MAT>,
}

impl Edigeo {
    pub fn read<'a, P: AsRef<Path>>(path: P) -> String {
        let bytes = fs::read(path).unwrap();
        let contents = String::from_utf8_lossy(&bytes);
        contents.into_owned()
    }
}
