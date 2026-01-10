use crate::ir::BfIr;
use crate::cell::BfCell;
use crate::error::ParseError;

/// An abstraction of a complete brainfuck program.
///
/// This structure can serve as an abstraction of a preprocessed brainfuck program
/// that can be directly executed by the VM.
/// The state during execution (such as the program counter) is also stored within this structure.
#[derive(Debug)]
pub struct BfFrame<T: BfCell> {
    codes: Vec<BfIr<T>>,
    pc: usize, // problem counter
}

impl<T> BfFrame<T>
where
    T: BfCell,
{
    /// News a BfFrame.
    ///
    /// # Failures
    ///
    /// The function will return a `ParseError` if:
    /// 1. An error occurs during an IO operation.
    /// 2. The left and right brackets do not match.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use crain::interp::BfFrame;
    /// use crain::celltype::Cell8;
    ///
    /// let frame = BfFrame::<Cell8>::new(Cursor::new("+-<>".to_string()))?;
    /// # Ok::<(), crain::ParseError>(())
    /// ```
    pub fn new(source_codes: impl std::io::BufRead) -> Result<BfFrame<T>, ParseError> {
        Ok(Self {
            codes: BfIr::parse(source_codes)?,
            pc: 0,
        })
    }

    /// Returns the preprocessed brainfuck program in the frame.
    pub fn codes(&self) -> &Vec<BfIr<T>> {
        &(self.codes)
    }
    /// Returns the program counter in the frame.
    pub fn pc(&self) -> &usize {
        &(self.pc)
    }
    /// Modify the program counter to implement jump.
    pub fn jump(&mut self, new_pc: usize) {
        self.pc = new_pc
    }
}
