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
use std::path::PathBuf;
use std::process::ExitCode;

/// The main func of Crain in effect.
///
/// The main func. Most of the logic is command-line argument processing.
pub fn main_func() -> ExitCode {
    let cmd = cmd_factory();
    let matches = cmd.try_get_matches();

    let when_err = |err: clap::Error| {
        let render = err.render();
        eprintln!("{render}");
        <i32 as TryInto<u8>>::try_into(err.exit_code())
            .unwrap()
            .into()
    };

    if let Err(err) = matches {
        when_err(err)
    } else if let Ok(matches) = matches {
        if let Some(subcmd) = matches.subcommand() {
            match subcmd {
                ("run", sub_matches) => {
                    let file = sub_matches.get_one::<PathBuf>("file");
                    let string = sub_matches.get_one::<String>("string");

                    let size = *sub_matches.get_one::<u64>("size").unwrap() as usize;
                    let ptr = *sub_matches.get_one::<u64>("ptr").unwrap() as usize;
                    let translate_newline = *sub_matches.get_one::<bool>("translate_newline").unwrap();
                    let eof_value = sub_matches.get_one::<String>("eof_value").unwrap();
                    let endian = sub_matches.get_one::<String>("endian").unwrap();

                    // check if the arguments are valid
                    if ptr >= size {
                        return when_err(cmd_factory()
                            .error(
                                clap::error::ErrorKind::ValueValidation,
                                "Pointer position must be less than the size of the memory array.",
                            )
                        );
                    }

                    let eof_value = match eof_value.as_str() {
                        "zero" => bfsetting::EofValue::Zero,
                        "negative_one" => bfsetting::EofValue::NegativeOne,
                        "no_write" => bfsetting::EofValue::NoWrite,
                        _ => unreachable!(),
                    };

                    let endian = match endian.as_str() {
                        "little" => bfsetting::Endian::Little,
                        "big" => bfsetting::Endian::Big,
                        "native" => bfsetting::Endian::Native,
                        _ => unreachable!(),
                    };

                    let setting = BfSetting {
                        size,
                        ptr,
                        translate_newline,
                        eof_value,
                        endian,
                    };

                    if let Some(file) = file {
                        eval_file(file, setting)
                            .map_err(|e| eprintln!("Error: {e}"))
                            .map_or(ExitCode::SUCCESS, |_| ExitCode::SUCCESS)
                    } else if let Some(string) = string {
                        eval_string(string, setting)
                            .map_err(|e| eprintln!("Error: {e}"))
                            .map_or(ExitCode::SUCCESS, |_| ExitCode::SUCCESS)
                    } else {
                        unreachable!()
                    }
                }
                _ => unreachable!(),
            }
        } else {
            cmd_factory().print_long_help().unwrap();
            ExitCode::FAILURE
        }
    } else {
        when_err(matches.unwrap_err())
    }
}

fn cmd_factory() -> clap::Command {
    command!().subcommand(
        Command::new("run")
            .about("Run the brainfuck code.")
            .arg(
                Arg::new("file")
                    .help("Run the brainfuck code in a file. (default)")
                    .value_name("FILE")
                    .value_parser(value_parser!(PathBuf))
            )
            .arg(
                Arg::new("string")
                    .action(clap::ArgAction::Set)
                    .help("Run the brainfuck code in a string.")
                    .long("string")
                    .value_name("CODE"),
            )
            .group(
                ArgGroup::new("run_modes")
                    .args(["file", "string"])
                    .multiple(false)
                    .required(true),
            )
            .arg(
                Arg::new("size")
                    .default_value("30000")
                    .help("The size of the memory array.")
                    .long("size")
                    .short('s')
                    .value_name("SIZE")
                    .value_parser(
                        // `builder::ValueParserFactory` is not implemented for `usize`
                        // So we use `u64`, it is always greater than or equal to `usize`
                        value_parser!(u64).range(1..),
                    ),
            )
            .arg(
                Arg::new("ptr")
                    .default_value("0")
                    .help("The initial pointer position.")
                    .long("ptr")
                    .short('p')
                    .value_name("POSITION")
                    .value_parser(
                        // `builder::ValueParserFactory` is not implemented for `usize`
                        // So we use `u64`, it is always greater than or equal to `usize`
                        value_parser!(u64),
                    ),
            )
            .arg(
                Arg::new("translate_newline")
                    .default_value(if cfg!(target_family = "windows") {"true"} else {"false"})
                    .help("Translate \"\\r\\n\" to \"\\n\" (0x0A).")
                    .long("translate-newline")
                    .short('t')
                    .value_name("BOOL")
                    .value_parser(value_parser!(bool)),
            )
            .arg(
                Arg::new("eof_value")
                    .default_value("zero")
                    .help("Value to set when EOF is encountered.")
                    .long("eof-value")
                    .short('e')
                    .value_name("VALUE")
                    .value_parser(builder::PossibleValuesParser::new([
                        "zero",
                        "negative_one",
                        "no_write",
                    ])),
            )
            .arg(
                Arg::new("endian")
                    .default_value("little")
                    .help("Endianness to interpret the cell as character(s).")
                    .long("endian")
                    .short('n')
                    .value_name("ENDIANNESS")
                    .value_parser(builder::PossibleValuesParser::new([
                        "little", "big", "native",
                    ])),
            ),
    )
}

#[cfg(test)]
#[test]
fn verify_cmd() {
    cmd_factory().debug_assert();
}
