use crate::{Hrtor, CommandStatus, CommandResult};

pub mod add;
pub mod write;

pub(self) fn result_ok() -> CommandStatus {
    CommandStatus::Continue(CommandResult::Ok)
}

impl Hrtor {
    pub(crate) fn quit(&self) -> CommandStatus {
        CommandStatus::Quit
    }
    pub(crate) fn delete_all(&mut self) -> CommandStatus {
        self.editing_file.context = String::new();
        result_ok()
    }
    pub(crate) fn print(&self) -> CommandStatus {
        println!("{}", self.editing_file.context);
        result_ok()
    }
}