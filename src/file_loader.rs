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

/// Get file's path and file's context from a CommandLine Argument
pub fn get_file_info() -> Result<(String, String), Box<dyn Error>> {
    // record filepath through a CommandLine Argument
    let filepath: String = {
        let app = AppArg::parse();
        let filepath: String = app.path;
        filepath
    };

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

/// Get config's path and config's context from the config CommandLine Option
pub fn get_config_info() -> Result<(String, String), Box<dyn Error>> {
    // let configpath: String = read_configpath()?;
    let configpath: String = {
        let app = AppArg::parse();
        app.config.unwrap_or_else(|| {
            println!("failed to load config file");
            String::from("")
        })
    };
    let config_context: String =
        std::fs::read_to_string(&configpath).unwrap_or_else(|_| String::new());
    Ok((configpath, config_context))
}
