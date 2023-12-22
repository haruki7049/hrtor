use clap::Parser;
use std::{error::Error, io::ErrorKind};

/// CommandLine Argument
#[derive(Parser)]
#[command(author, version, about)]
pub struct AppArg {
    /// File's Path
    #[arg(help = "The file you want to edit")]
    pub path: String,

    //#[arg(long, default_value_t = String::from("./init.lua"))]
    #[arg(short, long, default_value_t = String::from("./init.lua"), help = "your config file which is as config.lua")]
    pub config: String,
}

pub struct FileInfo {
    pub path: String,
    pub context: String,
}

/// Get file's path and file's context from a CommandLine Argument
pub fn get_file_info() -> Result<FileInfo, Box<dyn Error>> {
    let app = AppArg::parse();
    Ok(FileInfo {
        path: app.path.clone(),
        context: std::fs::read_to_string(&app.path).unwrap_or_else(|_| {
            println!("your file cannot find. create a new buffer to continue this process.");
            String::new()
        }),
    })
}

/// Get config's path and config's context from the config CommandLine Option
pub fn get_config_info() -> Option<FileInfo> {
    let app = AppArg::parse();
    let path = app.config;
    match std::fs::read_to_string(&path) {
        Ok(context) => {
            Some(FileInfo {
                path, context
            })
        },
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            println!("Failed to load config: {} is not found.", path);
            None
        },
        Err(e) => {
            panic!("An error occured during loading {}: {:?}", path, e);
        }
    }
}
