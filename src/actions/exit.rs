//! # Exit action
//! If you want to exit Hrtor by your action, then you can set the return value to `CommandStatus::Quit`.

use crate::ProcessorImplementation;
use hrtor_core::constants::CommandStatus;

pub trait HrtorExit {
    fn exit(&self, _arguments: &str) -> CommandStatus;
}

impl HrtorExit for ProcessorImplementation {
    fn exit(&self, _arguments: &str) -> CommandStatus {
        CommandStatus::Quit
    }
}
