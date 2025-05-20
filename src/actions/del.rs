mod parser;

use crate::ProcessorImplementation;
use hrtor_core::constants::CommandStatus;

pub trait HrtorDel {
    fn del(&mut self, _arguments: &str) -> anyhow::Result<CommandStatus>;
}

impl HrtorDel for ProcessorImplementation {
    fn del(&mut self, arguments: &str) -> anyhow::Result<CommandStatus> {
        let args = parser::parse(arguments)?;
        dbg!(args);

        todo!()
    }
}
