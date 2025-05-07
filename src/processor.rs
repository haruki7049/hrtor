//! # Hrtor processor module
//!
//! This crate has some structs
//! - Hrtor
//! - HrtorProcessor
//!
//! And some traits
//! - Processor
//!
//! ## Structs
//!
//! ### Hrtor
//! #### Variables
//! - processor: `Arc<HrtorProcessor>`
//!
//! #### Interface
//! - new: `Fn(processor: HrtorProcessor) -> Self`
//! - from: `Fn(file: FileInfo) -> Self`
//!
//! ### HrtorProcessor
//! #### variables
//! - editing_file: `Arc<Mutex<FileInfo>>`
//!
//! #### Interface
//! - handle_command: `Fn(&self, command: ReadResult) -> CommandStatus`
//! - interpret_command_status: `Fn(&self, status: CommandStatus)`
//! - from: `Fn(file: FileInfo) -> Self`
//!
//! ## Traits
//!
//! ### Processor
//! - interpret_command_status: `Fn(&self, status: CommandStatus)`
//! - handle_command: `Fn(&self, command: ReadResult) -> CommandStatus`

pub mod actions;
pub mod constants;
pub mod parser;

use crate::cli::FileInfo;
use crate::processor::parser::{Command, Expression};
use constants::{CommandResult, CommandStatus};
use linefeed::{ReadResult, Signal};
use std::sync::{Arc, Mutex};

pub struct HrtorProcessor {
    pub editing_file: Arc<Mutex<FileInfo>>,
}

impl HrtorProcessor {
    fn from(file: FileInfo) -> Self {
        Self {
            editing_file: Arc::new(Mutex::new(file)),
        }
    }
}

pub fn command_status_ok() -> CommandStatus {
    CommandStatus::Continue(CommandResult::Ok)
}

pub trait Processor {
    /// Interpret CommandStatus without Input loop
    fn interpret_command_status(&self, status: CommandStatus);

    fn handle_command(&self, command: ReadResult) -> anyhow::Result<CommandStatus>;

    /// Evaluates the command
    fn eval(&self, str: String) -> anyhow::Result<CommandStatus>;
}

pub struct Hrtor {
    pub processor: Arc<HrtorProcessor>,
}

impl Hrtor {
    /// Creates Hrtor instance by HrtorProcessor
    pub fn new(processor: HrtorProcessor) -> Self {
        Self {
            processor: processor.into(),
        }
    }

    /// Creates Hrtor instance from the file user want to edit
    pub fn from(file: FileInfo) -> Self {
        Self {
            processor: Arc::new(HrtorProcessor::from(file)),
        }
    }
}

impl Processor for HrtorProcessor {
    fn interpret_command_status(&self, status: CommandStatus) {
        match status {
            CommandStatus::Continue(CommandResult::Ok) => (),
            CommandStatus::Continue(CommandResult::NothingToDo) => (),
            CommandStatus::Quit => {
                // Exit status zero
                println!("Bye!!");
                std::process::exit(0);
            }
        }
    }

    fn handle_command(&self, command: ReadResult) -> anyhow::Result<CommandStatus> {
        match command {
            ReadResult::Input(str) => self.eval(str),
            ReadResult::Eof
            | ReadResult::Signal(Signal::Interrupt)
            | ReadResult::Signal(Signal::Quit) => Ok(CommandStatus::Quit),
            _ => Ok(CommandStatus::Continue(CommandResult::NothingToDo)),
        }
    }

    fn eval(&self, str: String) -> anyhow::Result<CommandStatus> {
        let expr: Expression = match parser::parse(str.as_str()) {
            Ok(v) => v,
            Err(e) => anyhow::bail!(e),
        };

        return match expr.cmd {
            Command::Exit => Ok(self.exit()),
            Command::Write => Ok(self.write()),
            Command::Add => Ok(self.add()),
            Command::DeleteAll => Ok(self.delete_all()),
            Command::Print => Ok(self.print()),
        };
    }
}

#[cfg(test)]
mod test {
    use crate::cli::FileInfo;
    use crate::processor::Hrtor;
    use crate::processor::HrtorProcessor;
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_handle_command() {
        let hrtor_processor: HrtorProcessor = HrtorProcessor {
            editing_file: Arc::new(Mutex::new(FileInfo {
                path: PathBuf::from("test"),
                context: "test".to_string(),
            })),
        };
        let _hrtor = Hrtor::new(hrtor_processor);
    }
}
