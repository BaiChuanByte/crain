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
//!
//! # Features
//!
//! - **Fast**: Leveraging Rust's high performance and Crain's optimizations,
//! Crain can quickly interpret and execute brainfuck programs.
//!
//! - **Ready to use**: Crain is designed to allow users to write brainfuck program
//! with minimal configuration and manual code modifications.
//!
//! - **Highly configurable**: Crain offers a wealth of configuration options,
//! suitable for many brainfuck codes that require special configurations,
//! and even some brainfuck variants.
//!
//! - **Cross-platform support**: Crain can be compiled and run on multiple
//! operating systems, including Linux, macOS, and Windows.
//!
//! > Tip: Some features will be available in future versions.

pub mod bfir;
pub mod celltype;
pub mod interp;

mod error;

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
        } => interp::run_bf_file(file_or_string, size, ptr),

        Commands::Run {
            mode: RunMode {
                file: false,
                string: true,
            },
            file_or_string,
            size,
            ptr,
        } => interp::run_bf_string(file_or_string, size, ptr),

        Commands::Run { .. } => unreachable!(),
    };

    if let Err(e) = err {
        eprintln!("{e}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
