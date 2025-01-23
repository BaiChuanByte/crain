use crate::celltype::BFCell;


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BFCode<T: BFCell> {
    AddCell(T),         // +
    SubCell(T),         // -
    LeftShift(usize),   // <
    RightShift(usize),  // >
    Input,              // ,
    Output,             // .
    Jz(usize),          // [
    Jnz(usize),         // ]
}

