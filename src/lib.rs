#![warn(missing_docs)]
//! # [`EDIGéO`](https://www.data.gouv.fr/s/resources/plan-cadastral-informatise/20170906-150737/standard_edigeo_2013.pdf) Exchange Format
//!
//! The EDIGéO (Electronic Data Interchange in the field of Geographic Information) format was established
//! by the French standards association (AFNOR). EDIGéO is a standardized format commonly used in France for
//! the exchange of geographical information.
//!
//! The top-level data structure for an EDIGéO dataset is the exchange. An exchange appears as a single .THF file.
//! This file does not hold the main data; instead it specifies which lots belong to the exchange. An exchange,
//! therefore, consists of one or more lots. A lot in EDIGéO is conceptually a dataset. Within a lot, all data is
//! self-contained. Therefore, opening an exchange file with multiple lots is conceptually identical to opening
//! several exchange files each having one lot.
//!
//! An EDIGéO lot is described in several plain text files. These files are listed below:
//! `.GEN` - General Information
pub mod blocks;
pub mod error;
pub mod format;
pub mod header;
pub mod line;
pub mod reader;

pub use blocks::*;
pub use format::*;
pub use header::*;
pub use line::*;
pub use reader::*;
