use clap::Parser;
use hrtor::cli::{CLIArgs, FileInfo};
use hrtor::processor::constants::{CommandStatus, CommandResult};
use hrtor::processor::{Hrtor, HrtorProcessor, Processor};
use linefeed::Interface;
use std::sync::{Arc, Mutex};

/// PROMPT message in interpreter
pub const PROMPT: &str = "hrtor:> ";

/// main function
fn main() -> anyhow::Result<()> {
    let args: CLIArgs = CLIArgs::parse();

    let file: FileInfo = args.read_fileinfo();

    // create interpreter by linefeed
    let reader = Interface::new(PROMPT)?;
    reader.set_prompt(PROMPT.to_string().as_ref())?;

    let instance = Hrtor::new(HrtorProcessor {
        editing_file: Arc::new(Mutex::new(file)),
    });

    // mainloop by linefeed
    while let CommandStatus::Continue(result) = {
        let read = reader.read_line()?;
        instance.processor.handle_command(read)
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
