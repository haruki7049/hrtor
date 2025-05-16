use crate::processor::HrtorProcessor;
use crate::processor::constants::CommandStatus;

impl HrtorProcessor {
    pub fn grep(&self, arguments: &str) -> anyhow::Result<CommandStatus> {
        let result = hrtor_grep::run(self.editing_file.context.clone(), arguments)?;
        println!("{}", result);

        Ok(CommandStatus::Continue)
    }
}
