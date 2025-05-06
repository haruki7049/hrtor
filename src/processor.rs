//! # Hrtor processor module
//!
//! This crate has some structs
//! - Hrtor
//! - HrtorProcessor
//!
//! And some traits
//! - Processor
//!
//! ---
//! Structures
//!
//! ## Hrtor
//! ### Variables
//! - processor: `Arc<HrtorProcessor>`
//!
//! ### Interface
//! - new: `Fn(processor: HrtorProcessor) -> Self`
//!
//! ## HrtorProcessor
//! ### variables
//! - editing_file: `Arc<Mutex<FileInfo>>`
//!
//! ### Interface
//! - handle_command: `Fn(&self, command: ReadResult) -> CommandStatus`
//! - interpret_command_status: `Fn(&self, status: CommandStatus)`
//!
//! ---
//! Traits
//!
//! ## Processor
//! - interpret_command_status: `Fn(&self, status: CommandStatus)`

pub mod actions;
pub mod constants;

use crate::cli::FileInfo;
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

    fn handle_command(&self, command: ReadResult) -> CommandStatus;
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
            CommandStatus::Continue(CommandResult::NotFound(name)) => {
                panic!("unknown command: {:?}", name);
            }
            CommandStatus::Quit => {
                // Exit status zero
                println!("Bye!!");
                std::process::exit(0);
            }
        }
    }

    fn handle_command(&self, command: ReadResult) -> CommandStatus {
        match command {
            ReadResult::Input(str) => {
                if str == "exit" {
                    return self.quit();
                }
                if str == "write" {
                    return self.write();
                }
                if str == "add" {
                    return self.add();
                }
                if str == "delete_all" {
                    return self.delete_all();
                }
                if str == "print" {
                    return self.print();
                }
                CommandStatus::Continue(CommandResult::NotFound(str))
            }
            ReadResult::Eof
            | ReadResult::Signal(Signal::Interrupt)
            | ReadResult::Signal(Signal::Quit) => CommandStatus::Quit,
            _ => CommandStatus::Continue(CommandResult::NothingToDo),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cli::FileInfo;
    use crate::processor::Hrtor;
    use crate::processor::HrtorProcessor;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_handle_command() {
        let hrtor_processor: HrtorProcessor = HrtorProcessor {
            editing_file: Arc::new(Mutex::new(FileInfo {
                path: "test".to_string(),
                context: "test".to_string(),
            })),
        };
        let _hrtor = Hrtor::new(hrtor_processor);
    }
}
