#[macro_use]
extern crate log;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "policy")]
struct Cli {
    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Create a new default policy document
    New {
        /// Output file, stdout if not present
        #[structopt(parse(from_os_str))]
        output: Option<PathBuf>,

        /// Where to write the output: to `stdout` or `file`
        #[structopt(short)]
        out_type: String,

        /// File name: only required when `out` is set to `file`
        #[structopt(name = "FILE", required_if("out_type", "file"))]
        file_name: String,
    },
    /// Verify an existing policy document
    Verify {
        /// Input file
        #[structopt(parse(from_os_str))]
        input: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("starting...");
    let args = Cli::from_args();
    println!("{:#?}", args);
    Ok(())
}
