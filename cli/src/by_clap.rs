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
