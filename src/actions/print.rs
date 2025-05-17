use crate::processor::HrtorProcessor;
use crate::processor::constants::CommandStatus;

impl HrtorProcessor {
    pub fn print(&self, _arguments: &str) -> CommandStatus {
        let mut result: String = String::new();
        let setp_str: Vec<&str> = self.editing_file.context.split('\n').collect(); // Separated String at '\n'

        // Line Counter
        let mut counter: usize = 0;
        for string in setp_str {
            counter += 1;
            let s: String = format!("{}: {}\n", counter, string);
            result.push_str(&s);
        }

        println!("{}", result);

        CommandStatus::Continue
    }
}
