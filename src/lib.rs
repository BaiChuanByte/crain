mod bfir;
mod celltype;
mod error;
mod interp;

use clap::*;

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
pub struct RunMode {
    /// Run the brainfuck file. (default)
    #[clap(long)]
    file: bool,

    /// Run the brainfuck string.
    #[clap(long)]
    string: bool,
}

pub fn main_func() {
    let command = Args::parse().command;
    let err;

    match command {
        Commands::Run {
            mode:
                RunMode {
                    file: _, // true or false (because file mode is default)
                    string: false,
                },
            file_or_string,
            size,
            ptr,
        } => {
            err = interp::run_bf_file(file_or_string, size, ptr);
        }

        Commands::Run {
            mode: RunMode {
                file: false,
                string: true,
            },
            file_or_string,
            size,
            ptr,
        } => {
            err = interp::run_bf_string(file_or_string, size, ptr);
        }

        Commands::Run { .. } => unreachable!(),
    }

    if let Err(e) = err {
        println!("{e}");
        std::process::exit(1);
    }
}
