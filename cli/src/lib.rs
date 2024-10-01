pub mod by_clap;

use file_loader::{CommandLineArgsParser, FileInfo};
use std::error::Error;

pub trait CLI {
    fn eval(&self) -> Result<CLIArgs, Box<dyn Error>>;
}

pub struct CLIArgs {
    pub text_file: String,
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
}

#[cfg(test)]
mod tests {
    use crate::CLIArgs;
    use file_loader::{ FileInfo, CommandLineArgsParser };

    #[test]
    fn how_to_read_fileinfo() {
        let args: CLIArgs = CLIArgs {
            text_file: String::from("test.txt"),
        };

        let fileinfo: FileInfo = args.read_fileinfo().unwrap();

        assert_eq!(String::new(), fileinfo.context);
    }
}
