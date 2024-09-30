use std::sync::{Arc, Mutex};
use std::io::StdinLock;
use constants::{CommandResult, CommandStatus};
use file_loader::FileInfo;
use linefeed::ReadResult;

pub struct HrtorProcessor {
    pub editing_file: Arc<Mutex<FileInfo>>,
}

pub(crate) fn command_status_ok() -> CommandStatus {
    CommandStatus::Continue(CommandResult::Ok)
}

pub trait Processor {
    /// Interpret CommandStatus without Input loop
    fn interpret_command_status(&self, status: CommandStatus);

    fn handle_command(&self, command: ReadResult) -> CommandStatus;
}

impl HrtorProcessor {
    pub(crate) fn quit(&self) -> CommandStatus {
        CommandStatus::Quit
    }
    pub(crate) fn delete_all(&self) -> CommandStatus {
        {
            let mut editing_file = self.editing_file.lock().unwrap();
            editing_file.context.clear();
        }
        command_status_ok()
    }
    pub(crate) fn print(&self) -> CommandStatus {
        let context = { &self.editing_file.lock().unwrap().context };
        println!("{}", context);
        command_status_ok()
    }
}

impl HrtorProcessor {
    pub(crate) fn add(&self) -> CommandStatus {
        let reader: StdinLock = std::io::stdin().lock();
        let _writer: std::io::Stdout = std::io::stdout();

        let new_context = push_context(reader, _writer);
        {
            self.editing_file.lock().unwrap().context = new_context;
        }
        command_status_ok()
    }
}

impl HrtorProcessor {
    pub(crate) fn write(&self) -> CommandStatus {
        {
            let editing_file = self.editing_file.lock().unwrap();
            save_file(&editing_file.path, &editing_file.context);
        }
        command_status_ok()
    }
}

/// save file
fn save_file(filepath: &String, file_context: &String) {
    if let Err(err) = std::fs::write(filepath, file_context) {
        eprintln!("Error saving file: {}", err);
    } else {
        println!("file saved successfully");
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
        let writer: Vec<u8> = Vec::new();
        let reader = std::io::Cursor::new(b"test\ntest\ntest\n.\n");
        let result = push_context(reader, writer);
        assert_eq!(result, "test\ntest\ntest\n");
    }
}
