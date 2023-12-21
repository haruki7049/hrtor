use clap::Parser;
use std::error::Error;

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
pub fn get_config_info() -> Result<FileInfo, Box<dyn Error>> {
    let app = AppArg::parse();
    Ok(FileInfo {
        path: app.config.clone().unwrap_or_else(|| {
            println!("failed to load config file");
            String::from("")
        }),
        context: std::fs::read_to_string(app.config.clone().unwrap_or_default())
            .unwrap_or_else(|_| String::new()),
    })
}
