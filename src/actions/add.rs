use super::command_status_ok;
use crate::{CommandStatus, HrtorProcessor};
use std::io::StdinLock;

impl HrtorProcessor {
    pub(crate) fn add(&self) -> CommandStatus {
        let reader: StdinLock = std::io::stdin().lock();
        let writer: std::io::Stdout = std::io::stdout();

        let new_context = push_context(reader, writer);
        {
            self.editing_file.lock().unwrap().context = new_context;
        }
        command_status_ok()
    }
}

/// get some context from standard input, and return String
fn push_context<R, W>(mut reader: R, _writer: W) -> String
where
    R: std::io::BufRead,
    W: std::io::Write,
{
    let mut inputed_text: String = String::new();
    loop {
        let mut last_line: String = String::new();

        reader
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_context() {
        let mut writer: Vec<u8> = Vec::new();
        let reader = std::io::Cursor::new(b"test\ntest\ntest\n.\n");
        let result = push_context(reader, writer);
        assert_eq!(result, "test\ntest\ntest\n");
    }
}
