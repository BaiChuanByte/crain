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

pub mod bfic;
pub mod bfsetting;
pub mod celltype;
pub mod error;
pub mod interp;

pub use bfic::BfCode;
pub use bfsetting::{BfSetting, Endian, EofValue};
pub use celltype::*;
pub use error::*;
pub use interp::{BfFrame, BfVm};
pub use interp::{eval_file, eval_string};
