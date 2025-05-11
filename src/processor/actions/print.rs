use crate::processor::HrtorProcessor;
use crate::processor::constants::CommandStatus;

impl HrtorProcessor {
    pub fn print(&self, _arguments: &str) -> CommandStatus {
        let context = { &self.editing_file.lock().unwrap().context };
        println!("{}", context);

        CommandStatus::Continue
    }
}
