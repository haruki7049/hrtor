use std::sync::{Arc, Mutex};

use file_loader::FileInfo;
use linefeed::{ReadResult, Signal};
use user_script::{lua::LuaScript, UserScript};

mod actions;
pub mod constants;
pub mod file_loader;
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
        self.user_scripts.push(Box::new(LuaScript {
            hrtor: Arc::clone(&self.processor),
            entrypoint,
        }));
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

pub enum CommandStatus {
    Continue(CommandResult),
    Quit,
}
pub enum CommandResult {
    Ok,
    NotFound(String),
    NothingToDo,
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
    pub fn handle_command(&self, command: ReadResult) -> CommandStatus {
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
