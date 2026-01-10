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

mod bfic;
mod bfsetting;
mod celltype;
mod error;
mod interp;

use bfsetting::{BfSetting, Endian, EofValue};
use celltype::*;
use error::*;
use interp::{eval_file, eval_string};

use clap::*;
use std::path::PathBuf;
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

        /// The setting about the interpreter.
        #[clap(flatten)]
        setting: BfSettingArgs,
    },
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct RunMode {
    /// Run the brainfuck file. (default)
    file: Option<PathBuf>,

    /// Run the brainfuck string.
    #[clap(long)]
    string: Option<String>,
}

#[derive(Args, Debug)]
struct BfSettingArgs {
    /// The size of the memory array.
    #[clap(long, default_value_t = 30000)]
    size: usize,

    /// The initial pointer position.
    #[clap(long, default_value_t = 0)]
    ptr: usize,

    /// The value to if translate "\r\n" to "\n" (0x0A).
    #[clap(long, default_value_t = true)]
    translate_newline: bool,

    /// The value to set what EOF is converted to.
    #[clap(long, default_value_t = EofValue::Zero, value_enum)]
    eof_value: EofValue,

    /// The value to set which endian interprets the cell as character(s).
    #[clap(long, default_value_t = Endian::Little, value_enum)]
    endian: Endian,
}

impl From<BfSettingArgs> for BfSetting {
    fn from(val: BfSettingArgs) -> Self {
        let BfSettingArgs {
            size,
            ptr,
            translate_newline,
            eof_value,
            endian,
        } = val;
        BfSetting {
            size,
            ptr,
            translate_newline,
            eof_value,
            endian,
        }
    }
}

fn main() -> ExitCode {
    let command = Args::parse().command;
    let err = match command {
        Commands::Run { mode, setting } => {
            if let RunMode {
                file: Some(file), ..
            } = mode
            {
                eval_file(&file, setting.into())
            } else if let RunMode {
                string: Some(string),
                ..
            } = mode
            {
                eval_string(&string, setting.into())
            } else {
                unreachable!()
            }
        }
    };

    if let Err(e) = err {
        eprintln!("{e}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Args::command().debug_assert();
}
