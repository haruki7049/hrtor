use crate::{CLIArgs, CLI};
use clap::Parser;
use std::error::Error;

#[derive(Parser)]
pub struct AppArg {
    /// File's Path
    #[arg(help = "The file you want to edit")]
    pub path: String,
}

impl CLI for AppArg {
    fn eval(&self) -> Result<CLIArgs, Box<dyn Error>> {
        Ok(CLIArgs {
            text_file: self.path.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::by_clap::AppArg;
    use crate::{CLIArgs, CLI};
    use file_loader::{CommandLineArgsParser, FileInfo};

    #[test]
    fn how_to_use_eval() {
        let clap_cli: AppArg = AppArg {
            path: String::from("test.txt"),
        };

        let args: CLIArgs = clap_cli.eval().unwrap();

        assert_eq!(
            FileInfo {
                context: String::new(),
                path: String::from("test.txt")
            },
            args.read_fileinfo().unwrap()
        );
        assert_eq!(String::from("test.txt"), args.text_file);
    }
}
