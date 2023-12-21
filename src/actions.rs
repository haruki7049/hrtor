use crate::{CommandResult, CommandStatus, HrtorProcessor};

pub mod add;
pub mod write;

fn command_status_ok() -> CommandStatus {
    CommandStatus::Continue(CommandResult::Ok)
}

impl HrtorProcessor {
    pub(crate) fn quit(&self) -> CommandStatus {
        CommandStatus::Quit
    }
    pub(crate) fn delete_all(&self) -> CommandStatus {
        {
            self.editing_file.borrow_mut().context = String::new();
        }
        command_status_ok()
    }
    pub(crate) fn print(&self) -> CommandStatus {
        {
            println!("{}", self.editing_file.borrow().context);
        }
        command_status_ok()
    }
}
