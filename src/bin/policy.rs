use aws_iam::io;
use aws_iam::model::Policy;
use aws_iam::report;
use aws_iam::report::MarkdownGenerator;
use std::error::Error;
use std::fmt;
use std::io::{stdin, stdout};
use std::path::PathBuf;
use std::str::FromStr;
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
        /// The input file to validate, stdin if not present
        #[structopt(parse(from_os_str))]
        file_name: Option<PathBuf>,
        /// Output format for successful results (latex, markdown, rust)
        #[structopt(long, short)]
        format: Option<Format>,
    },
}

#[derive(Debug)]
enum Format {
    Rust,
    Markdown,
    Latex,
}

#[derive(Debug)]
enum FormatError {
    MissingFormat,
    InvalidFormat,
}

impl ToString for Format {
    fn to_string(&self) -> String {
        match self {
            Format::Rust => "rust".to_string(),
            Format::Markdown => "markdown".to_string(),
            Format::Latex => "latex".to_string(),
        }
    }
}

impl FromStr for Format {
    type Err = FormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(FormatError::MissingFormat)
        } else if s == "rust" {
            Ok(Format::Rust)
        } else if s == "markdown" {
            Ok(Format::Markdown)
        } else if s == "latex" {
            Ok(Format::Latex)
        } else {
            Err(FormatError::InvalidFormat)
        }
    }
}

impl ToString for FormatError {
    fn to_string(&self) -> String {
        match self {
            FormatError::MissingFormat => "No format was provided".to_string(),
            FormatError::InvalidFormat => "Input not a valid format".to_string(),
        }
    }
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
        Command::Verify { file_name, format } => verify_file(file_name, format),
    }
}

fn create_new_file(file_name: Option<PathBuf>) -> Result<(), ToolError> {
    Ok(())
}

fn verify_file(file_name: Option<PathBuf>, format: Option<Format>) -> Result<(), ToolError> {
    println!("verify_file(file_name: {:?})", file_name);
    match file_name {
        Some(file_name) => {
            if file_name.exists() && file_name.is_file() {
                println!("|- verify_file reading file");
                verify_file_result(io::read_from_file(&file_name), format)
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
            verify_file_result(io::read_from_reader(stdin()), format)
        }
    }
}

fn verify_file_result(
    result: Result<Policy, io::Error>,
    format: Option<Format>,
) -> Result<(), ToolError> {
    match result {
        Ok(policy) => {
            match format {
                Some(format) => {
                    println!("|- verify_file parsed successfully");
                    println!("'- result in {:?} format:", format);
                    match format {
                        Format::Rust => println!("{:#?}", policy),
                        Format::Markdown => {
                            let generator = MarkdownGenerator::default();
                            report::walk_policy(&policy, &generator, &mut stdout());
                        }
                        _ => println!("darn"),
                    }
                }
                None => println!("'- verify_file parsed successfully"),
            }
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
