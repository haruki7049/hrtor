use crate::processor::HrtorProcessor;
use crate::processor::constants::CommandStatus;

impl HrtorProcessor {
    pub fn print(&self, _arguments: &str) -> CommandStatus {
        println!("{}", &self.editing_file.context);

        CommandStatus::Continue
    }
}
