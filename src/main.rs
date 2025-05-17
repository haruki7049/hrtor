use clap::Parser;
use hrtor::cli::{CLIArgs, display_shellcompletion};
use hrtor::processor::constants::CommandStatus;
use hrtor::processor::{FileInfo, Hrtor, Processor, ReadResult, Signal};
use linefeed::Interface;

/// PROMPT message in interpreter
pub const PROMPT: &str = "hrtor:> ";

/// main function
fn main() -> anyhow::Result<()> {
    // Gets CLIArgs by Hrtor's Command-Line Interface
    let args: CLIArgs = CLIArgs::parse();

    // Generates shell completion if the completion option is selected by Hrtor CLI interface
    if args.completion.is_some() {
        display_shellcompletion(args.completion.unwrap());
        return Ok(());
    }

    // Gets FileInfo from CLIArgs
    let file: FileInfo = args.read_fileinfo()?;

    // create interpreter by linefeed
    let reader = Interface::new(PROMPT)?;
    reader.set_prompt(PROMPT.to_string().as_ref())?;

    // Create Hrtor instance
    let mut instance = Hrtor::from(file);

    // mainloop by linefeed
    loop {
        let read: ReadResult = convert_linefeed(&reader)?;
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

fn convert_linefeed<Term: linefeed::Terminal>(
    interface: &Interface<Term>,
) -> anyhow::Result<ReadResult> {
    return match interface.read_line()? {
        linefeed::ReadResult::Eof => Ok(ReadResult::Eof),
        linefeed::ReadResult::Input(string) => Ok(ReadResult::Input(string)),
        linefeed::ReadResult::Signal(signal) => match signal {
            linefeed::Signal::Break => Ok(ReadResult::Signal(Signal::Break)),
            linefeed::Signal::Continue => Ok(ReadResult::Signal(Signal::Continue)),
            linefeed::Signal::Interrupt => Ok(ReadResult::Signal(Signal::Interrupt)),
            linefeed::Signal::Resize => Ok(ReadResult::Signal(Signal::Resize)),
            linefeed::Signal::Suspend => Ok(ReadResult::Signal(Signal::Suspend)),
            linefeed::Signal::Quit => Ok(ReadResult::Signal(Signal::Quit)),
        },
    };
}
