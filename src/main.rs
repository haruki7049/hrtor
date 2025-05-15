use clap::Parser;
use hrtor::cli::CLIArgs;
use hrtor::processor::constants::CommandStatus;
use hrtor::processor::{FileInfo, Hrtor, Processor};
use linefeed::Interface;

/// PROMPT message in interpreter
pub const PROMPT: &str = "hrtor:> ";

/// main function
fn main() -> anyhow::Result<()> {
    // Gets CLIArgs by Hrtor's Command-Line Interface
    let args: CLIArgs = CLIArgs::parse();

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
