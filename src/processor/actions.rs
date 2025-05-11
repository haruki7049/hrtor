pub mod add;
pub mod exit;
pub mod delete_all;
pub mod write;

use super::constants::CommandStatus;
use crate::processor::{HrtorProcessor, command_status_ok};

impl HrtorProcessor {
    pub fn print(&self) -> CommandStatus {
        let context = { &self.editing_file.lock().unwrap().context };
        println!("{}", context);
        command_status_ok()
    }
}
