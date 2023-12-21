use std::{cell::RefCell, rc::Rc};

use file_loader::FileInfo;
use linefeed::ReadResult;
use user_script::{lua::LuaScript, UserScript};

mod actions;
pub mod constants;
pub mod file_loader;
pub mod user_script;

pub struct Hrtor<'a> {
    pub processor: &'a HrtorProcessor,
    user_scripts: Vec<Box<dyn UserScript + 'a>>,
}

impl<'a> Hrtor<'a> {
    pub fn new(processor: &'a HrtorProcessor) -> Self {
        Self {
            processor,
            user_scripts: vec![],
        }
    }
}

impl<'a> Hrtor<'a> {
    fn get_hrtor_processor(&self) -> &'a HrtorProcessor {
        &self.processor
    }
    pub fn load_luascript(&mut self, entrypoint: FileInfo) {
        self.user_scripts.push(Box::new(LuaScript {
            hrtor: self.get_hrtor_processor(),
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
    pub editing_file: Rc<RefCell<FileInfo>>,
}

pub enum CommandStatus {
    Continue(CommandResult),
    Quit,
}
pub enum CommandResult {
    Ok,
    NotFound(String),
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
            _ => {
                eprintln!("Unexpected Result!");
                CommandStatus::Quit
            }
        }
    }
}
