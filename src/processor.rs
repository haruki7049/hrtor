//! # Hrtor processor module

mod actions;
pub mod constants;
mod parser;

use crate::processor::parser::{Action, Expression};
use constants::CommandStatus;
use linefeed::{ReadResult, Signal};
use std::sync::{Arc, Mutex};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub struct FileInfo {
    pub path: PathBuf,
    pub context: String,
}

pub struct HrtorProcessor {
    pub editing_file: Arc<Mutex<FileInfo>>,
}

impl HrtorProcessor {
    /// Creates Hrtorprocessor from a FileInfo which user want to edit
    fn from(file: FileInfo) -> Self {
        Self {
            editing_file: Arc::new(Mutex::new(file)),
        }
    }
}

pub trait Processor {
    /// Handle the strings from linefeed's inputs
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
    /// Handle the strings from linefeed's inputs
    fn handle_command(&self, command: ReadResult) -> anyhow::Result<CommandStatus> {
        match command {
            ReadResult::Input(str) => self.eval(str),
            ReadResult::Eof
            | ReadResult::Signal(Signal::Interrupt)
            | ReadResult::Signal(Signal::Quit) => Ok(CommandStatus::Quit),
            _ => Ok(CommandStatus::Continue),
        }
    }

    /// Eval function
    /// This function receives a String, then the internal logic converts it to a Expression.
    ///
    /// 1. Receives a String
    /// 2. Converts it to a Expression
    /// 3. Forks process by the Expression, then returns CommandStatus
    fn eval(&self, str: String) -> anyhow::Result<CommandStatus> {
        let expr: Expression = parser::parse(str.as_str())?;

        match expr.action {
            Action::Exit => Ok(self.exit(expr.arguments)),
            Action::Write => Ok(self.write(expr.arguments)),
            Action::Add => Ok(self.add(expr.arguments)),
            Action::DeleteAll => Ok(self.delete_all(expr.arguments)),
            Action::Print => Ok(self.print(expr.arguments)),
            Action::Tutorial => Ok(self.tutorial(expr.arguments)),
        }
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
                context: String::from("test"),
            })),
        };
        let _hrtor = Hrtor::new(hrtor_processor);
    }
}
