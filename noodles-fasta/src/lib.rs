//! **noodles-fasta** handles and reading and writing of the FASTA format.
//!
//! FASTA is a text format with no formal specification and only has de facto rules. It typically
//! consists of a list of records, each with a definition on the first line and a sequence in the
//! following lines.
//!
//! The definition starts with a `>` (greater than) character, and directly after it is the
//! reference sequence name. Optionally, whitespace may be used a delimiter for an extra
//! description or metadata of the sequence. For example,
//!
//! ```text
//!  reference sequence name
//!  | |
//! >sq0 LN:13
//!      |   |
//!      description
//! ```
//!
//! The sequence is effectively a byte array of characters representing a base. It is typically
//! hard wrapped at an arbitrary width. For example, the following makes up the sequence
//! `ACGTNACTGG`.
//!
//! ```text
//! ACGT
//! NACT
//! GG
//! ```
//!
//! # Examples
//!
//! ## Read all records in a FASTA file
//!
//! ```no_run
//! # use std::{fs::File, io::{self, BufReader}};
//! use noodles_fasta as fasta;
//!
//! let mut reader = File::open("reference.fa")
//!     .map(BufReader::new)
//!     .map(fasta::io::Reader::new)?;
//!
//! for result in reader.records() {
//!     let record = result?;
//!     // ...
//! }
//! # Ok::<(), io::Error>(())
//! ```

#[cfg(feature = "async")]
pub mod r#async;

pub mod fai;
pub mod fs;
pub mod io;
pub mod record;
pub mod repository;
pub mod sequence;

pub use self::{record::Record, repository::Repository};
