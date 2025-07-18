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
pub mod interp;

mod error;

pub use bfic::BfCode;
pub use bfsetting::BfSetting;
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

        /// The setting about the interpreter.
        #[clap(flatten)]
        setting: Setting,
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

#[derive(Args, Debug)]
struct Setting {
    /// The size of the memory array.
    #[clap(long, default_value = "30000")]
    size: usize,

    /// The initial pointer position.
    #[clap(long, default_value = "0")]
    ptr: usize,

    /// The value to if translate "\r\n" to "\n" (0x0A).
    #[clap(long, default_value = "true")]
    translate_newline: bool,

    /// The value to set what EOF is converted to.
    #[clap(long, default_value = "zero")]
    eof_value: String,

    /// The value to set which endian interprets the cell as character(s).
    #[clap(long, default_value = "little")]
    endian: String,
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
            setting: Setting {
                    size,
                    ptr,
                    translate_newline,
                    eof_value,
                    endian,
                },
        } => interp::eval_file(file_or_string, BfSetting {
            size,
            ptr,
            translate_newline,
            eof_value: match eof_value.as_str() {
                "zero" => bfsetting::EofValue::Zero,
                "negative_one" => bfsetting::EofValue::NegativeOne,
                "no_write" => bfsetting::EofValue::NoWrite,
                _ => panic!("Invalid EOF value: {}", eof_value),
            }
            , endian: match endian.as_str() {
                "little" => bfsetting::Endian::Little,
                "big" => bfsetting::Endian::Big,
                "native" => bfsetting::Endian::Native,
                _ => panic!("Invalid Endian value: {}", endian),
            },
        }),

        Commands::Run {
            mode: RunMode {
                file: false,
                string: true,
            },
            file_or_string,
            setting: Setting {
                size,
                ptr,
                translate_newline,
                eof_value,
                endian,
            },
        } => interp::eval_string(file_or_string, BfSetting {
            size,
            ptr,
            translate_newline,
            eof_value: match eof_value.as_str() {
                "zero" => bfsetting::EofValue::Zero,
                "negative_one" => bfsetting::EofValue::NegativeOne,
                "no_write" => bfsetting::EofValue::NoWrite,
                _ => panic!("Invalid EOF value: {}", eof_value),
            },
            endian: match endian.as_str() {
                "little" => bfsetting::Endian::Little,
                "big" => bfsetting::Endian::Big,
                "native" => bfsetting::Endian::Native,
                _ => panic!("Invalid Endian value: {}", endian),
            },
        }),

        Commands::Run { .. } => unreachable!(),
    };

    if let Err(e) = err {
        eprintln!("{e}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
