use crate::cell::BfCell;

/// The intermediate representation for brainfuck.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BfIr<T: BfCell> {
    /// The IR of "+".
    /// The associated data is the number of consecutive occurrences of the "+" command.
    AddCell(T), // +

    /// The IR of "-".
    /// The associated data is the number of consecutive occurrences of the "-" command.
    SubCell(T), // -

    /// The IR of "<".
    /// The associated data is the number of consecutive occurrences of the "<" command.
    LeftShift(usize), // <

    /// The IR of ">".
    /// The associated data is the number of consecutive occurrences of the ">" command.
    RightShift(usize), // >

    /// The IR of ",".
    Input, // ,

    /// The IR of ".".
    Output, // .

    /// The IR of "[".
    /// The associated data is the position of the corresponding "]" command for this command.
    Jz(usize), // [

    /// The IR of "]".
    /// The associated data is the position of the corresponding "[" command for this command.
    Jnz(usize), // ]
}
