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
pub mod celltype;
pub mod interp;

mod error;

pub use bfic::BfCode;
pub use celltype::*;
pub use interp::{BfFrame, BfVm};
pub use interp::{eval_file, eval_string};

pub use error::*;

use clap::*;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run the brainfuck code.
    Run {
        #[clap(flatten)]
        mode: RunMode,

        /// The brainfuck file name or string.
        #[clap(required(true))]
        file_or_string: String,

        /// The size of array.
        #[clap(short, long, default_value_t = 30000)]
        size: usize,

        /// The position of ptr (start from 0).
        #[clap(short, long, default_value_t = 0)]
        ptr: usize,
    },
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
struct RunMode {
    /// Run the brainfuck file. (default)
    #[clap(long)]
    file: bool,

    /// Run the brainfuck string.
    #[clap(long)]
    string: bool,
}

/// The main func of Crain in effect.
///
/// The main func. Most of the logic is command-line argument processing.
pub fn main_func() -> ExitCode {
    let command = Args::parse().command;
    let err = match command {
        Commands::Run {
            mode:
                RunMode {
                    file: _, // true or false (because file mode is default)
                    string: false,
                },
            file_or_string,
            size,
            ptr,
        } => interp::eval_file(file_or_string, size, ptr),

        Commands::Run {
            mode: RunMode {
                file: false,
                string: true,
            },
            file_or_string,
            size,
            ptr,
        } => interp::eval_string(file_or_string, size, ptr),

        Commands::Run { .. } => unreachable!(),
    };

    if let Err(e) = err {
        eprintln!("{e}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
