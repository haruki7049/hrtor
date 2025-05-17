use crate::ProcessorImplementation;
use hrtor_core::constants::CommandStatus;

pub trait HrtorDeleteAll {
    fn delete_all(&mut self, _arguments: &str) -> CommandStatus;
}

impl HrtorDeleteAll for ProcessorImplementation {
    fn delete_all(&mut self, _arguments: &str) -> CommandStatus {
        self.editing_file.context.clear();

        CommandStatus::Continue
    }
}
