use std::sync::{Arc, Mutex};
use std::io::StdinLock;
use constants::{CommandResult, CommandStatus};
use file_loader::FileInfo;
use linefeed::{ ReadResult, Signal };

pub struct HrtorProcessor {
    pub editing_file: Arc<Mutex<FileInfo>>,
}

pub fn command_status_ok() -> CommandStatus {
    CommandStatus::Continue(CommandResult::Ok)
}

pub trait Processor {
    /// Interpret CommandStatus without Input loop
    fn interpret_command_status(&self, status: CommandStatus);

    fn handle_command(&self, command: ReadResult) -> CommandStatus;
}

impl HrtorProcessor {
    pub fn quit(&self) -> CommandStatus {
        CommandStatus::Quit
    }
    pub fn delete_all(&self) -> CommandStatus {
        {
            let mut editing_file = self.editing_file.lock().unwrap();
            editing_file.context.clear();
        }
        command_status_ok()
    }
    pub fn print(&self) -> CommandStatus {
        let context = { &self.editing_file.lock().unwrap().context };
        println!("{}", context);
        command_status_ok()
    }
}

impl HrtorProcessor {
    pub fn add(&self) -> CommandStatus {
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
    pub fn write(&self) -> CommandStatus {
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

pub struct Hrtor {
    pub processor: Arc<HrtorProcessor>,
}

impl Hrtor {
    pub fn new(processor: HrtorProcessor) -> Self {
        Self {
            processor: processor.into(),
        }
    }
}

impl Processor for HrtorProcessor {
    fn interpret_command_status(&self, status: CommandStatus) {
        match status {
            CommandStatus::Continue(CommandResult::Ok) => (),
            CommandStatus::Continue(CommandResult::NothingToDo) => (),
            CommandStatus::Continue(CommandResult::NotFound(name)) => {
                panic!("unknown command: {:?}", name);
            }
            CommandStatus::Quit => {
                // Exit status zero
                println!("Bye!!");
                std::process::exit(0);
            }
        }
    }

    fn handle_command(&self, command: ReadResult) -> CommandStatus {
        match command {
            ReadResult::Input(str) => {
                if str == "exit" {
                    return self.quit();
                }
                if str == "write" {
                    return self.write();
                }
                if str == "add" {
                    return self.add();
                }
                if str == "delete_all" {
                    return self.delete_all();
                }
                if str == "print" {
                    return self.print();
                }
                CommandStatus::Continue(CommandResult::NotFound(str))
            }
            ReadResult::Eof
            | ReadResult::Signal(Signal::Interrupt)
            | ReadResult::Signal(Signal::Quit) => CommandStatus::Quit,
            _ => CommandStatus::Continue(CommandResult::NothingToDo),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Hrtor;
    use crate::push_context;
    use crate::HrtorProcessor;
    use file_loader::FileInfo;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_handle_command() {
        let hrtor_processor: HrtorProcessor = HrtorProcessor {
            editing_file: Arc::new(Mutex::new(FileInfo {
                path: "test".to_string(),
                context: "test".to_string(),
            })),
        };
        let _hrtor = Hrtor::new(hrtor_processor);
    }

    #[test]
    fn test_push_context() {
        let writer: Vec<u8> = Vec::new();
        let reader = std::io::Cursor::new(b"test\ntest\ntest\n.\n");
        let result = push_context(reader, writer);
        assert_eq!(result, "test\ntest\ntest\n");
    }
}
