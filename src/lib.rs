//! Crain -- A brainfuck compiler/interpreter/tool.
//!
//!
//! ```brainfuck
//!  ++++  +++++    +[   - >    +
//! +    + +    +  +  >  - ->   +
//! +      +    < <    < ] > +  +
//! +      ++++.  [->++> + <  < ]
//! >    . >   .  <    - - -   --
//!  ----  .    + +    + + +    .
//! ```
//!
//! Crain is a brainfuck compiler/interpreter/development tool written in Rust,
//! dedicated to providing a fast, reliable, and ready-to-use execution environment
//! for brainfuck programs.

pub mod bfsetting;
pub mod cell;
pub mod error;
pub mod interp;
pub mod ir;

pub use bfsetting::{BfSetting, Endian, EofValue};
pub use cell::*;
pub use error::*;
pub use interp::{BfFrame, BfVm};
pub use interp::{eval_file, eval_string};
pub use ir::BfIr;
