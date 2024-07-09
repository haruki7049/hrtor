use clap::Parser;
use std::error::Error;
use hrtor_utils::FileInfo;

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

/// Get file's path and file's context from a CommandLine Argument
pub fn get_file_info(app: &AppArg) -> Result<FileInfo, Box<dyn Error>> {
    Ok(FileInfo {
        path: app.path.clone(),
        context: std::fs::read_to_string(&app.path).unwrap_or_else(|_| {
            println!("your file cannot find. create a new buffer to continue this process.");
            String::new()
        }),
    })
}

/// Get config's path and config's context from the config CommandLine Option
pub fn get_config_info(app: &AppArg) -> Result<FileInfo, Box<dyn Error>> {
    Ok(FileInfo {
        path: app.config.clone(),
        context: std::fs::read_to_string(&app.config).unwrap_or_else(|_| {
            println!("your config file cannot find. Continue this process without config file.");
            String::new()
        }),
    })
}

#[cfg(test)]
mod test {
    use crate::file_loader::{get_config_info, get_file_info, AppArg};

    /// Test get_file_info function, whether it can get no context and file_path if the file is not exist.
    #[test]
    fn test_get_file_info() {
        let app = AppArg {
            path: String::from("test.lua"),
            config: String::from("config.lua"),
        };
        let file_info = get_file_info(&app).unwrap();
        assert_eq!(file_info.path, "test.lua");
        assert_eq!(file_info.context, "");
    }

    /// Test get_file_info function, whether it can get no context and config_path if the config_file is not exist.
    #[test]
    fn test_get_config_info() {
        let app = AppArg {
            path: String::from("test.lua"),
            config: String::from("config.lua"),
        };
        let file_info = get_config_info(&app).unwrap();
        assert_eq!(file_info.path, "config.lua");
        assert_eq!(file_info.context, "");
    }
}
