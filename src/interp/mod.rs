use crate::bfir::{BFCell, BFCode};
use crate::error::ParseError;


#[derive(Debug)]
pub struct BFFrame<T: BFCell> {
    codes: Vec<BFCode<T>>,
    pc: usize,  // problem counter
}


impl<T> BFFrame<T>
    where T: BFCell {
    pub fn new(source_codes: impl std::io::BufRead) -> Result<BFFrame<T>, ParseError> {
        Ok(Self{ codes: BFCode::parse(source_codes)?, pc: 0 })
    }

    pub fn codes(&self) -> &Vec<BFCode<T>> { &(self.codes) }
}

