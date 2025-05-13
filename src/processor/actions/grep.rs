use crate::processor::HrtorProcessor;
use crate::processor::constants::CommandStatus;

impl HrtorProcessor {
    pub fn grep(&self, arguments: &str) -> anyhow::Result<CommandStatus> {
        let context = self.editing_file.lock().unwrap().context.clone();
        let result = hrtor_grep::run(context, arguments)?;
        println!("{}", result);

        Ok(CommandStatus::Continue)
    }
}
