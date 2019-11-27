/*!
Command-line tool to read and verify policy files and create new from templates.
*/
#[macro_use]
extern crate tracing;

use aws_iam::io;
use aws_iam::model::Policy;
use aws_iam::report;
use aws_iam::report::{LatexGenerator, MarkdownGenerator};
use std::error::Error;
use std::fmt;
use std::fs::OpenOptions;
use std::io::{stdin, Write};
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

// ------------------------------------------------------------------------------------------------
// Command-Line Parsing
// ------------------------------------------------------------------------------------------------

#[derive(Debug, StructOpt)]
#[structopt(name = "policy")]
struct Cli {
    /// The level of logging to perform, from off to trace
    #[structopt(long, short = "v", parse(from_occurrences))]
    verbose: i8,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Create a new default policy document
    New {
        /// Name of a template, use 'list' to see supported templates
        #[structopt(long, short)]
        template: String,
        /// Force overwrite of existing file
        #[structopt(long, short)]
        force: bool,
        /// Output file, stdout if not present
        #[structopt(name = "FILE", parse(from_os_str))]
        file_name: Option<PathBuf>,
    },
    /// Verify an existing policy document
    Verify {
        /// Output format for successful results (latex, markdown, rust)
        #[structopt(long, short)]
        format: Option<Format>,
        /// The input file to validate, stdin if not present
        #[structopt(parse(from_os_str))]
        file_name: Option<PathBuf>,
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

// ------------------------------------------------------------------------------------------------
// Main Function
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
enum ToolError {
    CannotOpenForRead(String),
    CannotOpenForWrite(String),
    InvalidTemplateName(String),
    WriteToFile,
    VerifyFailed,
}

fn main() -> Result<(), ToolError> {
    let args = Cli::from_args();

    init_tracing(args.verbose);

    match args.cmd {
        Command::New {
            file_name,
            force,
            template,
        } => {
            if template == "list" {
                list_templates()
            } else {
                create_new_file(file_name, &template, force)
            }
        }
        Command::Verify { file_name, format } => verify_file(file_name, format),
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn init_tracing(verbosity: i8) {
    let log_level = match verbosity {
        0 => LevelFilter::OFF,
        1 => LevelFilter::ERROR,
        2 => LevelFilter::WARN,
        3 => LevelFilter::INFO,
        4 => LevelFilter::DEBUG,
        _ => LevelFilter::TRACE,
    };

    let filter = EnvFilter::from_default_env()
        .add_directive(
            format!("{}={}", module_path!(), log_level)
                .parse()
                .expect("Issue with command-line trace directive"),
        )
        .add_directive(
            format!("aws_iam={}", log_level)
                .parse()
                .expect("Issue with library trace directive"),
        );
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Unable to set global default tracing subscriber");
    info!("Log level set to `LevelFilter::{:?}`", log_level);
}

fn list_templates() -> Result<(), ToolError> {
    let span = debug_span!("list_templates");
    let _enter = span.enter();
    println!("templates: {:?}", templates::all_templates().keys());
    Ok(())
}

fn create_new_file(
    file_name: Option<PathBuf>,
    template: &String,
    force_write: bool,
) -> Result<(), ToolError> {
    let span = debug_span!("create_new_file", ?file_name, ?template, ?force_write);
    let _enter = span.enter();
    if !templates::all_templates().contains_key(template) {
        error!("'{}' is not a valid template name", template);
        return Err(ToolError::InvalidTemplateName(template.clone()));
    }
    match file_name {
        Some(file_name) => {
            if file_name.exists() && file_name.is_file() && !force_write {
                error!("could not open file for write, not a file, or missing -f");
                Err(ToolError::CannotOpenForWrite(
                    file_name
                        .to_str()
                        .unwrap_or("{error in file name}")
                        .to_string(),
                ))
            } else {
                debug!("opening output file");
                match OpenOptions::new()
                    .write(true)
                    .create_new(!force_write)
                    .create(true)
                    .truncate(true)
                    .open(file_name.clone())
                {
                    Ok(mut f) => {
                        match write!(f, "{}", templates::all_templates().get(template).unwrap()) {
                            Ok(()) => Ok(()),
                            Err(e) => {
                                error!("write error: {:?}", e);
                                Err(ToolError::WriteToFile)
                            }
                        }
                    }
                    Err(e) => {
                        error!("could not open file for write, error {:?}", e);
                        Err(ToolError::CannotOpenForWrite(
                            file_name
                                .to_str()
                                .unwrap_or("{error in file name}")
                                .to_string(),
                        ))
                    }
                }
            }
        }
        None => {
            debug!("writing to stdout");
            println!("{}", templates::all_templates().get(template).unwrap());
            Ok(())
        }
    }
}

fn verify_file(file_name: Option<PathBuf>, format: Option<Format>) -> Result<(), ToolError> {
    let span = debug_span!("verify_file", ?file_name, ?format);
    let _enter = span.enter();
    match file_name {
        Some(file_name) => {
            if file_name.exists() && file_name.is_file() {
                debug!("reading file");
                verify_file_result(io::read_from_file(&file_name), format)
            } else {
                error!("could not read from file");
                Err(ToolError::CannotOpenForRead(
                    file_name
                        .to_str()
                        .unwrap_or("{error in file name}")
                        .to_string(),
                ))
            }
        }
        None => {
            debug!("reading from stdin");
            verify_file_result(io::read_from_reader(stdin()), format)
        }
    }
}

fn verify_file_result(
    result: Result<Policy, io::Error>,
    format: Option<Format>,
) -> Result<(), ToolError> {
    let span = debug_span!("verify_file_result", ?result, ?format);
    let _enter = span.enter();
    match result {
        Ok(policy) => {
            match format {
                Some(format) => {
                    debug!("file parsed successfully");
                    match format {
                        Format::Rust => println!("{:#?}", policy),
                        Format::Markdown => {
                            let mut generator = MarkdownGenerator::default();
                            report::walk_policy(&policy, &mut generator);
                        }
                        Format::Latex => {
                            let mut generator = LatexGenerator::default();
                            report::walk_policy(&policy, &mut generator);
                        }
                    }
                }
                None => debug!("parsed successfully"),
            }
            Ok(())
        }
        Err(e) => {
            match e {
                io::Error::DeserializingJson(s) => {
                    error!("failed to parse, error: {:?}", s);
                }
                io::Error::ReadingFile(e) => {
                    error!(
                        "failed to read, error: {:?}, cause: {}",
                        e,
                        match e.source() {
                            Some(source) => source.to_string(),
                            None => "unknown".to_string(),
                        }
                    );
                }
                err => {
                    error!("failed with an unexpected error: {:?}", err);
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
            ToolError::InvalidTemplateName(name) => {
                write!(f, "No template named '{}' supported", name)
            }
            ToolError::WriteToFile => write!(f, "Write operation to file failed"),
            ToolError::VerifyFailed => write!(f, "Verification of policy failed"),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod templates;
