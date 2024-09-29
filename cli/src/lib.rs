pub mod by_clap;

use file_loader::{CommandLineArgsParser, FileInfo};
use std::error::Error;

pub trait CLI {
    fn eval(&self) -> Result<CLIArgs, Box<dyn Error>>;
}

pub struct CLIArgs {
    pub text_file: String,
    pub config: String,
}

impl CommandLineArgsParser for CLIArgs {
    fn read_fileinfo(&self) -> Result<FileInfo, Box<dyn Error>> {
        Ok(FileInfo {
            path: self.text_file.clone(),
            context: std::fs::read_to_string(self.text_file.clone()).unwrap_or_else(|_| {
                eprintln!("your file cannot find. create a new buffer to continue this process.");
                String::new()
            }),
        })
    }

    fn read_configinfo(&self) -> Result<FileInfo, Box<dyn Error>> {
        Ok(FileInfo {
            path: self.config.clone(),
            context: std::fs::read_to_string(self.config.clone()).unwrap_or_else(|_| {
                eprintln!(
                    "your config file cannot find. Continue this process without config file."
                );
                String::new()
            }),
        })
    }
}
