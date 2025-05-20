use crate::ProcessorImplementation;
use hrtor_core::constants::CommandStatus;

pub trait HrtorGrep {
    fn grep(&self, arguments: &str) -> anyhow::Result<CommandStatus>;
}

impl HrtorGrep for ProcessorImplementation {
    fn grep(&self, arguments: &str) -> anyhow::Result<CommandStatus> {
        let mut result: String = String::new();
        let setp_str: Vec<&str> = self.buffer.context.split('\n').collect(); // Separated String at '\n'

        // Line Counter
        let mut counter: usize = 0;
        for string in setp_str {
            counter += 1;
            let s: String = format!("{}: {}\n", counter, string);
            if string.contains(arguments) {
                result.push_str(&s);
            }
        }

        println!("{}", result);

        Ok(CommandStatus::Continue)
    }
}
