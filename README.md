<h1 align="center">
  <a href= "https://cadastre.data.gouv.fr/datasets/plan-cadastral-informatise">
  <img src="https://cadastre.data.gouv.fr/static/images/logos/cadastre.data.gouv.fr.svg">
  </a>
</h1>


![ci](https://github.com/duvenagep/edigeo/actions/workflows/main.yaml/badge.svg)


# EDIGéO Exchange Format

The [`EDIGéO`](https://www.data.gouv.fr/s/resources/plan-cadastral-informatise/20170906-150737/standard_edigeo_2013.pdf) (_Electronic Data Interchange in the field of Geographic Information_) format was established
by the French standards association (**AFNOR**). EDIGéO is a standardized format commonly used in France for
the exchange of geographical information.

The top-level data structure for an EDIGéO dataset is the exchange. An exchange appears as a single .THF file.
This file does not hold the main data; instead it specifies which lots belong to the exchange. An exchange,
therefore, consists of one or more lots. A lot in EDIGéO is conceptually a dataset. Within a lot, all data is
self-contained. Therefore, opening an exchange file with multiple lots is conceptually identical to opening
several exchange files each having one lot.

An EDIGéO lot is described in several plain text files. These files are listed below:
* `.THF` - a single batch of general data specific to the transmission (.THF extension, 1 per exchange)
* `.GEN` - contains general data indicating the structure and geographical extent of the data (one
per set)
* `.GEO` - contains the coordinate reference (example: one of the 9 conforming conic zones, one per set)
* `.QAL` - provides quality information (0 or 1 per set)
* `.DIC` - defines the nomenclature of objects, attributes and relationships (1 per set is mandatory
if there is at least one VEC file)
* `.SCD` - defines the conceptual data schema (1 per set is mandatory if there is at least one VEC file)
* `.MAT` - contains raster geographic data (0 or N per set; in the PCI/EDIGéO exchange, N = 0)
* `.VEC` - contains vector geographic data (0 or N per set; in the PCI/EDIGéO
exchange, N = 4)

## EDIGéO Bundle
The Edigeo exchange budle consists of the afore mentioned files however some of the files are optional.
The following structure highlights the minimum required files that constitute a valid exchange

```rust
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
```

If the min required files are not present the reader should `panic!` and the exchange should be considered incomplete or corrupted.

## Usage Examples
```rust
use edigeo::*;

let reader = EdigeoReader::new("exchange_file.tar.bz2");
let data = reader.into_inner().read_bundle();
let thf = decode_file(&data.thf);
let lines: Vec<&str> = thf.lines().filter(|l| !l.is_empty()).collect();
let pf = parse_blocks(lines);
println!("{:?}", pf);
```
