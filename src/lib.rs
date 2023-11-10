use clap::Parser;
use std::error::Error;

/// PROMPT message in interpreter
pub const PROMPT: &str = "hrtor:> ";

/// CommandLine Argument
#[derive(Parser)]
#[command(author, version, about)]
pub struct AppArg {

    /// File's Path
    #[arg(help = "The file you want to edit")]
    pub path: String,

    //#[arg(long, default_value_t = String::from("./init.lua"))]
    #[arg(short, long, help = "your config file which is as config.lua")]
    pub config: Option<String>,
}

/// read filepath from CommandLine's first argument
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
        Err(_) => {
            println!("your file cannot find. create a new buffer to continue this process.");
            String::new()
        }
    };
    Ok((filepath, file_context))
}

/// read filepath from CommandLine's config argument
fn read_configpath() -> Result<String, Box<dyn Error>> {
    let app = AppArg::parse();
    let configpath: String = match app.config {
        Some(path) => path,
        None => {
            println!("failed to load config file");
            String::from("")
        }
    };
    Ok(configpath)
}

/// Get config's path and config's context from the config CommandLine Option
pub fn get_config_info() -> Result<(String, String), Box<dyn Error>> {
    let configpath: String = read_configpath()?;
    let config_context: String = match std::fs::read_to_string(&configpath) {
        Ok(context) => {
            context
        }
        Err(_) => {
            String::new()
        }
    };
    Ok((configpath, config_context))
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
#[cfg(windows)]
pub fn push_context() -> String {
    let mut inputed_text: String = String::new();
    loop {
        let mut last_line: String = String::new();

        std::io::stdin()
            .read_line(&mut last_line)
            .expect("failed to read line");

        if last_line.as_str() == ".\r\n" {
            break;
        }
        inputed_text.push_str(&last_line);
    }
    inputed_text
}

/// get some context from standard input, and return String
#[cfg(unix)]
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
