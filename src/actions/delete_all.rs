use hrtor_core::constants::CommandStatus;
use crate::ProcessorImplementation;

pub trait HrtorDeleteAll {
    fn delete_all(&mut self, _arguments: &str) -> CommandStatus;
}

impl HrtorDeleteAll for ProcessorImplementation {
    fn delete_all(&mut self, _arguments: &str) -> CommandStatus {
        self.editing_file.context.clear();

        CommandStatus::Continue
    }
}
