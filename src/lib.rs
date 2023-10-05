//! `floral` is a command line interface and crate
//! to generate and format floral formulae. The style format
//! is inspired from two books:
//!
//! - Floral Diagrams (<i>Ronse De Crane</i>, 2010)
//! - Plant Systematics, A Phylogenetic Approach (<i>Judd et al.,</i> 4th Ed 2016)
//!
//! The floral formula is very well typed (perhaps overly so)
//! and the CLI interface works by ingesting a database of floral
//! formulae. This database is a CSV, with particular format needed
//! for each column, and in a specific layout. The particulars are outlined
//! in the README accompanying the repository <a href="https://github.com/Euphrasiologist/floral">here</a>.
//!
//! A couple of things! The current form of this code base is geared to a CLI, with not much of the
//! `floral` API exposed, or useful for re-use. This might change in the future.
//!
//! This is just a personal side project. Please do get involved and use if it is useful!

/// An error module to encompass the main errors that might occur when parsing, or
/// attempting to display a floral formula.
pub mod error;
/// A work in progress to implement an explain trait for each of the relevant floral
/// parts.
pub mod explain;
/// The main module containing all of the typed parts of a floral formula and mainly
/// [`Display`](std::fmt::Display) implementations on each of these.
pub mod floral;
/// Parse the input from the database into the [`Formula`] object.
pub mod parse;

/// Command line parsing specific to the tool
pub mod cli;
