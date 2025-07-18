use std::io::BufRead;
use std::io::Write;

use crate::bfsetting::BfSetting;

/// The cell of the Brainfuck virtual machine.
///
/// Any type that acts as the cell for the Brainfuck virtual machine
/// must implement this trait.
/// This trait defines the atomic operations required for the Brainfuck virtual machine
/// to interact with the cell.
pub trait BfCell: Clone + Copy + ToString + From<u8> + Eq + PartialEq {
    /// Zero of the type.
    const ZERO: Self;
    /// One of the type.
    const ONE: Self;
    /// The number of bits in the cell.
    const BITS: u32;
    /// The size of the cell.
    const SIZE: usize = (Self::BITS as usize) / 8;

    /// Increases `self` by the given value.
    ///
    /// This will modify `self` in place and does not return a value.
    fn add(&mut self, rhs: Self);
    /// Decreases `self` by the given value.
    ///
    /// This will modify `self` in place and does not return a value.
    fn sub(&mut self, rhs: Self);

    /// Returns true if `self` is zero.
    fn iszero(self) -> bool;

    /// Reads bytes of the same length as `self` from the input stream and writes them into `self`.
    ///
    /// # Failures
    /// A `std::io::Error` will be returned if it occurs during the reading process.
    fn input(&mut self, bfinput: &mut impl BufRead, setting: &BfSetting) -> Result<(), std::io::Error>;
    /// Writes `self` to the output stream.
    ///
    /// # Failures
    /// A `std::io::Error` will be returned if it occurs during the writing process.
    fn output(self, bfoutput: &mut impl Write, setting: &BfSetting) -> Result<(), std::io::Error>;
}
