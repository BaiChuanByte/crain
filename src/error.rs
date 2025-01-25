use thiserror::Error;


#[derive(Error, Debug)]
pub enum ParseError {
    #[error("IO error: {source}")]
    IO { #[from] source: std::io::Error },
    
    #[error("Mismatched bracket: '{bracket}' at {line}:{row}")]
    MismatchedBracket { bracket: char, line: usize, row: usize },
}


#[derive(Error, Debug)]
pub enum VMError {
    #[error("IO error: {source}")]
    IO { #[from] source: std::io::Error },

    #[error("Pointer Overflow: {info}")]
    PointerOverflow { info: String },

    #[error("Cell Overflow: {info}")]
    CellOverflow { info: String },
}

#[derive(Error, Debug)]
pub enum InterpError {
    #[error("{source}")]
    Parse { #[from] source: ParseError },

    #[error("{source}")]
    VM { #[from] source: VMError },

    #[error("IO error: {source}")]
    IO { #[from] source: std::io::Error },
}

