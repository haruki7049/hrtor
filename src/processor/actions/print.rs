use crate::processor::constants::CommandStatus;
use crate::processor::{HrtorProcessor, command_status_ok};

impl HrtorProcessor {
    pub fn print(&self, _arguments: &str) -> CommandStatus {
        let context = { &self.editing_file.lock().unwrap().context };
        println!("{}", context);
        command_status_ok()
    }
}
