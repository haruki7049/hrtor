use crate::processor::FileInfo;
use anyhow::Context as _;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Generator, Shell, generate};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
/// CLI Arguments, parsed by clap crate.
pub struct CLIArgs {
    /// File's Path
    #[arg(help = "The file you want to edit")]
    pub path: Option<PathBuf>,

    #[clap(subcommand)]
    action: Option<Action>,
}

#[derive(Debug, Subcommand)]
enum Action {
    Completion { shell: Shell },
}

impl CLIArgs {
    /// Creates file data typed FileInfo, from CLIArgs
    pub fn read_fileinfo(&self) -> anyhow::Result<FileInfo> {
        let path: PathBuf = self
            .path
            .clone()
            .context("No file found at the path you entered")?;

        Ok(FileInfo {
            path: path.clone(),
            context: std::fs::read_to_string(path.clone()).unwrap_or_else(|_| {
                eprintln!("your file cannot find. create a new buffer to continue this process.");
                String::new()
            }),
        })
    }
}

fn display_shellcompletion<G: Generator>(generator: G) {
    generate(
        generator,
        &mut CLIArgs::command(),
        env!("CARGO_PKG_NAME"),
        &mut std::io::stdout(),
    );
}

#[cfg(test)]
mod tests {
    use super::CLIArgs;
    use super::FileInfo;
    use std::path::PathBuf;

    #[test]
    /// How to read FileInfo from CLIArgs struct.
    fn how_to_read_fileinfo() -> anyhow::Result<()> {
        let args: CLIArgs = CLIArgs {
            path: Some(PathBuf::from("test.txt")),
            action: None,
        };

        let fileinfo: FileInfo = args.read_fileinfo()?;

        assert_eq!(String::new(), fileinfo.context);

        Ok(())
    }
}
