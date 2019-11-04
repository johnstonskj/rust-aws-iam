use aws_iam::io;
use aws_iam::model::Policy;
use std::error::Error;
use std::fmt;
use std::io::stdin;
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
        #[structopt(name = "FILE", parse(from_os_str))]
        file_name: Option<PathBuf>,
    },
    /// Verify an existing policy document
    Verify {
        /// Input file, stdin if not present
        #[structopt(name = "FILE", parse(from_os_str))]
        file_name: Option<PathBuf>,
    },
}

#[derive(Debug)]
enum ToolError {
    CannotOpenForRead(String),
    CannotOpenForWrite(String),
    VerifyFailed,
}

fn main() -> Result<(), ToolError> {
    println!("starting...");
    match Cli::from_args().cmd {
        Command::New { file_name } => create_new_file(file_name),
        Command::Verify { file_name } => verify_file(file_name),
    }
}

fn create_new_file(file_name: Option<PathBuf>) -> Result<(), ToolError> {
    Ok(())
}

fn verify_file(file_name: Option<PathBuf>) -> Result<(), ToolError> {
    println!("verify_file(file_name: {:?})", file_name);
    match file_name {
        Some(file_name) => {
            if file_name.exists() && file_name.is_file() {
                println!("|- verify_file reading file");
                verify_file_result(io::read_from_file(&file_name))
            } else {
                println!("'- verify_file could not read from file");
                Err(ToolError::CannotOpenForRead(
                    file_name
                        .to_str()
                        .unwrap_or("{error in file name}")
                        .to_string(),
                ))
            }
        }
        None => {
            println!("|- verify_file reading from stdin");
            verify_file_result(io::read_from_reader(stdin()))
        }
    }
}

fn verify_file_result(result: Result<Policy, io::Error>) -> Result<(), ToolError> {
    match result {
        Ok(policy) => {
            println!("'- verify_file parsed successfully");
            println!("{:#?}", policy);
            Ok(())
        }
        Err(e) => {
            match e {
                io::Error::DeserializingJson(s) => {
                    println!("'- verify_file failed to parse, error: {:?}", s);
                }
                io::Error::ReadingFile(e) => {
                    println!(
                        "'- verify_file failed to read, error: {:?}, cause: {}",
                        e,
                        match e.source() {
                            Some(source) => source.to_string(),
                            None => "unknown".to_string(),
                        }
                    );
                }
                err => {
                    println!("'- verify_file failed with an unexpected error: {:?}", err);
                }
            }
            Err(ToolError::VerifyFailed)
        }
    }
}

impl fmt::Display for ToolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ToolError::CannotOpenForRead(file_name) => {
                write!(f, "Error reading from file: {}", file_name)
            }
            ToolError::CannotOpenForWrite(file_name) => {
                write!(f, "Error writing to file: {}", file_name)
            }
            ToolError::VerifyFailed => write!(f, "Verification of policy failed"),
        }
    }
}
