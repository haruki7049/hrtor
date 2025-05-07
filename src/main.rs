use clap::Parser;
use hrtor::cli::{CLIArgs, FileInfo};
use hrtor::processor::constants::{CommandResult, CommandStatus};
use hrtor::processor::{Hrtor, Processor};
use linefeed::Interface;

/// PROMPT message in interpreter
pub const PROMPT: &str = "hrtor:> ";

/// main function
fn main() -> anyhow::Result<()> {
    // Gets CLIArgs by Hrtor's Command-Line Interface
    let args: CLIArgs = CLIArgs::parse();

    // Gets FileInfo from CLIArgs
    let file: FileInfo = args.read_fileinfo();

    // create interpreter by linefeed
    let reader = Interface::new(PROMPT)?;
    reader.set_prompt(PROMPT.to_string().as_ref())?;

    // Create Hrtor instance
    let instance = Hrtor::from(file);

    // mainloop by linefeed
    while let CommandStatus::Continue(result) = {
        let read = reader.read_line()?;
        match instance.processor.handle_command(read) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                CommandStatus::Continue(CommandResult::NothingToDo)
            }
        }
    } {
        match result {
            CommandResult::Ok => {}
            CommandResult::NothingToDo => {}
            CommandResult::NotFound(name) => {
                eprintln!("unknown command: {:?}", name);
            }
        }
    }

    println!("Bye!!");
    Ok(())
}
