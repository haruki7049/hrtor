use crate::processor::HrtorProcessor;
use crate::processor::constants::CommandStatus;

impl HrtorProcessor {
    pub fn grep(&self, arguments: &str) -> anyhow::Result<CommandStatus> {
        let mut result: String = String::new();
        let setp_str: Vec<&str> = self.editing_file.context.split('\n').collect(); // Separated String at '\n'

        for s in setp_str {
            if s.contains(arguments) {
                result.push_str(s);
                result.push('\n');
            }
        }

        println!("{}", result);

        Ok(CommandStatus::Continue)
    }
}
