
use super::*;

use crate::bfir::BFCode;
use crate::celltype::BFCell;
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
    pub fn pc(&self) -> &usize { &(self.pc) }
    pub fn jump(&mut self, new_pc: usize) { self.pc = new_pc }
}

