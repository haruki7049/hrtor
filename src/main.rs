use clap::Parser;
use linefeed::Interface;
use linefeed::ReadResult;
use linefeed::Terminal;
use std::error::Error;
use std::fs::read_to_string;
use std::io::BufReader;
use std::io::Read;
use std::fs::File;
use std::path::Path;

/// Commands enumeration in interpreter
#[derive(Debug, PartialEq)]
enum Commands {
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

/// PROMPT message in interpreter
const PROMPT: &str = "hrtor:> ";

/// CommandLine Argument
#[derive(Parser)]
struct AppArg {

    /// File's Path
    path: String,
}

/// read context from the file which is selected by CommandLine Argument
/// TODO: this function might be replaced by std::fs::File
fn read_context() -> Result<String, Box<dyn Error>> {
    let app = AppArg::parse();
    let context: String = read_to_string(app.path.as_str())?;
    Ok(context)
}

/// read filepath from the first argument
fn read_filepath() -> Result<String, Box<dyn Error>> {
    let app = AppArg::parse();
    let filepath: String = app.path;
    Ok(filepath)
}


/// main function
fn main() -> Result<(), Box<dyn Error>> {
    // record filepath through a CommandLine Argument
    let filepath: String = read_filepath()?;
    // file_context is used as buffer
    let mut file_context: String = match std::fs::read_to_string(&filepath) {
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

    println!("{}", &file_context);

    // create interpreter by linefeed
    let reader = Interface::new(PROMPT).unwrap();
    reader.set_prompt(format!("{}", PROMPT).as_ref()).unwrap();

    println!("filepath: {}", filepath);

    // mainloop by linefeed
    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        let input = input.parse::<Commands>().unwrap();

        match input {
            Commands::Exit => {
                break;
            }
            Commands::Print => {
                println!("{}", file_context);
            }
            Commands::Write => {
                if let Err(err) = std::fs::write(&filepath, &file_context) {
                    eprintln!("Error saving file: {}", err);
                } else {
                    println!("file saved successfully");
                }
            }
            Commands::Add => {
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
                file_context = inputed_text;
            }
            Commands::DeleteAll => {
                file_context = String::new();
                println!("Deleted all in buffer's context");
            }
        }
    }

    // Good bye message
    println!("Bye!!");
    Ok(())
}

#[cfg(test)]
mod test {
    //! tests for this main.rs
}
