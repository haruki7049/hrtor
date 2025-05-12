use anyhow::Context as _;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Generator, Shell, generate};
use hrtor::processor::constants::CommandStatus;
use hrtor::processor::{FileInfo, Hrtor, Processor};
use linefeed::Interface;
use std::path::PathBuf;

/// PROMPT message in interpreter
pub const PROMPT: &str = "hrtor:> ";

/// main function
fn main() -> anyhow::Result<()> {
    // Gets CLIArgs by Hrtor's Command-Line Interface
    let args: CLIArgs = CLIArgs::parse();

    if let Some(Action::Completion { shell: generator }) = args.action {
        display_shellcompletion(generator);

        // Early return
        return Ok(());
    }

    // Gets FileInfo from CLIArgs
    let file: FileInfo = args.read_fileinfo()?;

    // create interpreter by linefeed
    let reader = Interface::new(PROMPT)?;
    reader.set_prompt(PROMPT.to_string().as_ref())?;

    // Create Hrtor instance
    let instance = Hrtor::from(file);

    // mainloop by linefeed
    loop {
        let read = reader.read_line()?;
        let status: CommandStatus = instance.processor.handle_command(read).unwrap_or_else(|e| {
            // Display the error if your command has an error, then continues hrtor.
            eprintln!("{}", e);
            CommandStatus::Continue
        });

        match status {
            CommandStatus::Continue => continue,
            CommandStatus::Quit => break,
        }
    }

    println!("Bye!!");
    Ok(())
}

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
