use crate::ProcessorImplementation;
use hrtor_core::constants::CommandStatus;

pub trait HrtorPrint {
    fn print(&mut self, arguments: &str) -> CommandStatus;
}

impl HrtorPrint for ProcessorImplementation {
    fn print(&mut self, _arguments: &str) -> CommandStatus {
        let mut result: String = String::new();
        let setp_str: Vec<&str> = self.buffer.context.split('\n').collect(); // Separated String at '\n'

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
