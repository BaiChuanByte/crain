//! The definition and parsing of the intermediate code for brainfuck.
//!
//! This module defines the brainfuck intermediate code,
//! and provides a function to parse brainfuck code into the intermediate code.

mod bfir;
mod parser;

pub use bfir::*;
