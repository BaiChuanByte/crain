use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("IO error: {source}")]
    IO {
        #[from]
        source: std::io::Error,
    },

    #[error("Mismatched bracket: '{bracket}' at {line}:{row}")]
    MismatchedBracket {
        bracket: char,
        line: usize,
        row: usize,
    },
}

#[derive(Error, Debug)]
pub enum VmError {
    #[error("IO error: {source}")]
    IO {
        #[from]
        source: std::io::Error,
    },

    #[error("Pointer Overflow: {info}")]
    PointerOverflow { info: String },
    // It maybe used in the future.
    //  #[error("Cell Overflow: {info}")]
    //  CellOverflow { info: String },
}

#[derive(Error, Debug)]
pub enum InterpError {
    #[error("{source}")]
    Parse {
        #[from]
        source: ParseError,
    },

    #[error("{source}")]
    Vm {
        #[from]
        source: VmError,
    },

    #[error("IO error: {source}")]
    IO {
        #[from]
        source: std::io::Error,
    },
}
