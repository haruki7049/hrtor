//! # Exit action
//! If you want to exit Hrtor by your action, then you can set the return value to `CommandStatus::Quit`.

use hrtor_core::constants::CommandStatus;
use crate::ProcessorImplementation;

pub trait HrtorExit {
    fn exit(&self, _arguments: &str) -> CommandStatus;
}

impl HrtorExit for ProcessorImplementation {
    fn exit(&self, _arguments: &str) -> CommandStatus {
        CommandStatus::Quit
    }
}
