use std::sync::{Arc, Mutex};

use constants::{CommandResult, CommandStatus};
use file_loader::FileInfo;
use linefeed::{ReadResult, Signal};
use user_script::{lua::LuaScript, UserScript};

pub mod user_script;

pub struct Hrtor {
    pub processor: Arc<HrtorProcessor>,
    user_scripts: Vec<Box<dyn UserScript>>,
}

impl Hrtor {
    pub fn new(processor: HrtorProcessor) -> Self {
        Self {
            processor: processor.into(),
            user_scripts: vec![],
        }
    }
}

impl Hrtor {
    pub fn load_luascript(&mut self, entrypoint: FileInfo) {
        self.user_scripts.push(Box::new(LuaScript::new(
            Arc::clone(&self.processor),
            entrypoint,
        )));
    }
    pub fn init(&self) {
        for script in &self.user_scripts {
            script.init();
        }
    }
}

pub struct HrtorProcessor {
    pub editing_file: Arc<Mutex<FileInfo>>,
}

impl HrtorProcessor {
    /// Interpret CommandStatus without Input loop
    pub(crate) fn interpret_command_status(&self, status: CommandStatus) {
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
}

impl HrtorProcessor {
    pub fn handle_command(&self, hrtor: &Hrtor, command: ReadResult) -> CommandStatus {
        match command {
            ReadResult::Input(str) => {
                for script in &hrtor.user_scripts {
                    let Some(result) = script.request_handle(&str) else {
                        continue;
                    };
                    return result;
                }

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
}
