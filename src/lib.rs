pub mod actions;
pub mod cli;

use crate::actions::add::HrtorAdd;
use hrtor_core::constants::CommandStatus;
use hrtor_core::parser::{self, Expression};
use hrtor_core::{FileInfo, Processor, ReadResult, Signal};

pub struct ProcessorImplementation {
    pub editing_file: FileInfo,
}

impl Processor for ProcessorImplementation {
    /// Handle the strings from inputs by main.rs on Hrtor implementation
    fn handle_command(&mut self, command: ReadResult) -> anyhow::Result<CommandStatus> {
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
    fn eval(&mut self, str: String) -> anyhow::Result<CommandStatus> {
        let expr: Expression = parser::parse(str.as_str())?;

        match expr.action {
            "add" => Ok(self.add(expr.arguments)),
            s => anyhow::bail!("ACTION_LOADING_ERROR: Unknown action's name, {}", s),
        }
    }
}
