use std::error::Error;
use file_loader::{CommandLineArgsParser, FileInfo};
use clap::Parser;

pub trait CLI {
    fn eval(&self) -> Result<CLIArgs, Box<dyn Error>>;
}

pub struct CLIArgs {
    pub text_file: String,
    pub config: String,
}

#[derive(Parser)]
pub struct AppArg {
    /// File's Path
    #[arg(help = "The file you want to edit")]
    pub path: String,

    //#[arg(long, default_value_t = String::from("./init.lua"))]
    #[arg(short, long, default_value_t = String::from("./init.lua"), help = "your config file which is as config.lua")]
    pub config: String,
}

impl CommandLineArgsParser for AppArg {
    fn read_fileinfo(&self) -> Result<FileInfo, Box<dyn Error>> {
        Ok(FileInfo {
            path: self.path.clone(),
            context: std::fs::read_to_string(self.path.clone()).unwrap_or_else(|_| {
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
