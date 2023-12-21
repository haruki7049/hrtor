use super::command_status_ok;
use crate::{CommandStatus, Hrtor};

impl Hrtor<'_> {
    pub(crate) fn add(&mut self) -> CommandStatus {
        self.editing_file.context = push_context();
        command_status_ok()
    }
}

/// get some context from standard input, and return String
fn push_context() -> String {
    let mut inputed_text: String = String::new();
    loop {
        let mut last_line: String = String::new();

        std::io::stdin()
            .read_line(&mut last_line)
            .expect("failed to read line");

        #[cfg(unix)]
        if last_line.as_str() == ".\n" {
            break;
        }
        #[cfg(windows)]
        if last_line.as_str() == ".\r\n" {
            break;
        }

        inputed_text.push_str(&last_line);
    }
    inputed_text
}
