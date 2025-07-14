//! Error types for the Crain Brainfuck interpreter and VM.
//!
//! This module defines error enums for parsing, virtual machine execution, and interpretation
//! in the Brainfuck toolchain. All errors implement the `std::error::Error` trait via `thiserror`.

use thiserror::Error;

/// Errors that may occur when parsing Brainfuck code.
#[derive(Error, Debug)]
pub enum ParseError {
    /// IO error occurred during parsing.
    #[error("IO error: {source}")]
    IO {
        #[from]
        source: std::io::Error,
    },

    /// Mismatched bracket found in the code.
    #[error("Mismatched bracket: '{bracket}' at {line}:{row}")]
    MismatchedBracket {
        bracket: char,
        line: usize,
        row: usize,
    },
}

/// Errors that may occur during Brainfuck VM execution.
#[derive(Error, Debug)]
pub enum VmError {
    /// IO error occurred during VM execution.
    #[error("IO error: {source}")]
    IO {
        #[from]
        source: std::io::Error,
    },

    /// The pointer moved out of the valid range.
    #[error("Pointer Overflow: {info}")]
    PointerOverflow { info: String },
    // It maybe used in the future.
    //  #[error("Cell Overflow: {info}")]
    //  CellOverflow { info: String },
}

/// Errors that may occur during interpretation, including parsing and VM errors.
#[derive(Error, Debug)]
pub enum InterpError {
    /// Error occurred during parsing.
    #[error("{source}")]
    Parse {
        #[from]
        source: ParseError,
    },

    /// Error occurred during VM execution.
    #[error("{source}")]
    Vm {
        #[from]
        source: VmError,
    },

    /// IO error occurred during interpretation.
    #[error("IO error: {source}")]
    IO {
        #[from]
        source: std::io::Error,
    },
}
