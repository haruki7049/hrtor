use clap::Parser;
use std::error::Error;

/// PROMPT message in interpreter
pub const PROMPT: &str = "hrtor:> ";

/// CommandLine Argument
#[derive(Parser)]
pub struct AppArg {

    /// File's Path
    pub path: String,
}

/// Commands enumeration in interpreter
#[derive(Debug, PartialEq)]
pub enum Commands {
    /// A command in interpreter, add.
    Add,

    /// command in interpreter, delete_all.
    DeleteAll,

    /// command in interpreter, print.
    Print,

    /// command in interpreter, write.
    Write,

    /// command in interpreter, exit.
    Exit,
}

/// Display implementation for Commands
impl std::fmt::Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "ADD"),
            Self::DeleteAll => write!(f, "DELETE_All"),
            Self::Print => write!(f, "PRINT"),
            Self::Write => write!(f, "WRITE"),
            Self::Exit => write!(f, "EXIT"),
        }
    }
}

/// FromStr implementation for Commands
impl std::str::FromStr for Commands {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ADD" => Ok(Commands::Add),
            "DELETE_ALL" => Ok(Commands::DeleteAll),
            "PRINT" => Ok(Commands::Print),
            "WRITE" => Ok(Commands::Write),
            "EXIT" => Ok(Commands::Exit),
            _ => Err("Undefined Commands for hrtor's interpreter."),
        }
    }
}

/// read filepath from the first argument
fn read_filepath() -> Result<String, Box<dyn Error>> {
    let app = AppArg::parse();
    let filepath: String = app.path;
    Ok(filepath)
}

/// Get file's path and file's context from a CommandLine Argument
pub fn get_file_info() -> Result<(String, String), Box<dyn Error>> {
    // record filepath through a CommandLine Argument
    let filepath: String = read_filepath()?;
    // file_context is used as buffer
    let file_context: String = match std::fs::read_to_string(&filepath) {
        Ok(context) => {
            println!("{}", &filepath);
            context
        }
        Err(err) => {
            println!("{}", err);
            println!("create a new buffer to continue this process.");
            String::new()
        }
    };
    Ok((filepath, file_context))
}

/// save file
pub fn save_file(filepath: &String, file_context: &String) {
    if let Err(err) = std::fs::write(filepath, file_context) {
        eprintln!("Error saving file: {}", err);
    } else {
        println!("file saved successfully");
    }
}

/// get some context from standard input, and return String
pub fn push_context() -> String {
    let mut inputed_text: String = String::new();
    loop {
        let mut last_line: String = String::new();

        std::io::stdin()
            .read_line(&mut last_line)
            .expect("failed to read line");

        if last_line.as_str() == ".\n" {
            break;
        }
        inputed_text.push_str(&last_line);
    }
    inputed_text
}
