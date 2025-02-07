use crate::celltype::BfCell;

/// Intermediate code for brainfuck.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BfCode<T: BfCell> {
    /// Intermediate code of "+".
    /// The associated data is the number of consecutive occurrences of the "+" command.
    AddCell(T), // +

    /// Intermediate code of "-".
    /// The associated data is the number of consecutive occurrences of the "-" command.
    SubCell(T), // -

    /// Intermediate code of "<".
    /// The associated data is the number of consecutive occurrences of the "<" command.
    LeftShift(usize), // <

    /// Intermediate code of ">".
    /// The associated data is the number of consecutive occurrences of the ">" command.
    RightShift(usize), // >

    /// Intermediate code of ",".
    Input, // ,

    /// Intermediate code of ".".
    Output, // .

    /// Intermediate code of "[".
    /// The associated data is the position of the corresponding "]" command for this command.
    Jz(usize), // [

    /// Intermediate code of "]".
    /// The associated data is the position of the corresponding "[" command for this command.
    Jnz(usize), // ]
}
