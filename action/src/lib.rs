use crate::{CommandResult, CommandStatus, HrtorProcessor};

pub mod add;
pub mod write;

pub(crate) fn command_status_ok() -> CommandStatus {
    CommandStatus::Continue(CommandResult::Ok)
}

impl HrtorProcessor {
    pub(crate) fn quit(&self) -> CommandStatus {
        CommandStatus::Quit
    }
    pub(crate) fn delete_all(&self) -> CommandStatus {
        {
            let mut editing_file = self.editing_file.lock().unwrap();
            editing_file.context.clear();
        }
        command_status_ok()
    }
    pub(crate) fn print(&self) -> CommandStatus {
        let context = { &self.editing_file.lock().unwrap().context };
        println!("{}", context);
        command_status_ok()
    }
}
